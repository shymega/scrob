// This file is part of rscribble.

// rscribble is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// rscribble is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with rscribble.  If not, see <http://www.gnu.org/licenses/>

extern crate clap;
extern crate mpd;
extern crate rscribble;

use rscribble::sources::mpd::display_mpd_songs;

use clap::{App, Arg, ArgMatches};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_arguments() -> ArgMatches<'static> {
    App::new("rscribble")
        .version(VERSION)
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Extensible editing: screw limits.")
        .about("A modular scrobbler for a variety of music players.")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .required(false)
            .help("Sets the level of logging verbosity."))
        .get_matches()
}

fn main() {
    let args = get_arguments();

    println!("rscribble starting NOW..");
    display_mpd_songs();

}
