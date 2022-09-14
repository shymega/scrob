//! Main client code for `scrob`.
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

use clap::{Arg, ArgMatches, Command};

fn get_arguments() -> ArgMatches {
    Command::new("scrobd")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Dom Rodriguez <shymega@shymega.org.uk>")
            .about("scrob daemon")
            .arg(
                Arg::new("v")
                    .short('v')
                    .multiple_occurrences(true)
                    .required(false)
                    .help("Sets the level of logging verbosity."),
            )
            .subcommand(Command::new("spawn")
                .alias("init")
                .alias("start")
                .about("Spawn the `scrobd` daemon."))
            .get_matches()
}

fn handle_spawn_subcommand() {
    println!("This command is not implemented yet.");
    todo!();
}

fn main() {
    let args = get_arguments();

    match args.subcommand_name() {
        Some("spawn") => {
            handle_spawn_subcommand();
        }
        _ => {}
    }
    todo!()
}
