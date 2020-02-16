use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use reqwest;
use rood::cli::OutputManager;
use rood::sys::file;
use sha2::{Digest, Sha256};
use tempfile::tempdir;
use tokio::runtime::Runtime;

use super::fuzzy_semver::parse_version_fuzzy;
use super::{Config, State, StateEntry};
use crate::error::{BinmanError, BinmanResult, Cause};
use crate::github::{Asset, Client, Repository};
use rood::sys::file::ensure_exists;
use std::ffi::OsStr;

async fn save_asset(asset: &Asset, install_location: &Path) -> BinmanResult<String> {
    let mut asset_dest_path = install_location.join(&format!(
        "{}-{}-{}",
        asset.name(),
        asset.platform(),
        asset.architecture()
    ));
    asset_dest_path.set_extension(asset.extension());

    // Download the file
    let resp = reqwest::get(&asset.browser_download_url).await?;
    let bytes_buffer = resp.bytes().await?;
    let body: &[u8] = bytes_buffer.as_ref();
    let mut dest = File::create(&asset_dest_path)?;
    dest.write_all(body)?;

    // Mark executable.
    file::make_executable(&asset_dest_path)?;
    Ok(String::from(asset_dest_path.to_str().unwrap()))
}

fn do_checksum(src_dir: &Path, checksum_file_path: &Path) -> BinmanResult<()> {
    // TODO: Extract to rood.

    // Read checksum file.
    let checksum_raw = fs::read_to_string(checksum_file_path)?;

    let expected_hash = checksum_raw
        .split_ascii_whitespace()
        .next()
        .ok_or_else(|| BinmanError::new(Cause::InvalidState, "Invalid SHA256 format"))?;

    let checksum_file_name = checksum_raw
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| BinmanError::new(Cause::InvalidState, "Invalid SHA256 format"))?;

    let checksum_target_path = src_dir.join(checksum_file_name);
    ensure_exists(&checksum_target_path)
        .map_err(|_e| BinmanError::new(Cause::NotFound, "Checksum target not found"))?;

    let mut checksum = Sha256::new();
    let artifact_data = fs::read(checksum_target_path)?;
    checksum.input(artifact_data);
    let checksum_value = checksum.result();
    let nicely_formatted_hash = format!("{:x}", checksum_value);

    if nicely_formatted_hash != expected_hash {
        return Err(BinmanError::new(
            Cause::InvalidState,
            &format!("Checksum verification failed for {}", checksum_file_name),
        ));
    }

    // Delete checksum file
    fs::remove_file(checksum_file_path)?;

    Ok(())
}

fn move_assets(src_dir: &Path, dst_dir: &Path, output: OutputManager) -> BinmanResult<Vec<String>> {
    let mut final_assets = Vec::new();
    let dir_entries = fs::read_dir(src_dir)?;
    for possible_entry in dir_entries {
        let entry = possible_entry?;
        if let Some(ext) = entry.path().extension() {
            match ext.to_str().unwrap() {
                "sha256" => {
                    output.progress(&format!(
                        "Validating checksum: {}",
                        entry.file_name().to_str().unwrap()
                    ));
                    do_checksum(src_dir, &entry.path())?;
                    continue;
                }
                "md5" => continue,
                _ => {}
            }
        }

        let raw_fn = entry.file_name();
        let current_file_name = raw_fn.to_str().unwrap();
        let mut final_file_name = PathBuf::from(current_file_name.split('-').next().unwrap());
        final_file_name.set_extension(entry.path().extension().unwrap_or(&OsStr::new("")));
        let dst_entry = dst_dir.join(&final_file_name);

        output.progress(&format!("Asset: {}", dst_entry.to_str().unwrap()));
        fs::rename(entry.path(), &dst_entry)?;
        final_assets.push(String::from(dst_entry.to_str().unwrap()));
    }

    Ok(final_assets)
}

pub async fn async_install(
    repo_url: &str,
    version: &str,
    install_location: &str,
    output: &OutputManager,
) -> BinmanResult<StateEntry> {
    let temp_dir = tempdir()?;

    let client = Client::new()?;
    let repo = client.get_repository(repo_url)?;
    output.step(&format!("Installing [{}]", &repo.name));

    let maybe_release = if version == "latest" {
        Some(client.latest_release(&repo).await?)
    } else {
        let releases = client.get_releases(&repo).await?;
        let semv = parse_version_fuzzy(version)?;
        releases
            .iter()
            .find(|release| release.version().is_ok() && release.version().unwrap() == semv)
            .cloned()
    };

    if let Some(release) = maybe_release {
        let assets = release.platform_assets();
        for asset in assets.iter() {
            save_asset(asset, temp_dir.path()).await?;
        }

        let asset_paths = move_assets(temp_dir.path(), Path::new(install_location), output.push())?;
        output.success("OK");
        Ok(StateEntry {
            name: repo.name.clone(),
            url: String::from(repo_url),
            version: release.version()?,
            artifacts: asset_paths,
        })
    } else {
        Err(BinmanError::new(
            Cause::NotFound,
            &format!("Version {} not found", version),
        ))
    }
}

pub fn install_target(repo_url: &str, version: &str, output: &OutputManager) -> BinmanResult<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    let mut used_url = String::from(repo_url);

    if !used_url.contains('/') {
        output.debug("URL not recognized - using default code host");
        used_url = vec![cfg.default_code_host.clone(), String::from(repo_url)].join("/");
    }

    let app_name = &Repository::from_url(&used_url)?.name;

    match state.get(app_name) {
        Some(t) => Err(BinmanError::new(
            Cause::AlreadyExists,
            &format!("Target [{}] is already installed", t.name),
        )),
        None => {
            // TODO: Reuse runtime for multiple targets somehow.
            let new_entry = Runtime::new()?.block_on(async_install(
                &used_url,
                version,
                &cfg.install_location,
                output,
            ))?;

            // Insert installation in state.
            state.insert(new_entry)?;

            Ok(())
        }
    }
}
