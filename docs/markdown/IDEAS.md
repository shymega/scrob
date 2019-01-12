# Ideas for Scrobblers.

## Sources/Targets.

Targets:

- Statistics.
- Arbitary HTTP endpoint - POST method.
- File target.
- Script (pass via command-line arguments).
- Script (pass via STDIN).
- Listenbrainz (official, self-hosted).
- Audioscrobbler API (i.e: Last.fm, Libre.fm, self-hosted).

Sources:

- Manual song - inject data via `scrobblers-ctl` client.
- MPRIS.
- MPD.

## Logging

Use `slog-rs` to print to terminal (pretty-printed) and file output.
Log file output: plain text.

`slog-rs` will use `slog-scope`.

## Client/server protocol.

Plain text, similar to MPD.

## Database

Plain text, similar to MPD.
