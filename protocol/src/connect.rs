use crate::field_lens::UNAME_LEN;

/**
Protocol message: client requesting to connect with another client
*/
pub struct C2cConnReq {
    req_uname: [u8; UNAME_LEN],
    resp_uname: [u8; UNAME_LEN],
}

/**
Protocol message: client responding to connection request from other client
*/
pub struct C2cConnResp {
    req_uname: [u8; UNAME_LEN],
    resp_uname: [u8; UNAME_LEN],
    response: u8
}