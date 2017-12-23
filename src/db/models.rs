//! Models module for DB.

/// Struct for a Song's DB entry
#[derive(Debug)]
pub struct DbSong {
    id: i32,
    title: String,
    album: String,
    artist: String,
    album_artist: String,
    genre: String,
    track: String,
    composer: String,
    ts_created: String,
    ts_inputted: String,
    rscr_source: String,
}
