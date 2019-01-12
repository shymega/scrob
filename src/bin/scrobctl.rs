// This file is part of Scrobblers.

// Scrobblers is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Scrobblers is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Scrobblers.  If not, see <http://www.gnu.org/licenses/>

extern crate clap;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use clap::{App, Arg, ArgMatches};
use slog::Drain;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_arguments() -> ArgMatches<'static> {
    App::new("scrobctl")
        .version(VERSION)
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Scrobblers client: Modular scrobbler for your music.")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .required(false)
                .help("Sets the level of logging verbosity."),
        )
        .get_matches()
}

fn init_logger(min_log_level: slog::Level) -> slog::Logger {
    /* initialise decorator */
    let decorator = slog_term::TermDecorator::new().build();
    /* create drain - terminal formatting */
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    /* adjust drain - add minimum level to log at */
    let drain = slog::LevelFilter::new(drain, min_log_level).fuse();
    /* adjust drain - make async */
    let drain = slog_async::Async::new(drain).build().fuse();

    /* return Logger instance */
    slog::Logger::root(drain, o!())
}

fn main() {
    let args = get_arguments();
    let min_log_level = match args.occurrences_of("v") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };

    let _verbosity_count = args.occurrences_of("v");

    let log = init_logger(min_log_level);

    info!(log, "Initialising.");

    unimplemented!();
}
