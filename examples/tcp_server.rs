use std::{sync::mpsc::channel, thread, time::Duration};

use length_delimit::tcp::server;

fn main() {
    let addr = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6759".to_string())
        .parse()
        .expect("Usage: tcp_sender addr:port");

    let (tx, rx) = channel();

    thread::spawn(move || server(addr, rx));

    let mut i = 0;
    loop {
        let msg = format!("Hello, world! {}", i).into_bytes();
        tx.send(msg).unwrap();
        i += 1;
        thread::sleep(Duration::from_millis(50));
    }
}
