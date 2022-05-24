use std::io::{Read, Result, Write};
pub mod tcp;

// TODO: BufWriter for encoder, flush on each message?

/// Write a packet containing up to 4 Gibibytes
/// (2^32 bytes) of data, preceded by a 4 byte length header
pub fn encode<W: Write>(mut w: W, packet: &[u8]) -> Result<()> {
    // Write header
    let length: u32 = packet.len().try_into().expect("Max packet size exceeded");

    w.write_all(&length.to_le_bytes())?;

    // Write data
    w.write_all(packet)
}

/// Decode a packet containing a 4 byte length
/// header and then up to 4 Gibibytes of data.
pub fn decode<R: Read>(mut r: R) -> Result<Vec<u8>> {
    // Read header
    let mut header: [u8; 4] = [0; 4];
    r.read_exact(&mut header)?;
    let length: usize = u32::from_le_bytes(header)
        .try_into()
        .expect("Packet too long");

    // Read data
    let mut data = vec![0; length as usize];
    r.read_exact(&mut data)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip() {
        let message = "Macaroni in a pot".as_bytes();

        let mut buffer = vec![];

        encode(&mut buffer, message).unwrap();

        let packet = decode(&*buffer).unwrap();

        assert_eq!(&packet, message);
    }

    #[test]
    #[should_panic]
    fn missing_data() {
        let buffer = Cursor::new(vec![]);
        decode(buffer).unwrap();
    }
}
