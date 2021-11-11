# scrob.
=====

This is a modular audio scrobbler written in Rust. Supports multiple
sources, such as MPRIS and MPD.

scrob works with the concept of "sources" and "targets".

Targets are a *destination* for scrobbles, such as GNU FM (Libre.fm,
for example), Last.fm or ListenBrainz.

Sources are a *source* of data for scrobbles to be formed from, such
as MPD, or MPRIS.

## Project status

Working on project locally. Refactoring to introduce queues, worker
threads,and proper error handling - especially with sources and
targets.

## IRC

If you want to have a real-time conversation, feel free to drop in the IRC
channel for this project on Libera.chat - `#scrob`
