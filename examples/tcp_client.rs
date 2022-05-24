use std::{sync::mpsc::channel, thread};

use length_delimit::tcp::client;

fn main() {
    let addr = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6759".to_string())
        .parse()
        .expect("Usage: tcp_client addr:port");

    let (tx, rx) = channel();

    thread::spawn(move || client(addr, tx).unwrap());

    while let Ok(msg) = rx.recv() {
        let s = String::from_utf8(msg).expect("Malformed string");
        println!("Server: {}", s);
    }
}
