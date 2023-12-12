// Bring in and re-export all protocol message types
pub mod packet;
pub mod chat_message;
pub mod verify;
pub mod signup;
pub mod connect;
pub use packet::Packet;
pub use chat_message::ChatMessage;
pub use verify::C2sVerify;
pub use signup::{ C2sSignup, S2cSignup};
pub use connect::{ C2cConnReq, C2cConnResp};

/**
Each possible protocol message is assigned a unique method number, used to identify
it in the Protocol.method field.
*/
pub mod message_type {
    pub enum MessageType {
        ChatMessage = 0,
        C2sVerify = 1,
        C2sSignup = 2,
        S2cSignup = 3,
        C2cConnReq = 4,
        C2cConnResp = 5,
        Invalid
    }

    pub fn method_num_to_message_type(index: u8) -> MessageType {
        let mt = match index {
            0 => MessageType::ChatMessage,
            1 => MessageType::C2sVerify,
            2 => MessageType::C2sSignup,
            3 => MessageType::S2cSignup, 
            4 => MessageType::C2cConnReq,
            5 => MessageType::C2cConnResp,
            _ => MessageType::Invalid
        };
        mt
    }
}

// protocol message field lengths
pub mod field_lens {
    pub const UNAME_LEN: usize = 50;
    pub const MSGLEN_LEN: usize = 4;
    pub const TOKEN_LEN: usize = 32;
    pub const METHOD_LEN: usize = 1;
    pub const ERR_CODE_LEN: usize = 4;
    pub const MAX_PACKET_LEN: usize = 1024;
}

// defines all errors used by protocol
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

// miscellaneous helper functions used by all protocol code
pub mod shared {
    use rand::Rng;

    // sets username field of an arbitrary protocol message 
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

    pub fn generate_token() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let token: [u8; 32] = rng.gen();

        token
    }

    pub fn uname_to_string(uname: [u8; crate::field_lens::UNAME_LEN]) -> String {
        String::from_utf8_lossy(&uname).to_string()
    }

    pub fn token_to_string(token: [u8; crate::field_lens::TOKEN_LEN]) -> String {
        let mut result = String::from("0x");
        for byte in token.iter() {
            result.push_str(&format!("{:02x}", byte));
        }

        result
    }
}