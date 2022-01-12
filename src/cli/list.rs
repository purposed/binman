use anyhow::Result;

use clap::Parser;

use rood::cli::OutputManager;

use binlib::{Config, State, StateEntry};

fn display_entry(output: &OutputManager, entry: &StateEntry) {
    output.step(&format!("{}@{}", &entry.name, &entry.version));

    let pushed = output.push();
    for artifact in entry.artifacts.iter() {
        pushed.debug(&artifact);
    }
}

#[derive(Parser)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn run(&self, output: OutputManager) -> Result<()> {
        let cfg = Config::new()?;

        let state = State::new(&cfg.state_file_path)?;

        let mut installed_applications = state.list();

        installed_applications.sort_by(|a, b| a.name.cmp(&b.name));

        for entry in installed_applications.iter() {
            display_entry(&output, entry);
        }
        Ok(())
    }
}
