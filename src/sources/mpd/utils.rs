//! Utilities module for MPD source.

use std::collections::BTreeMap;

/// SongTags is a representation of the tags associated with a
/// song. This is a BTreeMap with two Strings.
pub type SongTags = BTreeMap<String, String>;

/// This function takes tags of SongTags type, and a tag to retrieve.
pub fn get_tag(tags: &SongTags, tag: &str) -> Option<String> {
    match tags.get(tag) {
        Some(t) => Some(t),
        None => None,
    }
}
