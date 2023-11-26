use crate::field_lens::UNAME_LEN;
use std::fmt;

/**
Protocol message: client requesting to connect with another client
*/
pub struct C2cConnReq {
    req_uname: [u8; UNAME_LEN],
    resp_uname: [u8; UNAME_LEN],
}

impl fmt::Debug for C2cConnReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C2cConnReq {{ req_uname: \"{}\", resp_uname: \"{}\" }}",
            String::from_utf8_lossy(&self.req_uname).to_string(),
            String::from_utf8_lossy(&self.resp_uname).to_string()
        )
    }
}

/**
Protocol message: client responding to connection request from other client
*/
pub struct C2cConnResp {
    req_uname: [u8; UNAME_LEN],
    resp_uname: [u8; UNAME_LEN],
    response: u8
}

impl fmt::Debug for C2cConnResp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C2cConnResp {{ req_uname: \"{}\", resp_uname: \"{}\", response: {} }}",
            String::from_utf8_lossy(&self.req_uname).to_string(),
            String::from_utf8_lossy(&self.resp_uname).to_string(),
            self.response
        )
    }
}