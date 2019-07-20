-- Initial revision of SQL script.

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS Songs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  album TEXT NOT NULL,
  artist TEXT NOT NULL,
  album_artist TEXT NOT NULL,
  genre TEXT NOT NULL,
  track TEXT NOT NULL,
  composer TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Artists (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Scrobbles (
  id INTEGER PRIMARY KEY AUTOINCREMENT
  -- artist
  -- timestamp of scrobble
  -- scrobble source
  -- target(s) <-- enum?
);

COMMIT TRANSACTION;
