//! This is the module for the `MPRIS` source for scrobble.rs.
//! It is currently a work in progress. This source needs a stable and
//! well-tested D-Bus library for Rust.

use mpris::{LoopStatus, Metadata, PlaybackStatus, PlayerFinder, Progress};

/// Loop over MPRIS events, and display new song/artist combos.
pub fn display_mpris_songs() {
    let player = PlayerFinder::new().unwrap().find_active().unwrap();

    let identity = player.identity();
    let mut progress_tracker = player.track_progress(1000).unwrap();

    loop {
        let (progress, did_refresh) = progress_tracker.tick();

        let meta = progress.metadata();

        if !did_refresh {
            continue;
        }
        println!("{} by {}", meta.title().unwrap(), meta.artists().unwrap()[0]);
    }
}
