use anyhow::Result;

use binlib::uninstall_target;

use clap::Parser;

#[derive(Parser)]
pub struct UninstallCommand {
    /// The package(s) to uninstall.
    binary: Vec<String>,
}

impl UninstallCommand {
    pub async fn run(&self) -> Result<()> {
        for target in self.binary.iter() {
            uninstall_target(target)?;
        }
        Ok(())
    }
}
