use anyhow::Result;

use binlib::update_target;

use clap::Parser;

#[derive(Parser)]
pub struct UpdateCommand {
    /// The name of the package(s) to update.
    binary: Vec<String>,

    /// Whether to force a re-install if versions are identitcal.
    #[clap(short = 'f', long = "force")]
    force: bool,
}

impl UpdateCommand {
    pub async fn run(&self) -> Result<()> {
        for target in self.binary.iter() {
            update_target(target, self.force).await?;
        }

        Ok(())
    }
}
