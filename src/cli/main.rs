use clap::ArgMatches;

use crate::error::BinmanResult;

use super::{install, list, uninstall, update};

#[tokio::main]
pub async fn run_main(matches: ArgMatches) -> BinmanResult<()> {
    match matches.subcommand() {
        ("list", Some(m)) => list(m),
        ("install", Some(m)) => install(m).await,
        ("update", Some(m)) => update(m).await,
        ("uninstall", Some(m)) => uninstall(m),
        _ => Ok(()),
    }
}
