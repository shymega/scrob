/// Struct for a song representation.
#[derive(Debug, Default, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub date: String,
    pub genre: String,
    pub track: String,
    pub composer: String,
}

impl Song {
    pub fn new() -> Song {
        Song {
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            album_artist: String::new(),
            date: String::new(),
            genre: String::new(),
            track: String::new(),
            composer: String::new(),
        }
    }
}
