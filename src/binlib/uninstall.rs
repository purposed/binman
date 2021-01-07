use std::fs;

use anyhow::{anyhow, Result};

use rood::cli::OutputManager;

use super::{Config, State};

pub fn uninstall_target(target_name: &str, output: &OutputManager) -> Result<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    let entry = state
        .get_copy(target_name)
        .ok_or_else(|| anyhow!("Target [{}] is not installed", target_name))?;

    output.step(&format!("Uninstalling [{}]", target_name));

    let pushed = output.push();
    for artifact in entry.artifacts.iter() {
        pushed.debug(&format!("Removing {}", artifact));
        fs::remove_file(artifact)?;
    }
    // Commit uninstall to state.
    state.remove(&entry.name)?;
    Ok(())
}
