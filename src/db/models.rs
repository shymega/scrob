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

//! Models module for DB.

/// Struct for a Song's DB entry
#[derive(Debug, Default)]
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
