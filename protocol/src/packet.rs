use crate::field_lens::{ MSGLEN_LEN, METHOD_LEN };
use std::error::Error;
use crate::errors::LengthError;

/**
MTU (maximum transmission unit) of the protocol. Acts as a wrapper for all protocol messages.
*/
pub struct Packet {
    pub method: u8,
    pub msg_length: u32,
    pub msg_buffer: Vec<u8>,
}

impl Packet {
    pub fn new(meth: u8, len: u32, msg_buf: Vec<u8>) -> Self {
        Packet {
            method: meth,
            msg_length: len,
            msg_buffer: msg_buf
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.method);
        buffer.extend_from_slice(&self.msg_length.to_be_bytes());
        buffer.extend_from_slice(&self.msg_buffer);
        
        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut method = 0;
        let mut msg_length = 0;
        let mut length_buffer = [0u8; MSGLEN_LEN];
        
        method = bytes[0];
        length_buffer.copy_from_slice(&bytes[METHOD_LEN .. METHOD_LEN + MSGLEN_LEN]);
        msg_length = u32::from_be_bytes(length_buffer);

        let msg_buffer = bytes[Packet::fixed_size() .. (Packet::fixed_size() + msg_length as usize)].to_vec();

        Ok(Packet {
            method,
            msg_length,
            msg_buffer,
        })
    }

    pub fn length(&self) -> usize {
        return Packet::fixed_size();
    }

    fn fixed_size() -> usize {
        METHOD_LEN + MSGLEN_LEN
    }
}