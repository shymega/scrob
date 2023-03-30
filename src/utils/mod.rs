//! Module exporting various utilities for `scrob`, including logging.

use std::borrow::Borrow;

mod logging;

pub use logging::*;

/// `StrType` is derived from a PR on `listenbrainz-rs`.
pub trait StrType: Borrow<str> {}
impl<T: Borrow<str>> StrType for T {}
