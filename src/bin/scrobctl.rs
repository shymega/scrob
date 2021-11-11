#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs, temp disable
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

use clap::{App, Arg, ArgMatches};

fn get_arguments() -> ArgMatches<'static> {
    App::new("scrobc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Client for controlling scrob")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .required(false)
                .help("Sets the level of logging verbosity."),
        )
        .get_matches()
}

fn main() {
    let _args = get_arguments();

    unimplemented!();
}
