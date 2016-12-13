extern crate mpd;

use mpd::Client;

fn main() {
    println!("*** LOADING RSCRIBBLE VER {} ***",
             env!("CARGO_PKG_VERSION"));

    let mpd_host = "127.0.0.1:6600";

    let mut conn = Client::connect(mpd_host).unwrap();

    let _ = conn.volume(75);

    let song = conn.currentsong().unwrap().unwrap();;
    let tags = song.tags;
    let song_title = song.title.unwrap();

    match tags.get("Artist") {
        Some(x) => {
            println!("Artist: {}\nTitle: {}", x, song_title)
        }
        None => println!("Could not find the artist."),
    }
}
