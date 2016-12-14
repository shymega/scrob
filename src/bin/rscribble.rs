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

use mpd::idle::Idle;
use mpd::Client;
use mpd::idle::Subsystem::Player;
use std::collections::BTreeMap;

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

fn get_artist(tags: BTreeMap<String, String>) -> String {
    match tags.get("Artist") {
        Some(x) => x.to_owned(),
        None => "None".to_owned(),
    }
}

fn main() {
    let _ = get_arguments();

    println!("rscribble starting NOW..");

    let addr = "127.0.0.1:6600";
    let mut conn = Client::connect(addr).unwrap();

    loop {
        let _ = conn.wait(&[Player]);
        match conn.currentsong().unwrap() {
            Some(s) => {
                println!("New song detected.");
                let song_title = s.title.unwrap();
                let song_artist = get_artist(s.tags);
                println!("Song title: {}", song_title);
                println!("Song artist: {}", song_artist);
            }
            None => {
                println!("No value.");

            }
        }
    }

}
