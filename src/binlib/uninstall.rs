use std::fs;

use anyhow::{anyhow, Result};

use super::{Config, State};

pub fn uninstall_target(target_name: &str) -> Result<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    let entry = state
        .get_copy(target_name)
        .ok_or_else(|| anyhow!("Target [{}] is not installed", target_name))?;

    tracing::info!(target=%target_name, "starting package removal");

    for artifact in entry.artifacts.iter() {
        fs::remove_file(artifact)?;
        tracing::info!(asset=%artifact, "removed asset");
    }
    // Commit uninstall to state.
    state.remove(&entry.name)?;
    tracing::debug!(target=%entry.name, "removed state entry");
    Ok(())
}
