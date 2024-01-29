use crate::field_lens::{ UNAME_LEN, TOKEN_LEN, ERR_CODE_LEN};
use crate::status_codes::{self, StatusCode};
use std::fmt;
use crate::errors::LengthError;
use std::error::Error;

/**
Protocol message: client verifying itself upon connecting with server
*/
pub struct VerifyReq {
    pub cli_uname: [u8; UNAME_LEN],
    pub token: [u8; TOKEN_LEN],
}

impl VerifyReq {
    pub fn empty() -> Self {
        VerifyReq {
            cli_uname: [0u8; UNAME_LEN],
            token: [0u8; TOKEN_LEN]
        }
    }

    pub fn new(c_uname: &str, token: [u8; TOKEN_LEN]) -> Self {
        let mut verify = VerifyReq::empty();
        crate::shared::set_uname(&mut verify.cli_uname, c_uname);
        if token.len() != TOKEN_LEN {
            println!("Tokens must be 32 bytes");
            return verify;
        }
        verify.token.copy_from_slice(&token[..TOKEN_LEN]);

        verify
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.cli_uname);
        buffer.extend_from_slice(&self.token);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        println!("bytes len: {:?}, fixed size: {:?}", bytes.len(), VerifyReq::fixed_size());
        if bytes.len() != VerifyReq::fixed_size() {
            return Err(Box::new(LengthError));
        }

        let mut cli_uname = [0u8; UNAME_LEN];
        let mut token = [0u8; TOKEN_LEN];
        cli_uname.copy_from_slice(&bytes[..UNAME_LEN]);
        token.copy_from_slice(&bytes[UNAME_LEN .. VerifyReq::fixed_size()]);

        Ok (VerifyReq {
            cli_uname,
            token
        })
    }

    pub fn length(&self) -> usize {
        VerifyReq::fixed_size()
    }

    fn fixed_size() -> usize {
        UNAME_LEN + TOKEN_LEN
    }
}

impl fmt::Debug for VerifyReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VerifyReq {{ cli_uname: {}, token: {} }}",
            crate::shared::uname_to_string(self.cli_uname),
            crate::shared::token_to_string(self.token)
        )
    }
}

/**
Protocol message: Server response to client verify request
*/
pub struct VerifyResp {
    pub status_code: StatusCode
}

impl VerifyResp {
    pub fn new(code: StatusCode) -> Self {
        VerifyResp {
            status_code: code
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.status_code.clone() as u8);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != VerifyResp::fixed_size() {
            return Err(Box::new(LengthError));
        }

        let status_code = status_codes::decode_status_code(bytes[0]);

        Ok (VerifyResp {
            status_code
        })
    }

    pub fn length(&self) -> usize {
        VerifyResp::fixed_size()
    }

    fn fixed_size() -> usize {
        ERR_CODE_LEN
    }
}

impl fmt::Debug for VerifyResp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VerifyResp {{ status_code: {} }}",
            self.status_code.to_string()
        )
    }
}