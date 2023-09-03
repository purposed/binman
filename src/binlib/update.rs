use anyhow::{anyhow, Result};

use crate::github::Client;
use crate::{install::async_install, uninstall_target, Config, State, StateEntry};

async fn async_update(
    entry: &StateEntry,
    install_location: &str,
    force: bool,
) -> Result<Option<StateEntry>> {
    let client = Client::new()?;
    let repo = client.get_repository(&entry.url)?;
    let latest = client.latest_release(&repo).await?;

    let latest_v = latest.version();
    if latest_v > entry.version || force {
        tracing::info!(target=%entry.name, version=%latest_v, "upgrade available");
        uninstall_target(&entry.name)?;
        Ok(Some(
            async_install(&entry.url, &latest_v.to_string(), install_location).await?,
        ))
    } else {
        tracing::info!("nothing to do");
        Ok(None)
    }
}

#[tracing::instrument]
pub async fn update_target(target: &str, force: bool) -> Result<()> {
    let cfg = Config::new()?;

    // Get existing entry.
    let entry;
    {
        // Get read scope on state.
        let state = State::new(&cfg.state_file_path)?;
        entry = state
            .get_copy(target)
            .ok_or_else(|| anyhow!("Binary [{}] is not installed", target))?;
    }

    let possible_new_entry = async_update(&entry, &cfg.install_location, force).await?;
    {
        // Get write scope on state.
        let mut state = State::new(&cfg.state_file_path)?;
        if let Some(new_entry) = possible_new_entry {
            state.insert(new_entry)?;
        }
    }

    Ok(())
}
