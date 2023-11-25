use std::io::{self, Read, Write};
use std::net::TcpStream;
use protocol::{chat_message, packet, MessageType};

fn handle_client(ser_message: Vec<u8>, mut stream: TcpStream) -> io::Result<()> {
    stream.write_all(&ser_message)?;
    let mut method = [10u8; 1];
    let mut rest_buffer = [0u8; 1024];
    stream.read(&mut method)?;

    if method[0] != MessageType::ChatMessage as u8 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error reading",
        ));
    }

    let mut packet_length = [0u8; 4];
    stream.read(&mut packet_length)?;

    let bytes_read = stream.read(&mut rest_buffer)?;

    let der_message = chat_message::ChatMessage::deserialize(&rest_buffer[..bytes_read])
        .unwrap_or_else(|| {
            println!("Failed to deserialize");
            chat_message::ChatMessage::empty()
        });
    println!("Chat message received:\n\n{:?}", der_message);
    Ok(())

}

fn main() -> io::Result<()> {
    let method_num = MessageType::ChatMessage as u8;
    let message = chat_message::ChatMessage::new("gramble_guy", "jess", "Morning darling, have a great day!");
    let packet = packet::Packet::new(method_num, message.length(), message.serialize());
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    handle_client(packet.serialize(), stream)?;
    Ok(())
}
