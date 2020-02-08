use std::fs;

use rood::cli::OutputManager;

use super::{Config, State};
use crate::error::{BinmanError, BinmanResult, Cause};

pub fn uninstall_target(target_name: &str, output: &OutputManager) -> BinmanResult<()> {
    let cfg = Config::new()?;
    let mut state = State::new(&cfg.state_file_path)?;

    match state.get_copy(target_name) {
        Some(entry) => {
            output.step(&format!("Uninstalling [{}]", target_name), 0);

            for artifact in entry.artifacts.iter() {
                output.debug(&format!("Removing {}", artifact), 1);
                fs::remove_file(artifact)?;
            }
            // Commit uninstall to state.
            state.remove(&entry.name)?;
            Ok(())
        }
        None => Err(BinmanError::new(
            Cause::NotFound,
            &format!("Target [{}] is not installed", target_name),
        )),
    }
}
