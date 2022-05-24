use std::{
    io::Result,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::{decode, encode};

/// Connect to a `sender()` at the specified address, and communicate result packets to the given
/// mpsc sender. When the Sender is hung up, the loop exits.
pub fn client(addr: SocketAddr, tx: Sender<Vec<u8>>) -> Result<()> {
    let mut sock = TcpStream::connect(addr)?;
    loop {
        let packet = decode(&mut sock)?;

        if tx.send(packet).is_err() {
            break Ok(());
        }
    }
}

/// Send messages from `rx` to the given stream.
pub fn sender(mut stream: TcpStream, rx: Receiver<Vec<u8>>) -> Result<()> {
    while let Ok(packet) = rx.recv() {
        encode(&mut stream, &packet)?;
    }

    Ok(())
}

/// Bind to `addr` and serve messages from `rx`. Creates a thread for each new connection.
pub fn server(addr: SocketAddr, rx: Receiver<Vec<u8>>) -> Result<()> {
    let listener = TcpListener::bind(addr)?;

    // List of connections, shared between distributor and listener
    let guests: Arc<Mutex<Vec<Sender<Vec<u8>>>>> = Arc::new(Mutex::new(vec![]));

    // Distributor thread, doles out messages to guests
    let dist_guests = guests.clone();
    thread::spawn(move || {
        while let Ok(packet) = rx.recv() {
            let mut guests = dist_guests
                .lock()
                .expect("distributor failed to lock guests");

            guests.retain(|tx| tx.send(packet.clone()).is_ok());
        }
    });

    // Listener loop, listens for clients and creates guests 
    loop {
        if let Ok((stream, _)) = listener.accept() {
            let (tx, rx) = channel();
            guests
                .lock()
                .expect("Listener failed to lock guests")
                .push(tx);

            thread::spawn(move || sender(stream, rx));
        }
    }
}
