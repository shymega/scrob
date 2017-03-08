# rscrobble

Modular audio scrobbler written in Rust. Supports multiple sources,
such as MPRIS and MPD. (That's pretty much it, right now.)

RScrobble works with the concept of "sources" and "targets".

Targets are a *destination* for scrobbles, such as GNU FM (Libre.fm,
for example) or Last.fm.

Sources are a *source* of data for scrobbles to be formed from, such
as MPD, or MPRIS - MPRIS could be supplied, in turn, by the Spotify
desktop client or Tomahawk.

## Project status

I'm currently waiting on the further development of [rustfm][rustfm],
which I am hoping will be able to support both GNU FM and Last.fm
(They have similar APIs) as multiple backends. This means that
rscrobble can _also_ support multiple "scrobble targets" as well.

[rustfm]: https://github.com/RoxasShadow/rustfm
