mod message;
use message::Message;

use std::io::{self, Read, Write};
use std::net::TcpStream;

/**
Represents the main wrapper for all protocol messages. 
*/
pub struct Packet {
    pub method: u8,
    pub length: u32,
    pub buffer: Vec<u8>,
}

fn handle_client(ser_message: Vec<u8>, mut stream: TcpStream) -> io::Result<()> {
    stream.write_all(&ser_message)?;
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let der_message = Message::deserialize(&buffer[..bytes_read])
        .unwrap_or_else(|| {
            println!("Failed to deserialize");
            Message::empty()
        });
    println!("{:?}", der_message);
    Ok(())
}

fn main() -> io::Result<()> {
    let message = Message::new("hcmgr", "jess", "Morning darling");
    let ser_message: Vec<u8> = message.serialize();
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    handle_client(ser_message, stream)?;
    Ok(())
}
