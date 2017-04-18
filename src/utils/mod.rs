//! Utilities module for rscribble.

/// Get the version from Cargo.
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
