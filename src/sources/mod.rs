//! This module defines the MPD and MPRIS sources.

pub mod mpd;
pub mod mpris;

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
