mod cli;
mod crypto;
use clap::crate_version;
use std::process;

use crate::cli::{build_cli, recover_processor, sign_processor};

fn main() {
    let version = crate_version!().to_string();
    let mut parser = build_cli(version.as_str());
    let matches = parser.clone().get_matches();

    if let Err(e) = match matches.subcommand() {
        ("sign", Some(m)) => sign_processor(m),
        ("recover", Some(m)) => recover_processor(m),
        _ => {
            parser.print_help().unwrap();
            Ok(())
        }
    } {
        eprintln!("{}", e);
        process::exit(1);
    }
}
