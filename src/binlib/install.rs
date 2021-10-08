use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};

use rood::cli::OutputManager;
use rood::sys::file::{self, ensure_exists};

use sha2::{Digest, Sha256};

use tempfile::tempdir;

use walkdir::WalkDir;

use super::fuzzy_semver::parse_version_fuzzy;
use super::zip;
use super::{Config, State, StateEntry};
use crate::github::{Asset, Client, Repository};

async fn save_asset(asset: &Asset, install_location: &Path, output: OutputManager) -> Result<()> {
    let mut asset_dest_path = install_location.join(&format!(
        "{}-{}-{}",
        asset.name(),
        asset.platform(),
        asset.architecture()
    ));
    let extension = asset.extension();
    asset_dest_path.set_extension(extension);

    output.debug(format!("asset download path: {:?}", &asset_dest_path));

    // Download the file
    let resp = reqwest::get(&asset.browser_download_url).await?;
    let bytes_buffer = resp.bytes().await?;
    let body: &[u8] = bytes_buffer.as_ref();
    let mut dest = File::create(&asset_dest_path)?;
    dest.write_all(body)?;

    // Extract, if required.
    if let Some(compression) = zip::get_compression(extension) {
        output.progress("Inflating compressed release");
        zip::extract(&asset_dest_path, install_location, compression)?;
        fs::remove_file(&asset_dest_path)?;
    } else {
        // If no compression, we should have an executable.
        file::make_executable(&asset_dest_path)?;
    }

    Ok(())
}

fn do_checksum(src_dir: &Path, checksum_file_path: &Path) -> Result<()> {
    // TODO: Extract to rood.

    // Read checksum file.
    let checksum_raw = fs::read_to_string(checksum_file_path)?;

    let expected_hash = checksum_raw
        .split_ascii_whitespace()
        .next()
        .context("Invalid SHA256 Format")?;

    let checksum_file_name = checksum_raw
        .split_ascii_whitespace()
        .last()
        .context("Invalid SHA256 Format")?;

    let checksum_target_path = src_dir.join(checksum_file_name);
    ensure_exists(&checksum_target_path).context("Checksum target not found")?;

    let mut checksum = Sha256::new();
    let artifact_data = fs::read(checksum_target_path)?;
    checksum.update(artifact_data);
    let checksum_value = checksum.finalize();
    let nicely_formatted_hash = format!("{:x}", checksum_value);

    ensure!(
        nicely_formatted_hash == expected_hash,
        "Checksum verification failed for {}",
        checksum_file_name
    );

    // Delete checksum file
    fs::remove_file(checksum_file_path)?;

    Ok(())
}

fn move_assets(src_dir: &Path, dst_dir: &Path, output: OutputManager) -> Result<Vec<String>> {
    let mut final_assets = Vec::new();
    let wk = WalkDir::new(src_dir);
    for entry in wk.into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        if let Some(ext) = entry.path().extension() {
            match ext.to_str().unwrap() {
                "sha256" => {
                    output.progress(&format!(
                        "Validating checksum: {}",
                        entry.file_name().to_str().unwrap()
                    ));
                    do_checksum(src_dir, entry.path())?;
                    continue;
                }
                "md5" => continue,
                _ => {}
            }
        }

        let raw_fn = entry.file_name();
        let current_file_name = raw_fn.to_str().unwrap();
        let mut final_file_name = PathBuf::from(current_file_name.split('-').next().unwrap());
        final_file_name.set_extension(entry.path().extension().unwrap_or_default());
        let dst_entry = dst_dir.join(&final_file_name);

        if file::is_executable(entry.path())? {
            output.progress(&format!("Produced asset: {}", dst_entry.to_str().unwrap()));
            fs::copy(entry.path(), &dst_entry)?;
            file::make_executable(&dst_entry)?;
            final_assets.push(String::from(dst_entry.to_str().unwrap()));
        }
    }

    Ok(final_assets)
}

pub async fn async_install(
    repo_url: &str,
    version: &str,
    install_location: &str,
    output: &OutputManager,
) -> Result<StateEntry> {
    // Ensure install directory exists.
    fs::create_dir_all(install_location)?;

    // Create temp dir for asset retrieval.
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

    ensure!(maybe_release.is_some(), "Version {} not found", version);

    let release = maybe_release.unwrap();

    let assets = release.platform_assets();

    ensure!(!assets.is_empty(), "No assets found for current platform");

    for asset in assets.iter() {
        if assets.len() == 1
            || output.prompt_yn(format!("Install asset {}", asset.full_name()), true)?
        {
            save_asset(asset, temp_dir.path(), output.push()).await?;
        }
    }

    let asset_paths = move_assets(temp_dir.path(), Path::new(install_location), output.push())?;
    output.success("OK");
    Ok(StateEntry {
        name: repo.name.clone(),
        url: String::from(repo_url),
        version: release.version()?,
        artifacts: asset_paths,
    })
}

pub async fn install_target(
    repo_url: &str,
    version: &str,
    output: &OutputManager,
    optional_dir_override: Option<&String>,
) -> Result<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    let mut used_url = String::from(repo_url);

    if !used_url.contains('/') {
        output.debug("URL not recognized - using default code host");
        used_url = vec![cfg.default_code_host.clone(), String::from(repo_url)].join("/");
    }

    let app_name = &Repository::from_url(&used_url)?.name;

    ensure!(
        state.get(app_name).is_none(),
        "Target [{}] is already installed",
        app_name
    );

    let install_dir = if let Some(overr) = optional_dir_override {
        overr
    } else {
        &cfg.install_location
    };

    let new_entry = async_install(&used_url, version, install_dir, output).await?;

    // Insert installation in state.
    state.insert(new_entry)?;

    Ok(())
}
