//! This is the module for the `MPRIS` source for scrobble.rs.
//! It is currently a work in progress. This source needs a stable and
//! well-tested D-Bus library for Rust.

use std::thread;
use mpris::{DBusError, Event, LoopStatus, Metadata, PlaybackStatus, PlayerFinder, Progress, ProgressTracker, Player};
use ::sources::common::{Song, ScrobbleEvent, ScrobbleSource};

/// Loop over MPRIS events, and display new song/artist combos.
pub fn display_mpris_songs() {
    let mut sources = get_mpris_sources();
    let mut player = sources.next().unwrap();

    for ev in player.into_stream() {
        println!("Event: {:?}", ev);
    }
}

/// Returns an Iterator over all MPRIS2 sources available to the system
pub fn get_mpris_sources<'p>() -> impl Iterator<Item=MprisSource<'p>> {
    PlayerFinder::new()
        .expect("Could not start mpris player finder")
        .find_all()
        .expect("Error finding all mpris players")
        .into_iter()
}

/// ScrobbleSource for MPRIS2
pub type MprisSource<'p> = Player<'p>;

fn mpris_event_to_scrobble_event(ev: Result<Event, DBusError>) -> Option<ScrobbleEvent> {
    println!("mpris event {:?}", ev);

    use mpris::Event::*;

    match ev.ok()? {
        Playing => {
            let song = Song::default();
            Some(ScrobbleEvent::NowPlaying(song))
        },
        Paused => None,
        Stopped => Some(ScrobbleEvent::Stopped),
        ev => unimplemented!("mpris event: {:?}", ev),
    }
}

impl<'p> ScrobbleSource<'p> for MprisSource<'p> {
    fn into_stream(&'p mut self) -> Box<Iterator<Item=ScrobbleEvent> + 'p> {
        Box::new(self.events()
                 .unwrap()
                 .filter_map(mpris_event_to_scrobble_event))
    }
}
