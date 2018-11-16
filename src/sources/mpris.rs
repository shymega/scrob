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

//! This is the module for the `MPRIS` source for Scrobblers.
//! It is currently a work in progress. This source needs a stable and
//! well-tested D-Bus library for Rust.

use mpris::{
    DBusError, Event, LoopStatus, Metadata, PlaybackStatus, Player, PlayerFinder, Progress,
    ProgressTracker,
};

use sources::common::{ScrobbleEvent, ScrobbleSource, Song};

/// Loop over MPRIS2 events, and display new artist/song combination.
/// This is for debugging, and only enabled on a debug build.
// #[cfg(debug_assertions)]
pub fn display_mpris_songs() {
    let mut sources = get_mpris_sources();
    let mut player = sources.next().expect("player mut FAIL");

    for ev in player.into_stream() {
        println!("Event: {:?}", ev);
    }
}

/// Returns an `Iterator` over all MPRIS2 sources available to the system.
fn get_mpris_sources<'p>() -> impl Iterator<Item = MprisSource<'p>> {
    PlayerFinder::new()
        .expect("Could not start MPRIS player finder.")
        .find_all()
        .expect("Error finding all MPRIS players.")
        .into_iter()
}

/// ScrobbleSource for MPRIS2.
type MprisSource<'p> = Player<'p>;

fn mpris_event_to_scrobble_event(ev: Result<Event, DBusError>) -> Option<ScrobbleEvent> {
    println!("MPRIS event: {:?}", ev);

    use mpris::Event::*;

    match ev.ok()? {
        Playing => {
            let song = Song::new();
            Some(ScrobbleEvent::NowPlaying(song))
        }
        Paused => None,
        Stopped => Some(ScrobbleEvent::Stopped),
        ev => None,
    }
}

impl<'p> ScrobbleSource<'p> for MprisSource<'p> {
    fn into_stream(&'p mut self) -> Box<Iterator<Item = ScrobbleEvent> + 'p> {
        Box::new(
            self.events()
                .expect("failed source")
                .filter_map(mpris_event_to_scrobble_event),
        )
    }
}
