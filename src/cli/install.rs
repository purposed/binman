use clap::ArgMatches;

use rood::cli::OutputManager;

use crate::binman::install_target;
use crate::error::BinmanResult;

pub fn install(matches: &ArgMatches) -> BinmanResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let target: &str = matches.value_of("repo_url").unwrap(); // Mandatory argument.
    let version = matches.value_of("version").unwrap();
    install_target(target, version, &output)?;

    output.success("Installation Successful", 0);
    Ok(())
}
