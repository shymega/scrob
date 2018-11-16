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

//! The library for Scrobblers.

#![cfg_attr(feature = "cargo-clippy", feature(plugin))]
#![cfg_attr(feature = "cargo-clippy", plugin(clippy))]
// Enable as many useful Rust and Clippy warnings as we can stand.  We'd
// also enable `trivial_casts`, but we're waiting for
// https://github.com/rust-lang/rust/issues/23416.
#![warn(missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]
#![cfg_attr(feature = "cargo-clippy", deny(cast_possible_truncation))]
#![cfg_attr(feature = "cargo-clippy", deny(cast_possible_wrap))]
#![cfg_attr(feature = "cargo-clippy", deny(cast_precision_loss))]
#![cfg_attr(feature = "cargo-clippy", deny(cast_sign_loss))]
#![cfg_attr(
    feature = "cargo-clippy",
    deny(missing_docs_in_private_items)
)]
#![cfg_attr(feature = "cargo-clippy", deny(mut_mut))]
// Disallow `println!`. Use `debug!` for debug output
// (which is provided by the `log` crate).
#![cfg_attr(feature = "cargo-clippy", deny(print_stdout))]
// This allows us to use `unwrap` on `Option` values (because doing makes
// working with Regex matches much nicer) and when compiling in test mode
// (because using it in tests is idiomatic).
#![cfg_attr(
    all(not(test), feature = "cargo-clippy"),
    deny(result_unwrap_used)
)]
#![cfg_attr(feature = "cargo-clippy", deny(unseparated_literal_suffix))]
#![cfg_attr(feature = "cargo-clippy", deny(wrong_pub_self_convention))]

extern crate mpd;
extern crate mpris;
extern crate time;

#[macro_use]
extern crate slog;

extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_term;

extern crate serde;

pub mod db;
pub mod logging;
pub mod sources;
pub mod targets;
