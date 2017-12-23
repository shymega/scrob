//! This is the module for the `MPD` source for scrobble.rs.

// This file is part of scrobble.rs.

// scrobble.rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// scrobble.rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with scrobble.rs.  If not, see <http://www.gnu.org/licenses/>

pub mod utils;

extern crate mpd;

use mpd::Client;
use mpd::idle::Idle;
use mpd::idle::Subsystem::Player;
use self::utils::get_tag;
use std::net::TcpStream;
use std::process::exit;

/// Struct for a Song.
#[derive(Default, Debug)]
struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub album_artist: String,
    pub date: String,
    pub genre: String,
    pub track: String,
    pub composer: String,
}

impl Song {
    fn new() -> Song {
        Song {
            title: "".to_string(),
            album: "".to_string(),
            artist: "".to_string(),
            album_artist: "".to_string(),
            date: "".to_string(),
            genre: "".to_string(),
            track: "".to_string(),
            composer: "".to_string(),
        }
    }
}

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
