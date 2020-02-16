use clap::ArgMatches;

use crate::binman::uninstall_target;
use crate::error::{BinmanResult, Cause};
use rood::cli::OutputManager;

pub fn uninstall(matches: &ArgMatches) -> BinmanResult<()> {
    let verbose = matches.is_present("verbose");

    let output = OutputManager::new(verbose);

    let targets: Vec<&str> = matches.values_of("binary").unwrap().collect(); // Mandatory argument.
    for target in targets.iter() {
        let v = uninstall_target(target, &output);
        if let Err(e) = &v {
            if e.cause == Cause::NotFound {
                // Target was not installed.
                output.success(&e.message);
            } else {
                return v;
            }
        }
    }
    output.success("Uninstallation Successful");
    Ok(())
}
