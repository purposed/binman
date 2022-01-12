use anyhow::Result;

use binlib::update_target;

use clap::Parser;

use rood::cli::OutputManager;

#[derive(Parser)]
pub struct UpdateCommand {
    /// The name of the package(s) to update.
    binary: Vec<String>,
}

impl UpdateCommand {
    pub async fn run(&self, output: OutputManager) -> Result<()> {
        for target in self.binary.iter() {
            update_target(target, &output).await?;
        }

        output.success("Update Complete");

        Ok(())
    }
}
