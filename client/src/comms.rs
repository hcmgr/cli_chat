/**
Module - comms

Handles all communication with the server.

See 'protocol' crate for explanation of the cli_chat protocol
*/

use std::io::{Read, Write};
use std::net::TcpStream;
use std::error::Error;

use protocol::{Packet, ChatMessage, SignupReq, SignupResp, VerifyReq, VerifyResp, C2cConnReq, C2cConnResp};
use protocol::{self, field_lens, message_types, errors, shared, status_codes};
use message_types::{MessageType, method_num_to_message_type};

/**
General handler for all protocol message-types

TODO: implement for rest of message types
*/
fn handle_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let packet = protocol::read_packet(stream)?;
    let message_type = method_num_to_message_type(packet.method);
    match message_type {
        MessageType::VerifyReq => handle_verify_message(packet),
        _ => Ok(())
    };

    Ok(())
}

/**
Handles server response to a VerifyReq message
*/
fn handle_verify_message(packet: Packet) -> Result<(), Box<dyn Error>> {
    let ver_msg = VerifyReq::deserialize(&packet.msg_buffer)?;
    println!("Received verify response:\n\n{:?}", ver_msg);
    Ok(())
}

// TESTS //

pub mod tests {
    use super::*;
    use crate::storage::storage;

    pub fn test_verify_req(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let username = storage::read_username()?;
        let token = storage::read_token()?;
        let mut verify_req = VerifyReq::new(&protocol::shared::uname_to_string(username), token);
        let mut packet = Packet::new(
            MessageType::VerifyReq as u8, 
            verify_req.length() as u32, 
            verify_req.serialize());

        // stream.write_all(&packet.serialize());
        stream.write_all(&verify_req.serialize());
        // handle_message(stream).unwrap();

        Ok(())
    }

    pub fn test_verify_resp() {
        let verify_resp = VerifyResp::new(status_codes::StatusCode::Success);
        println!("{:?}", verify_resp);
    }
}
