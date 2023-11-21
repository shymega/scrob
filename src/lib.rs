//! Associated crate for the `scrob` binaries.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(dead_code)]

mod db;
mod rpc;
mod sources;
mod targets;
pub mod utils;
