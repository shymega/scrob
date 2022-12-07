//! Main daemon code for `scrob`.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use clap::{Arg, ArgAction, ArgMatches, Command};

fn get_arguments() -> ArgMatches {
    Command::new("scrobc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Client for controlling `scrobd`")
        .subcommand_required(true)
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .required(false)
                .help("Sets the level of logging verbosity."),
        )
        .get_matches()
}

fn main() {
    let _args = get_arguments();

    unimplemented!();
}
