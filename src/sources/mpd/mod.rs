mod utils;

use mpd::idle::Idle;
use mpd::idle::Subsystem::Player;
use mpd::Client;
use std::net::TcpStream;
use std::process::exit;
use utils::get_tag;

use super::common::Song;

fn get_mpd_conn(addr: &str) -> Client<TcpStream> {
    match Client::connect(addr) {
        Ok(x) => x,
        Err(e) => {
            println!("Error found while creating client...");
            println!("Error: {}", e);
            println!("Cannot proceed, bailing!");
            exit(1);
        }
    }
}

/// Loop over MPD `Player` events, and display the song and artist.
/// This is for debugging, and only enabled on a debug build.
// #[cfg(debug_assertons)]
pub fn display_mpd_songs() {
    let addr = "127.0.0.1:6600";

    let mut conn = get_mpd_conn(addr);

    let mut song = Song::new();

    loop {
        let _t = conn.wait(&[Player]);

        if let Some(s) = conn.currentsong().unwrap() {
            let tags = s.tags.clone();
            let song_title = s.title.clone().unwrap();

            if song.title == song_title {
                println!("New song is the SAME as the old song!");
                println!("Logically, I should skip. Skipping.");
                continue;
            }
            song.title = s.title.unwrap();
            song.album = get_tag(&tags, "Album").unwrap_or_default();
            song.album_artist =
                get_tag(&tags, "AlbumArtist").unwrap_or_default();
            song.artist = get_tag(&tags, "Artist").unwrap_or_default();
            song.date = get_tag(&tags, "Date").unwrap_or_default();
            song.genre = get_tag(&tags, "Genre").unwrap_or_default();
            song.track = get_tag(&tags, "Track").unwrap_or_default();
            song.composer = get_tag(&tags, "Composer").unwrap_or_default();

            println!("**************************");
            println!("New song playing!");
            println!(
                "{title}, by {artist}",
                title = song.title,
                artist = song.artist
            );
            println!("**************************");
        }
    }
}
