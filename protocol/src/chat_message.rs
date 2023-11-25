use std::fmt;
use crate::field_lens::{ UNAME_LEN, MSGLEN_LEN, TOKEN_LEN, METHOD_LEN, ERR_CODE_LEN };

/**
Protocol message: chat message between clients (main 'unit' of the protocol)
*/
pub struct ChatMessage {
    pub msglen: u32,
    pub send_uname: [u8; UNAME_LEN],
    pub recv_uname: [u8; UNAME_LEN],
    pub message: Vec<u8>,
}

impl ChatMessage {
    pub fn empty() -> Self {
        ChatMessage {
            msglen: 0,
            send_uname: [0; UNAME_LEN],
            recv_uname: [0; UNAME_LEN],
            message: Vec::new(),
        }
    }

    pub fn new(new_send_uname: &str, new_recv_uname: &str, msg: &str) -> Self {
        let mut chat_message = ChatMessage::empty();
        crate::shared::set_uname(&mut chat_message.send_uname, new_send_uname);
        crate::shared::set_uname(&mut chat_message.recv_uname, new_recv_uname);
        chat_message.message.extend_from_slice(msg.as_bytes());
        chat_message.msglen = msg.len() as u32;

        chat_message
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&self.msglen.to_be_bytes());
        buffer.extend_from_slice(&self.send_uname);
        buffer.extend_from_slice(&self.recv_uname);
        buffer.extend_from_slice(&self.message);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::fixed_size() {
            return None;
        }

        let (fixed_size, variable_size) = bytes.split_at(Self::fixed_size());

        let mut send_uname = [0u8; UNAME_LEN];
        let mut recv_uname = [0u8; UNAME_LEN];
        let msglen = u32::from_be_bytes(fixed_size[..MSGLEN_LEN].try_into().ok()?);
        
        send_uname.copy_from_slice(&fixed_size[MSGLEN_LEN..(MSGLEN_LEN + UNAME_LEN)]);
        recv_uname.copy_from_slice(&fixed_size[(MSGLEN_LEN + UNAME_LEN)..]);

        if variable_size.len() < msglen as usize {
            return None;
        }

        let message = variable_size[..msglen as usize].to_vec();

        Some(ChatMessage {
            msglen,
            send_uname,
            recv_uname,
            message,
        })
    }

    fn fixed_size() -> usize {
        return MSGLEN_LEN + (2 * UNAME_LEN);
    }
}

// produce pretty debug output on print by implementing fmt::Debug trait
impl fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChatMessage {{ msglen: {}, send_uname: \"{}\", recv_uname: \"{}\", message: \"{}\" }}",
            self.msglen,
            String::from_utf8_lossy(&self.send_uname).to_string(),
            String::from_utf8_lossy(&self.recv_uname).to_string(),
            String::from_utf8_lossy(&self.message).to_string()
        )
    }
}

