-- Your SQL goes here

CREATE TABLE Songs (
       id INTEGER PRIMARY KEY,
       title TEXT NOT NULL,
       album TEXT NOT NULL,
       artist TEXT NOT NULL,
       album_artist TEXT NOT NULL,
       genre TEXT NOT NULL,
       track TEXT NOT NULL,
       composer TEXT NOT NULL,
       ts_created TEXT NOT NULL,
       ts_inputted TEXT NOT NULL,
       rscr_source TEXT NOT NULL
)
