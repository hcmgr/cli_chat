pub mod chat_message;
pub mod verify;
pub mod signup;
pub mod connect;

mod field_lens {
    pub const UNAME_LEN: usize = 50;
    pub const MSGLEN_LEN: usize = 4;
    pub const TOKEN_LEN: usize = 32;
    pub const METHOD_LEN: usize = 1;
    pub const ERR_CODE_LEN: usize = 4;
}

fn set_uname(target: &mut [u8], new_send_uname: &str) {
    if new_send_uname.len() <= field_lens::UNAME_LEN {
        target[..new_send_uname.len()].copy_from_slice(new_send_uname.as_bytes());
    } else {
        println!("Username can be a maximum of {} characters", field_lens::UNAME_LEN);
    }
}