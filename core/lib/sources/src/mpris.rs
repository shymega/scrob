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

use mpris::{DBusError, Event, Metadata, PlaybackStatus, Player, PlayerFinder, Progress};

use super::common::{ScrobbleEvent, ScrobbleSource, Song};

/// Loop over MPRIS2 events, and display new artist/song combination.
/// This is for debugging, and only enabled on a debug build.
#[cfg(debug_assertions)]
pub fn display_mpris_songs() {
    let mut sources = get_mpris_sources();
    let mut player = sources.next().expect("Player &mut fail.");

    for ev in player.into_stream() {
        println!("Event: {:?}", ev);
    }
}

/// Returns an `Iterator` over all MPRIS2 sources available to the system.
fn get_mpris_sources<'a>() -> impl Iterator<Item = MprisSource<'a>> {
    PlayerFinder::new()
        .expect("Failed to connect to D-Bus.")
        .find_all()
        .expect("Could not find list of players.")
        .into_iter()
}

/// ScrobbleSource for MPRIS2.
type MprisSource<'a> = Player<'a>;

fn mpris_event_to_scrobble_event(ev: Result<Event, DBusError>) -> Option<ScrobbleEvent> {
    println!("MPRIS data: {:?}", ev);

    use mpris::Event;

    match ev.ok()? {
        Event::Playing => {
            let song = Song::default();
            Some(ScrobbleEvent::NowPlaying(song))
        }
        Event::Paused => None,
        Event::Stopped => Some(ScrobbleEvent::Stopped),
        ev => {
            println!("MPRIS data: {:?}", ev);
            None
        }
    }
}

impl<'a> ScrobbleSource<'a> for MprisSource<'a> {
    fn into_stream(&'a mut self) -> Box<Iterator<Item = ScrobbleEvent> + 'a> {
        Box::new(
            self.events()
                .expect("failed source")
                .filter_map(mpris_event_to_scrobble_event),
        )
    }
}
