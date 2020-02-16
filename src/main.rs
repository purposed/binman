use clap::{App, AppSettings, Arg, SubCommand};

mod binman;
mod error;

mod cli;
use cli::run_main;

mod github;
use rood::cli::OutputManager;

fn main() {
    let app = App::new("binman")
        .version("v0.1.0")
        .author("Purposed")
        .about("Binary Package Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists installed binaries")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Whether to use verbose output")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs a binary")
                .arg(
                    Arg::with_name("repo_url")
                        .required(true)
                        .help("The URL of the github repo from which to fetch the binary"),
                )
                .arg(
                    Arg::with_name("version")
                        .required(false)
                        .default_value("latest")
                        .help("The version to install."),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Whether to use verbose output")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates a binary")
                .arg(
                    Arg::with_name("binary")
                        .required(false)
                        .multiple(true)
                        .help("The name of the binary to update"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .required(false)
                        .help("Whether to use verbose output"),
                ),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstalls a binary")
                .arg(
                    Arg::with_name("binary")
                        .required(true)
                        .multiple(true)
                        .help("The name of the binary to uninstall"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Whether to use verbose output")
                        .required(false),
                ),
        );

    let root_om = OutputManager::new(false);
    match run_main(app.get_matches()) {
        Ok(_) => {}
        Err(e) => root_om.error(&format!("{}", e)),
    }
}
