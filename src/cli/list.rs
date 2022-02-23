use anyhow::Result;

use clap::Parser;

use binlib::{Config, State};

#[derive(Parser)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn run(&self) -> Result<()> {
        let cfg = Config::new()?;

        let state = State::new(&cfg.state_file_path)?;

        let mut installed_applications = state.list();

        installed_applications.sort_by(|a, b| a.name.cmp(&b.name));

        for entry in installed_applications.iter() {
            tracing::info!("{}@{}", &entry.name, &entry.version);
        }
        Ok(())
    }
}
