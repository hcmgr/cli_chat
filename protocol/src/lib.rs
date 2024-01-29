/**
-----------------
Cli-chat protocol
-----------------

Protocol units are 'messages', each of which is sent wrapped in a Packet (see packet.rs).

The message types are:

    ChatMessage:
        - client sending a chat message to a mutual connection

    VerifyReq/VerifyResp:
        - sent at the start of every cli-chat session
        - user authenticates themselves by sending their username and previously-assigned PAT token
    
    SignupReq/SignupResp:
        - sends new user's chosen username to server
        - on success, server sends back PAT token
    
    C2cConnReq/C2cConnResp:
        - user requests to 'connect' with another user (based on username)
        - server relays this on to target user
        - on accept, clients add each other to their respective 'connections-list' stores,
          and can now send messages to each other
*/

// Bring in and re-export all protocol message types
pub mod packet;
pub mod chat_message;
pub mod verify;
pub mod signup;
pub mod connect;
pub use packet::Packet;
pub use chat_message::ChatMessage;
pub use verify::{ VerifyReq, VerifyResp };
pub use signup::{ SignupReq, SignupResp };
pub use connect::{ C2cConnReq, C2cConnResp};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::error::Error;

// protocol message types
pub mod message_types {
    pub enum MessageType {
        ChatMessage,
        VerifyReq,
        VerifyResp,
        SignupReq,
        SignupResp,
        C2cConnReq,
        C2cConnResp,
        Invalid
    }

    pub fn method_num_to_message_type(index: u8) -> MessageType {
        match index {
            0 => MessageType::ChatMessage,
            1 => MessageType::VerifyReq,
            2 => MessageType::VerifyResp,
            3 => MessageType::SignupReq, 
            4 => MessageType::SignupResp,
            5 => MessageType::C2cConnReq,
            6 => MessageType::C2cConnResp,
            _ => MessageType::Invalid
        }
    }
}

// protocol status codes
pub mod status_codes {
    #[derive(Copy, Clone)]
    pub enum StatusCode {
        Success,
        Failure,
        Invalid
    }

    pub fn decode_status_code(status_code: u8) -> StatusCode {
        match status_code {
            0 => StatusCode::Success,
            1 => StatusCode::Failure,
            _ => StatusCode::Invalid
        }
    }

    impl ToString for StatusCode {
        fn to_string(&self) -> String {
            match self {
                StatusCode::Success => String::from("Success"),
                StatusCode::Failure => String::from("Failure"),
                StatusCode::Invalid => String::from("Invalid"),
            }
        }
    }
}

// protocol message field lengths
pub mod field_lens {
    pub const UNAME_LEN: usize = 50;
    pub const MSGLEN_LEN: usize = 4;
    pub const TOKEN_LEN: usize = 32;
    pub const METHOD_LEN: usize = 1;
    pub const ERR_CODE_LEN: usize = 1;
    pub const MAX_PACKET_LEN: usize = 1024;
}

// custom errors
pub mod errors {
    use std::fmt;
    use std::error::Error;

    #[derive(Debug)]
    pub struct LengthError; 

    impl fmt::Display for LengthError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "incorrect message/packet length specified")
        }
    }

    impl Error for LengthError {}
}

// Reads a 'Packet' (see packet.rs) from the given server TCP scoket
pub fn read_packet(mut stream: TcpStream) -> Result<Packet, Box<dyn Error>> {
    let mut packet_buffer = [0u8; field_lens::MAX_PACKET_LEN];
    let bytes_read = stream.read(&mut packet_buffer)?;
    let packet = Packet::deserialize(&packet_buffer)?;

    Ok(packet)
}

// miscellaneous helper functions used by all protocol code
pub mod shared {
    use rand::Rng;
    use crate::field_lens;

    // Sets username field of an arbitrary protocol message 
    pub fn set_uname(target: &mut [u8], new_send_uname: &str) {
        if new_send_uname.len() <= field_lens::UNAME_LEN {
            target[..new_send_uname.len()].copy_from_slice(new_send_uname.as_bytes());
        } else {
            println!(
                "Username can be a maximum of {} characters",
                crate::field_lens::UNAME_LEN
            );
        }
    }

    // 32-byte token generator
    pub fn generate_token() -> [u8; field_lens::TOKEN_LEN] {
        let mut rng = rand::thread_rng();
        let token: [u8; field_lens::TOKEN_LEN] = rng.gen();

        token
    }

    /**
    Converts a username from its byte-rep to string-rep

    NOTE: conversion to string form truncates null bytes
    */
    pub fn uname_to_string(uname: [u8; crate::field_lens::UNAME_LEN]) -> String {
        // Find the position of the first null byte (0) in the buffer
        let null_byte_position = uname.iter().position(|&byte| byte == 0);
    
        // Create a slice of the buffer up to the null byte position (if found)
        let uname_slice = match null_byte_position {
            Some(pos) => &uname[..pos],
            None => &uname,
        };
    
        // Convert the slice to a String
        String::from_utf8_lossy(uname_slice).to_string()
    }

    // Converts a token from its byte-rep to string-rep
    pub fn token_to_string(token: [u8; crate::field_lens::TOKEN_LEN]) -> String {
        let mut result = String::from("0x");
        for byte in token.iter() {
            result.push_str(&format!("{:02x}", byte));
        }

        result
    }
}