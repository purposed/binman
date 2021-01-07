use anyhow::Result;

use binlib::install_target;

use clap::Clap;

use rood::cli::OutputManager;

#[derive(Clap)]
pub struct InstallCommand {
    /// The repository URL.
    #[clap(name = "repo_url")]
    repo_url: String,

    /// The pacakge version.
    #[clap(name = "version", default_value = "latest")]
    version: String,

    /// The installation directory (overrides config.json)
    #[clap(name = "dir", long = "dir", value_name = "INSTALL_DIR")]
    dir: Option<String>,
}

impl InstallCommand {
    pub async fn run(&self, output: OutputManager) -> Result<()> {
        output.step("Installation");
        install_target(
            &self.repo_url,
            &self.version,
            &output.push(),
            self.dir.as_ref(),
        )
        .await?;
        output.success("Installation Successful");
        Ok(())
    }
}
