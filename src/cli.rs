mod recover_command;
mod sign_command;
mod util;

use clap::{App, AppSettings};

pub use self::recover_command::{recover_command, recover_processor};
pub use self::sign_command::{sign_command, sign_processor};

/// Generate cli
pub fn build_cli(version: &str) -> App {
    App::new("digital-sign")
        .version(version)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .subcommand(sign_command())
        .subcommand(recover_command())
}
