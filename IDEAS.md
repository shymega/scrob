# Ideas for Scrobblers.

## Sources/Targets.

Targets:

- Arbitary HTTP endpoint - POST method.
- File target.
- Script (pass via command-line arguments).
- Script (pass to STDIN).
- Listenbrainz.
- Audioscrobbler API (i.e: Last.fm, Libre.fm)

Sources:

- Manual song - inject data via `scrobblers-ctl` client.
- MPRIS.
- MPD.
- Polling a plain text file, using a structured format.

## Logging

Use `slog-rs` to print to terminal and file output.
File output: JSON format, or plain text? Or Both?

## Client/server protocol.

Protocol will be definded later.

Considering JSON-RPC or just plain text.

## Database

SQLite backend

Design SQL schema required.
