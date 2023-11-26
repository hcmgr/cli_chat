use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::error::Error;

use protocol::{Packet, ChatMessage, C2sSignup, S2cSignup, C2sVerify, C2cConnReq, C2cConnResp};
use protocol::{self, field_lens, message_type, errors, };
use message_type::{MessageType, method_num_to_message_type};

fn read_packet(mut stream: TcpStream) -> Result<Packet, Box<dyn Error>> {
    let mut packet_buffer = [0u8; field_lens::MAX_PACKET_LEN];
    let bytes_read = stream.read(&mut packet_buffer)?;
    let packet = Packet::deserialize(&packet_buffer)?;

    Ok(packet)
}

fn handle_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let packet = read_packet(stream)?;
    let message_type = method_num_to_message_type(packet.method);
    match message_type {
        MessageType::ChatMessage => handle_chat_message(packet),
        MessageType::C2sVerify => handle_verify_message(packet),
        _ => Ok(())
    };

    Ok(())
}

fn handle_chat_message(packet: Packet) -> Result<(), Box<dyn Error>> {
    let chat_msg = ChatMessage::deserialize(&packet.msg_buffer)?;
    println!("{:?}", chat_msg);

    Ok(())
}

fn handle_verify_message(packet: Packet) -> Result<(), Box<dyn Error>> {
    let ver_msg = C2sVerify::deserialize(&packet.msg_buffer)?;
    println!("{:?}", ver_msg);

    Ok(())
}

fn test_chat_message(mut stream: TcpStream) -> io::Result<()> {
    let method_num = MessageType::ChatMessage as u8;
    let message = ChatMessage::new("gramble_guy", "jess", "Morning darling, have a great day!");
    let mut packet = Packet::new(method_num, message.length() as u32, message.serialize());
    stream.write_all(&packet.serialize())?;
    handle_message(stream).unwrap();

    Ok(())
}

fn test_c2sVerify(mut stream: TcpStream) -> io::Result<()> {
    let method_num = MessageType::C2sVerify as u8;
    let verify = C2sVerify::new("hcmgr", S2cSignup::generate_token());
    let mut packet = Packet::new(method_num, verify.length() as u32, verify.serialize());
    stream.write_all(&packet.serialize())?;
    handle_message(stream).unwrap();

    Ok(())
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // test_chat_message(stream);
    test_c2sVerify(stream);

    Ok(())
}
