/**
Module - comms

Handles all communication with the server.

See 'protocol' crate for explanation of the cli_chat protocol
*/

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::error::Error;

use protocol::{Packet, ChatMessage, C2sSignup, S2cSignup, C2sVerify, C2cConnReq, C2cConnResp};
use protocol::{self, field_lens, message_type, errors, shared};
use message_type::{MessageType, method_num_to_message_type};
use crate::storage::{conn_map, storage};

/**
Reads a 'Packet' (see 'protocol' crate) from the server TCP scoket.
*/
fn read_packet(mut stream: TcpStream) -> Result<Packet, Box<dyn Error>> {
    let mut packet_buffer = [0u8; field_lens::MAX_PACKET_LEN];
    let bytes_read = stream.read(&mut packet_buffer)?;
    let packet = Packet::deserialize(&packet_buffer)?;

    Ok(packet)
}

/**
General handler for all protocol message-types
*/
fn handle_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let packet = read_packet(stream)?;
    let message_type = method_num_to_message_type(packet.method);
    match message_type {
        MessageType::C2sVerify => handle_verify_message(packet),
        _ => Ok(())
    };

    Ok(())
}

/**
Handles server response to a C2sVerify message
*/
fn handle_verify_message(packet: Packet) -> Result<(), Box<dyn Error>> {
    let ver_msg = C2sVerify::deserialize(&packet.msg_buffer)?;
    println!("Received verify response:\n\n{:?}", ver_msg);
    Ok(())
}

// TESTS //

pub fn test_verify_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let username = storage::read_username()?;
    let token = storage::read_token()?;
    let mut verify = C2sVerify::new(&protocol::shared::uname_to_string(username), token);
    let mut packet = Packet::new(
        MessageType::C2sVerify as u8, 
        verify.length() as u32, 
        verify.serialize());


    stream.write_all(&packet.serialize());
    handle_message(stream).unwrap();

    Ok(())
}