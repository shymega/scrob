//! Utilities module for Scrobblers.

/// Get the version from Cargo.
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
