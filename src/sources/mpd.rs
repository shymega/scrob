//! This is the module for the `MPD` source for rscribble.

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

extern crate mpd;
extern crate rusqlite;

use mpd::Client;
use mpd::idle::Idle;
use mpd::idle::Subsystem::Player;
use self::rusqlite::Connection;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::exit;

/// Struct for a DB entry for a song.
#[derive(Debug)]
struct DbSong {
    id: i32,
    title: String,
    album: String,
    artist: String,
    album_artist: String,
    genre: String,
    track: String,
    composer: String,
    date: String,
}

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
    pub fn new() -> Song {
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

/// Return the Artist in a `BTree` of tags.
pub fn get_artist(tags: BTreeMap<String, String>) -> String {
    match tags.get("Artist") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the Album in a `BTree` of tags.
pub fn get_album(tags: BTreeMap<String, String>) -> String {
    match tags.get("Album") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the AlbumArtist in a `BTree` of tags.
pub fn get_album_artist(tags: BTreeMap<String, String>) -> String {
    match tags.get("AlbumArtist") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the Date in a `BTree` of tags.
pub fn get_date(tags: BTreeMap<String, String>) -> String {
    match tags.get("Date") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the Genre in a `BTree` of tags.
pub fn get_genre(tags: BTreeMap<String, String>) -> String {
    match tags.get("Genre") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the Track nymber in a `BTree` of tags.
pub fn get_track(tags: BTreeMap<String, String>) -> String {
    match tags.get("Track") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Return the Composer in a `BTree` of tags.
pub fn get_composer(tags: BTreeMap<String, String>) -> String {
    match tags.get("Composer") {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

/// Loop over MPD `Player` events, and display the song and artist.
pub fn display_mpd_songs() {
    let addr = "127.0.0.1:6600";
    let mut conn = match Client::connect(addr) {
        Ok(x) => x,
        Err(e) => {
            println!("Error found while creating client..");
            println!("Error: {error}", error = e);
            println!("Cannot proceed, bailing.");
            exit(1);
        }
    };

    let db_path = Path::new("/home/dzrodrig/.rscribble/music.db");
    let db_conn = match Connection::open(db_path) {
        Ok(x) => x,
        Err(e) => {
            println!("Error found while creating DB connection..");
            println!("Error: {error}", error = e);
            println!("Cannot proceed, bailing.");
            exit(1);
        }
    };

    loop {
        let _ = conn.wait(&[Player]);
        if let Some(s) = conn.currentsong().unwrap() {
            let mut song = Song::new();

            let song = Song {
                title: s.title.unwrap(),
                artist: get_artist(s.tags.clone()),
                album: get_album(s.tags.clone()),
                album_artist: get_album_artist(s.tags.clone()),
                date: get_date(s.tags.clone()),
                genre: get_genre(s.tags.clone()),
                track: get_track(s.tags.clone()),
                composer: get_composer(s.tags.clone()),
            };

            let db_song = DbSong {
                id: 0,
                title: song.title.to_string(),
                album: song.album.to_string(),
                artist: song.artist.to_string(),
                album_artist: song.album_artist.to_string(),
                genre: song.genre.to_string(),
                track: song.track.to_string(),
                composer: song.composer.to_string(),
                date: song.date.to_string(),
            };

            db_conn.execute("INSERT INTO Songs (title, album, artist,
                          \
                          album_artist, genre, track, composer, date)
                             \
                          VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                         &[&db_song.title,
                           &db_song.album,
                           &db_song.artist,
                           &db_song.album_artist,
                           &db_song.genre,
                           &db_song.track,
                           &db_song.composer,
                           &db_song.date])
                .unwrap();

            println!("**************************");
            println!("New song playing!");
            println!("{title}, by {artist}",
                     title = song.title,
                     artist = song.artist);
            println!("**************************");
        }
    }
}
