pub mod chat_message;
pub mod verify;
pub mod signup;
pub mod connect;

/**
Each possible protocol message is assigned a unique method number, used to identify
it in the Protocol.method field.
*/
pub enum MessageType {
    ChatMessage = 0,
    C2sVerify = 1,
    C2sSignup = 2,
    S2cSignup = 3,
    C2cConnReq = 4,
    C2cConnResp = 5
}

// protocol message field lengths
mod field_lens {
    pub const UNAME_LEN: usize = 50;
    pub const MSGLEN_LEN: usize = 4;
    pub const TOKEN_LEN: usize = 32;
    pub const METHOD_LEN: usize = 1;
    pub const ERR_CODE_LEN: usize = 4;
}


// shared code of protocol messages
mod shared {
    pub fn set_uname(target: &mut [u8], new_send_uname: &str) {
        if new_send_uname.len() <= crate::field_lens::UNAME_LEN {
            target[..new_send_uname.len()].copy_from_slice(new_send_uname.as_bytes());
        } else {
            println!(
                "Username can be a maximum of {} characters",
                crate::field_lens::UNAME_LEN
            );
        }
    }
}