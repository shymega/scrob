//! This is the module for the `MPRIS` source for scrobble.rs.
//! It is currently a work in progress. This source needs a stable and
//! well-tested D-Bus library for Rust.

use mpris::{LoopStatus, Metadata, PlaybackStatus, PlayerFinder, Progress, ProgressTracker, Player};
use ::sources::common::{Song, ScrobbleEvent, ScrobbleStream, ScrobbleSource};

/// Loop over MPRIS events, and display new song/artist combos.
pub fn display_mpris_songs() {
    let mut sources = get_mpris_sources();
    let player = sources.next().unwrap();

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
        .map(|player| {
            MprisSource {
                player,
            }
        })
}

#[derive(Debug)]
/// ScrobbleSource for MPRIS2
pub struct MprisSource<'p> {
    player: Player<'p>,
}

impl<'p> Iterator for MprisSource<'p> {
    type Item = ScrobbleEvent;

    fn next(&mut self) -> Option<ScrobbleEvent> {
        let mut current_track_id: String = self.player.get_metadata()
            .unwrap().track_id().to_owned();

        // XXX: we should only get one ProgressTracker,
        // since I'm not sure whether the short time inbetween
        // two next() calls drops events.
        let mut progress_tracker = self.player.track_progress(100)
            .expect("Cannot start progress tracker");

        loop {
            let (progress, was_refreshed) = progress_tracker.tick();
            if ! was_refreshed { continue }

            let meta = progress.metadata();

            let track_id = meta.track_id();
            if track_id == current_track_id {
                continue;
            }
            current_track_id = track_id.to_owned();

            let title = meta.title().unwrap().to_owned();
            let artists = meta.artists().unwrap();
            let artist = artists[0].clone();

            let song = Song {
                title,
                artist,
                .. Song::default()
            };

            // TODO: also emit a Scrobble event of the previous song
            return Some(ScrobbleEvent::NowPlaying(song));
        }
    }
}

impl ScrobbleSource for MprisSource<'static> {
    fn into_stream(self) -> Box<ScrobbleStream> {
        Box::new(self)
    }
}
