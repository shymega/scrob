// This file is part of Scrobblers.

// Scrobblers is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Scrobblers is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Scrobblers.  If not, see <http://www.gnu.org/licenses/>

//! Utilities module for MPD source.

use std::collections::BTreeMap;

/// SongTags is a representation of the tags associated with a
/// song. This is a BTreeMap with two Strings.
pub type SongTags = BTreeMap<String, String>;

/// This function takes tags of SongTags type, and a tag to retrieve.
pub fn get_tag(tags: &SongTags, tag: &str) -> Result<String, String> {
    /* I'd like to tidy this up a bit. I don't think I'm using `Result<>` correctly, and I don't
     * like the way I've written it either.
     * TODO: Tidy/refactor, make idiomatic. */
    match tags.get(tag) {
        Some(x) => Ok(x.to_string()),
        None => Ok("None defined".to_string()),
    }
}
