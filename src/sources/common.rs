/// Struct for a song.
#[derive(Debug, Default)]
pub(crate) struct Song {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    date: Option<String>,
    genre: Option<String>,
    track: Option<String>,
    composer: Option<String>,
}
