use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::{BinmanError, BinmanResult, Cause};
use crate::github::{Asset, Client, Repository};

use super::fuzzy_semver::parse_version_fuzzy;
use super::{Config, State, StateEntry};
use reqwest;
use rood::cli::OutputManager;
use rood::sys::file;
use tokio::runtime::Runtime;

async fn save_asset(
    asset: &Asset,
    install_location: &str,
    output: &OutputManager,
) -> BinmanResult<String> {
    let asset_dest_path = String::from(
        Path::new(install_location)
            .join(asset.name())
            .to_str()
            .unwrap(),
    );
    output.progress(
        &format!(
            "{}-{}-{} => {}",
            asset.name(),
            asset.platform(),
            asset.architecture(),
            asset_dest_path
        ),
        1,
    );

    // Download the file
    let resp = reqwest::get(&asset.browser_download_url).await?;
    let bytes_buffer = resp.bytes().await?;
    let body: &[u8] = bytes_buffer.as_ref();
    let mut dest = File::create(&asset_dest_path)?;
    dest.write_all(body)?;

    // Mark executable.
    file::make_executable(Path::new(&asset_dest_path))?;
    Ok(asset_dest_path)
}

pub async fn async_install(
    repo_url: &str,
    version: &str,
    install_location: &str,
    output: &OutputManager,
) -> BinmanResult<StateEntry> {
    let client = Client::new()?;
    let repo = client.get_repository(repo_url)?;
    output.step(&format!("Installing [{}]", &repo.name), 0);

    let maybe_release;
    if version == "latest" {
        maybe_release = Some(client.latest_release(&repo).await?);
    } else {
        let releases = client.get_releases(&repo).await?;
        let semv = parse_version_fuzzy(version)?;
        maybe_release = releases
            .iter()
            .find(|release| release.version().is_ok() && release.version().unwrap() == semv)
            .cloned();
    }

    if let Some(release) = maybe_release {
        let assets = release.platform_assets();
        let mut asset_paths = Vec::new();
        for asset in assets.iter() {
            if asset.browser_download_url.contains("md5")
                || asset.browser_download_url.contains("sha256")
            {
                // Skip hash checks.
                // TODO: Fix file extensions & support hash integrity checks.
                continue;
            }
            asset_paths.push(save_asset(asset, install_location, output).await?);
        }

        Ok(StateEntry {
            name: repo.name.clone(),
            url: String::from(repo_url),
            version: release.version()?,
            artifacts: asset_paths,
        })
    } else {
        return Err(BinmanError::new(
            Cause::NotFound,
            &format!("Version {} not found", version),
        ));
    }
}

pub fn install_target(repo_url: &str, version: &str, output: &OutputManager) -> BinmanResult<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    let mut used_url = String::from(repo_url);

    if !used_url.contains("/") {
        output.debug("URL not recognized - using default code host", 1);
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
