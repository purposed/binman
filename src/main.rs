mod cli;

use anyhow::Result;

use clap::Parser;

use cli::{InstallCommand, ListCommand, UninstallCommand, UpdateCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
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
        match &self.command {
            Domain::List(cmd) => cmd.run().await?,
            Domain::Install(cmd) => cmd.run().await?,
            Domain::Update(cmd) => cmd.run().await?,
            Domain::Uninstall(cmd) => cmd.run().await?,
        };

        Ok(())
    }
}

#[derive(Parser)]
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    return Root::parse().run().await;
}
