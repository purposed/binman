use anyhow::Result;

use binlib::uninstall_target;

use clap::Parser;

use rood::cli::OutputManager;

#[derive(Parser)]
pub struct UninstallCommand {
    /// The package(s) to uninstall.
    binary: Vec<String>,
}

impl UninstallCommand {
    pub async fn run(&self, output: OutputManager) -> Result<()> {
        for target in self.binary.iter() {
            uninstall_target(target, &output)?;
        }
        output.success("Uninstallation Successful");
        Ok(())
    }
}
