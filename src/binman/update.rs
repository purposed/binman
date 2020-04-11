use rood::cli::OutputManager;
use tokio::runtime::Runtime;

use super::{Config, State};
use crate::binman::install::async_install;
use crate::binman::{uninstall_target, StateEntry};
use crate::error::{BinmanError, BinmanResult, Cause};
use crate::github::Client;

async fn async_update(
    entry: &StateEntry,
    install_location: &str,
    output: &OutputManager,
) -> BinmanResult<Option<StateEntry>> {
    let client = Client::new()?;
    let repo = client.get_repository(&entry.url)?;
    let latest = client.latest_release(&repo).await?;

    let latest_v = latest.version()?;
    if latest_v > entry.version {
        output.progress(&format!("Upgrade available: {}@{}", entry.name, latest_v));
        uninstall_target(&entry.name, output)?;
        Ok(Some(
            async_install(&entry.url, &latest_v.to_string(), install_location, output).await?,
        ))
    } else {
        output.success("Nothing to do!");
        Ok(None)
    }
}

pub fn update_target(target: &str, output: &OutputManager) -> BinmanResult<()> {
    let cfg = Config::new()?;

    // Get existing entry.
    let entry;
    {
        // Get read scope on state.
        let state = State::new(&cfg.state_file_path)?;
        entry = state.get_copy(target).ok_or_else(|| {
            BinmanError::new(
                Cause::NotFound,
                &format!("Binary {} is not installed", target),
            )
        })?;
    }

    let possible_new_entry =
        Runtime::new()?.block_on(async_update(&entry, &cfg.install_location, output))?;
    {
        // Get write scope on state.
        let mut state = State::new(&cfg.state_file_path)?;
        if let Some(new_entry) = possible_new_entry {
            state.insert(new_entry)?;
        }
    }

    Ok(())
}
