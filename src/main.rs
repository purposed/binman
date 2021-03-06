mod cli;

use anyhow::Result;

use clap::Clap;

use cli::{InstallCommand, ListCommand, UninstallCommand, UpdateCommand};

use rood::cli::OutputManager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = VERSION, author = "Purposed")]
struct Root {
    /// Whether to use verbose output.
    #[clap(short = 'v', long = "verbose", global = true)]
    verbose: bool,

    #[clap(subcommand)]
    command: Domain,
}

impl Root {
    pub async fn run(&self) -> Result<()> {
        let cli = OutputManager::new(self.verbose);

        match &self.command {
            Domain::List(cmd) => cmd.run(cli).await?,
            Domain::Install(cmd) => cmd.run(cli).await?,
            Domain::Update(cmd) => cmd.run(cli).await?,
            Domain::Uninstall(cmd) => cmd.run(cli).await?,
        };

        Ok(())
    }
}

#[derive(Clap)]
enum Domain {
    /// List all installed packages.
    #[clap(name = "list")]
    List(ListCommand),

    /// Install a package from a given repository.
    #[clap(name = "install")]
    Install(InstallCommand),

    /// Update a package.
    #[clap(name = "update")]
    Update(UpdateCommand),

    /// Uninstall a package.
    #[clap(name = "uninstall")]
    Uninstall(UninstallCommand),
}

#[tokio::main]
async fn main() {
    if let Err(e) = Root::parse().run().await {
        OutputManager::new(false).error(&e.to_string());
    }
}
