// This file is part of Scrobblers..

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

/// Struct for a song representation.
#[derive(Debug)]
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

    pub fn default() -> Song {
        Song::new()
    }
}

#[derive(Debug)]
pub enum ScrobbleEvent {
    NowPlaying(Song),
    Stopped,
    Scrobble(Song),
}

pub trait ScrobbleSource<'a> {
    fn into_stream(&'a mut self) -> Box<Iterator<Item = ScrobbleEvent> + 'a>;
}
