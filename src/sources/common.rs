/// Struct for a Song.
#[derive(Default, Debug)]
pub struct Song {
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

#[derive(Debug)]
pub enum ScrobbleEvent {
    NowPlaying(Song),
    Stopped,
    Scrobble(Song),
}

// pub type ScrobbleStream = Iterator<Item=ScrobbleEvent>;

pub trait ScrobbleSource<'p> {
    fn into_stream(&'p mut self) -> Box<Iterator<Item=ScrobbleEvent> + 'p>;
}
