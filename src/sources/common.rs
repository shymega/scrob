/// Struct for a song.
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
