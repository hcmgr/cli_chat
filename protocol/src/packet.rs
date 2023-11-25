use crate::field_lens::{ MSGLEN_LEN, METHOD_LEN };

/**
Wrapper-message for all other protocol messages:

    method - message-type
    length - length of message buffer
    message_buffer - holds at most one protocol message

*/
pub struct Packet {
    pub method: u8,
    pub length: u32,
    pub message_buffer: Vec<u8>,
}

impl Packet {
    pub fn new(meth: u8, len: u32, msg_buf: Vec<u8>) -> Self {
        Packet {
            method: meth,
            length: len,
            message_buffer: msg_buf
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.method);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.message_buffer);
        
        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        let mut method = 0;
        let mut length = 0;
        let mut length_buffer = [0u8; MSGLEN_LEN];
        
        method = bytes[0];
        length_buffer.copy_from_slice(&bytes[METHOD_LEN .. MSGLEN_LEN]);
        length = u32::from_be_bytes(length_buffer);

        if bytes.len() - Packet::fixed_size() != length as usize {
            return None;
        }

        let message_buffer = bytes[Packet::fixed_size()..].to_vec();

        Some(Packet {
            method,
            length,
            message_buffer
        })
    }

    fn fixed_size() -> usize {
        METHOD_LEN + MSGLEN_LEN
    }
}