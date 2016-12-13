// This file is part of Xtensis.

// Xtensis is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Xtensis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Xtensis.  If not, see <http://www.gnu.org/licenses/>

extern crate clap;
extern crate mpd;
extern crate rscribble;

use mpd::Client;

use clap::{App, Arg, ArgMatches, SubCommand};

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
    let matches = get_arguments();

    println!("rscribble starting NOW..");
    println!("Version: {}", VERSION);

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();

    let curr_song = conn.currentsong().unwrap().unwrap();
    let curr_song_title = curr_song.title.unwrap();
    let tags = curr_song.tags;

    match tags.get("Artist") {
        Some(x) => {
            println!("The artist is: {artist}\nThe title is: {title}",
                     artist = x,
                     title = curr_song_title)
        }
        None => println!("Could not find the specified artist."),
    }
}
