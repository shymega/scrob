# Scrobblers.

This is a modular audio scrobbler written in Rust. Supports multiple
sources, such as MPRIS and MPD.

Scrobblers works with the concept of "sources" and "targets".

Targets are a *destination* for scrobbles, such as GNU FM (Libre.fm,
for example), Last.fm or ListenBrainz.

Sources are a *source* of data for scrobbles to be formed from, such
as MPD, or MPRIS

## Project status

I'm currently waiting on the further development of [rustfm][rustfm],
which I am hoping will be able to support both GNU FM and Last.fm, of
which implement the Audioscrobbler API. This allows for a greater
range of services to be supported, as long as they support the aforementioned
API.

A Rust library for Listenbrainz is also necessary for the target of the same
name.

[rustfm]: https://github.com/RoxasShadow/rustfm
