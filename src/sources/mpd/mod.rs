mod utils;

use mpd::idle::Idle;
use mpd::idle::Subsystem::Player;
use mpd::Client;
use std::net::TcpStream;
use std::process::exit;
use utils::get_tag;

use super::common::Song;

fn get_mpd_conn(addr: &str) -> Client<TcpStream> {
    match Client::connect(addr) {
        Ok(x) => x,
        Err(e) => {
            println!("Error found while creating client...");
            println!("Error: {}", e);
            println!("Cannot proceed, bailing!");
            exit(1);
        }
    }
}
