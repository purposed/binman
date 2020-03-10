use clap::ArgMatches;

use rood::cli::OutputManager;

use crate::binman::{Config, State, StateEntry};
use crate::error::BinmanError;

fn display_entry(output: &OutputManager, entry: &StateEntry) {
    output.step(&format!("{}@{}", &entry.name, &entry.version));

    let pushed = output.push();
    for artifact in entry.artifacts.iter() {
        pushed.debug(&artifact);
    }
}

pub fn list(matches: &ArgMatches) -> Result<(), BinmanError> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);
    let cfg = Config::new()?;

    let state = State::new(&cfg.state_file_path)?;

    let mut installed_applications = state.list();

    installed_applications.sort_by(|a, b| a.name.cmp(&b.name));

    for entry in installed_applications.iter() {
        display_entry(&output, entry);
    }
    Ok(())
}
