use std::collections::BTreeMap;

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

/// SongTags is a representation of the tags associated with a
/// song. This is a BTreeMap with two Strings.
pub(crate) type SongTags = BTreeMap<String, String>;

/// This function takes tags of SongTags type, and a tag to retrieve.
pub(crate) fn get_tag(tags: &SongTags, tag: &str) -> String {
    tags.get(tag).map(String::to_string).unwrap_or_default()
}
