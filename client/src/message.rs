use std::fmt;

mod msg_field_lens {
    pub const UNAME_LEN: usize = 50;
    pub const MSGLEN_LEN: usize = 4;
}

use msg_field_lens::{UNAME_LEN, MSGLEN_LEN};


pub struct Message {
    pub msglen: u32,
    pub send_uname: [u8; UNAME_LEN],
    pub recv_uname: [u8; UNAME_LEN],
    pub message: Vec<u8>,
}

impl Message {
    pub fn empty() -> Self {
        Message {
            msglen: 0,
            send_uname: [0; UNAME_LEN],
            recv_uname: [0; UNAME_LEN],
            message: Vec::new(),
        }
    }

    pub fn new(s_uname: &str, r_uname: &str, msg: &str) -> Self {
        let mut message = Message::empty();
        message.set_send_uname(s_uname);
        message.set_recv_uname(r_uname);
        message.set_message(msg);
        message.msglen = msg.len() as u32;

        message
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

        Some(Message {
            msglen,
            send_uname,
            recv_uname,
            message,
        })
    }

    pub fn set_send_uname(&mut self, new_send_uname: &str) {
        Message::set_uname(&mut self.send_uname, new_send_uname)
    }

    pub fn set_recv_uname(&mut self, new_recv_uname: &str) {
        Message::set_uname(&mut self.recv_uname, new_recv_uname)
    }

    fn set_uname(target: &mut [u8], new_send_uname: &str) {
        if new_send_uname.len() <= UNAME_LEN {
            target[..new_send_uname.len()].copy_from_slice(new_send_uname.as_bytes());
        } else {
            println!("Username can be a maximum of {} characters", UNAME_LEN);
        }
    }

    pub fn set_message(&mut self, message: &str) {
        self.message.extend_from_slice(message.as_bytes());
    }

    fn fixed_size() -> usize {
        return MSGLEN_LEN + (2 * UNAME_LEN);
    }
}

// produce pretty debug output on print by implementing fmt::Debug trait
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Message {{ msglen: {}, send_uname: \"{}\", recv_uname: \"{}\", message: \"{}\" }}",
            self.msglen,
            String::from_utf8_lossy(&self.send_uname).to_string(),
            String::from_utf8_lossy(&self.recv_uname).to_string(),
            String::from_utf8_lossy(&self.message).to_string()
        )
    }
}

