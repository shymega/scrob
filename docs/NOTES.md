Ideas for scrob
===============

## Sources/Targets.

Targets:

- Default: SQLite storage, statistics available by `scrobctl`
- HTTP endpoint - POST method.
- File target.
- Script (pass via command-line arguments).
- Script (pass via STDIN (JSON)).
- Listenbrainz (official, self-hosted).
- Audioscrobbler API (i.e: Last.fm, Libre.fm, self-hosted).

Filtering - JSON <-> STDIN, STDOUT <-> JSON.

Sources:

- Manual song - inject data via `scrobctl` client.
- MPRIS.
- MPD.

## Logging

Use `slog-rs` to print to terminal (plain), and journald (configurable by
feature flag).
