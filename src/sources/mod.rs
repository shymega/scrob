//! This module defines the MPD and MPRIS sources.

pub mod mpd;
pub mod mpris;
mod common;

use self::common::{Song, ScrobbleEvent, ScrobbleSource};
