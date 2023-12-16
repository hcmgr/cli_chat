use std::fmt;
use std::error::Error;
use crate::field_lens::{ UNAME_LEN, MSGLEN_LEN, TOKEN_LEN, METHOD_LEN, ERR_CODE_LEN };
use crate::errors::LengthError;

/**
Protocol message: chat message between clients (main 'unit' of the protocol)
*/
pub struct ChatMessage {
    pub msg_length: u32,
    pub send_uname: [u8; UNAME_LEN],
    pub recv_uname: [u8; UNAME_LEN],
    pub msg_buffer: Vec<u8>,
}

impl ChatMessage {
    pub fn empty() -> Self {
        ChatMessage {
            msg_length: 0,
            send_uname: [0; UNAME_LEN],
            recv_uname: [0; UNAME_LEN],
            msg_buffer: Vec::new(),
        }
    }

    pub fn new(new_send_uname: &str, new_recv_uname: &str, msg: &str) -> Self {
        let mut chat_message = ChatMessage::empty();
        crate::shared::set_uname(&mut chat_message.send_uname, new_send_uname);
        crate::shared::set_uname(&mut chat_message.recv_uname, new_recv_uname);
        chat_message.msg_buffer.extend_from_slice(msg.as_bytes());
        chat_message.msg_length = msg.len() as u32;

        chat_message
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&self.msg_length.to_be_bytes());
        buffer.extend_from_slice(&self.send_uname);
        buffer.extend_from_slice(&self.recv_uname);
        buffer.extend_from_slice(&self.msg_buffer);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<ChatMessage, Box<dyn Error>> {
        if bytes.len() < Self::fixed_size() {
            return Err(Box::new(LengthError));
        }

        let (fixed_size, variable_size) = bytes.split_at(Self::fixed_size());

        let mut send_uname = [0u8; UNAME_LEN];
        let mut recv_uname = [0u8; UNAME_LEN];
        let msg_length = u32::from_be_bytes(fixed_size[..MSGLEN_LEN].try_into().unwrap());
        
        send_uname.copy_from_slice(&fixed_size[MSGLEN_LEN..(MSGLEN_LEN + UNAME_LEN)]);
        recv_uname.copy_from_slice(&fixed_size[(MSGLEN_LEN + UNAME_LEN)..]);

        if variable_size.len() < msg_length as usize {
            return Err(Box::new(LengthError));
        }

        let message = variable_size[..msg_length as usize].to_vec();

        Ok(ChatMessage {
            msg_length: msg_length,
            send_uname: send_uname,
            recv_uname: recv_uname,
            msg_buffer: message
        })
    }

    pub fn length(&self) -> usize {
        ChatMessage::fixed_size() + self.msg_length as usize
    }

    pub fn fixed_size() -> usize {
        MSGLEN_LEN + (2 * UNAME_LEN)
    }
}

// produce pretty debug output on print by implementing fmt::Debug trait
impl fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChatMessage {{ msglen: {}, send_uname: \"{}\", recv_uname: \"{}\", message: \"{}\" }}",
            self.msg_length,
            String::from_utf8_lossy(&self.send_uname).to_string(),
            String::from_utf8_lossy(&self.recv_uname).to_string(),
            String::from_utf8_lossy(&self.msg_buffer).to_string()
        )
    }
}

