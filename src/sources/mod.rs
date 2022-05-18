//! This module defines the MPD and MPRIS sources.

pub(crate) mod common;

#[cfg(feature = "mpris-source")]
pub(crate) mod mpris;

#[cfg(feature = "mpd-source")]
pub(crate) mod mpd;
