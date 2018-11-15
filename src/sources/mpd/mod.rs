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

//! This is the module for the `MPD` source for Scrobblers.

pub mod utils;

extern crate mpd;

use self::utils::get_tag;
use mpd::idle::Idle;
use mpd::idle::Subsystem::Player;
use mpd::Client;
use std::net::TcpStream;
use std::process::exit;

use sources::common::Song;

fn get_mpd_conn(addr: &str) -> Client<TcpStream> {
    match Client::connect(addr) {
        Ok(x) => x,
        Err(e) => {
            println!("Error found while creating client...");
            println!("Error: {error_msg}", error_msg = e);
            println!("Cannot proceed, bailing!");
            exit(1);
        }
    }
}

/// Loop over MPD `Player` events, and display the song and artist.
pub fn display_mpd_songs() {
    let addr = "127.0.0.1:6600";

    let mut conn = get_mpd_conn(addr);

    let mut song = Song::new();

    loop {
        let _ = conn.wait(&[Player]);

        if let Some(s) = conn.currentsong().unwrap() {
            let tags = s.tags.clone();
            let song_title = s.title.clone().unwrap();

            if song.title == song_title {
                println!("New song is the SAME as the old song!");
                println!("Logically, I should skip. Skipping.");
                continue;
            } else {
                song.title = s.title.unwrap();
                song.album = get_tag(&tags, "Album").unwrap();
                song.album_artist = get_tag(&tags, "AlbumArtist").unwrap();
                song.artist = get_tag(&tags, "Artist").unwrap();
                song.date = get_tag(&tags, "Date").unwrap();
                song.genre = get_tag(&tags, "Genre").unwrap();
                song.track = get_tag(&tags, "Track").unwrap();
                song.composer = get_tag(&tags, "Composer").unwrap();
            }

            println!("**************************");
            println!("New song playing!");
            println!("{title}, by {artist}",
                     title = song.title,
                     artist = song.artist);
            println!("**************************");

        }
    }
}
