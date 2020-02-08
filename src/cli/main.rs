use clap::ArgMatches;

use crate::error::BinmanResult;

use super::{install, list, uninstall, update};

pub fn run_main(matches: ArgMatches) -> BinmanResult<()> {
    match matches.subcommand() {
        ("list", Some(m)) => list(m),
        ("install", Some(m)) => install(m),
        ("update", Some(m)) => update(m),
        ("uninstall", Some(m)) => uninstall(m),
        _ => Ok(()),
    }
}
