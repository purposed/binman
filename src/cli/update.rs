use clap::ArgMatches;
use rood::cli::OutputManager;

use crate::binman::update_target;
use crate::error::BinmanResult;

pub fn update(matches: &ArgMatches) -> BinmanResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let targets: Vec<&str> = matches.values_of("binary").unwrap().collect();
    for target in targets.iter() {
        update_target(target, &output)?;
    }

    output.success("Update Complete", 0);

    Ok(())
}
