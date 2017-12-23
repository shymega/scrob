# scrobble.rs

This is a modular audio scrobbler written in Rust. Supports multiple
sources, such as MPRIS and MPD.

scrobble.rs works with the concept of "sources" and "targets".

Targets are a *destination* for scrobbles, such as GNU FM (Libre.fm,
for example), Last.fm or ListenBrainz.

Sources are a *source* of data for scrobbles to be formed from, such
as MPD, or MPRIS - MPRIS could be supplied, in turn, by the Spotify
desktop client or Tomahawk, to name a few examples.

## Project status

I'm currently waiting on the further development of [rustfm][rustfm],
which I am hoping will be able to support both GNU FM and Last.fm,
which both have similar APIs, to work seamlessly with scrobble.rs.

This, in essence. means that scrobble.rs can _also_ support multiple
"scrobble targets" as well.

Work also needs to commence on a Rust library for the ListenBrainz
API.

[rustfm]: https://github.com/RoxasShadow/rustfm
