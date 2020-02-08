mod config;
pub mod fuzzy_semver;
mod install;
mod state;
mod uninstall;
mod update;

pub use config::Config;
pub use install::install_target;
pub use state::{State, StateEntry};
pub use uninstall::uninstall_target;
pub use update::update_target;
