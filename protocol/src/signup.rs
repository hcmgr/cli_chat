use std::fmt;
use std::error::Error;

use crate::field_lens::{ UNAME_LEN, TOKEN_LEN, ERR_CODE_LEN };
use crate::status_codes::{ self, StatusCode };
use crate::errors::LengthError;

/**
Protocol message: client attempting to sign up with a given username
*/
pub struct SignupReq {
   cli_uname: [u8; UNAME_LEN]
}

impl SignupReq {
    pub fn empty() -> Self {
        SignupReq {
            cli_uname: [0u8; UNAME_LEN]
        }
    }

    pub fn new(new_uname: &str) -> Self {
        let mut signup = SignupReq::empty();
        crate::shared::set_uname(&mut signup.cli_uname, new_uname);

        signup
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.cli_uname);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != SignupReq::fixed_size() {
            return Err(Box::new(LengthError));
        }

        let mut cli_uname = [0u8; UNAME_LEN];
        cli_uname.copy_from_slice(&bytes[..UNAME_LEN]);

        Ok (SignupReq {
            cli_uname
        })
    }

    pub fn length(&self) -> usize {
        SignupReq::fixed_size()
    }

    fn fixed_size() -> usize {
        UNAME_LEN
    }
}

impl fmt::Debug for SignupReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SignupReq {{ cli_uname: \"{}\" }}",
            String::from_utf8_lossy(&self.cli_uname).to_string()
        )
    }
}

/**
Protocol message: server responding to successful signup attempt with unique token
*/
pub struct SignupResp {
    status_code: StatusCode,
    token: [u8; TOKEN_LEN]
}

impl SignupResp {
    pub fn new(status_code: StatusCode) -> Self {
        SignupResp {
            status_code,
            token: crate::shared::generate_token()
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let status_code = *(&self.status_code) as u8;

        buffer.push(status_code);
        buffer.extend_from_slice(&self.token);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != SignupResp::fixed_size() {
            return Err(Box::new(LengthError));
        }

        let status_code = status_codes::decode_status_code(bytes[0]);
        let mut token = [0u8; TOKEN_LEN];
        token.copy_from_slice(&bytes[ERR_CODE_LEN .. (ERR_CODE_LEN + TOKEN_LEN)]);

        Ok (SignupResp {
            status_code,
            token
        })
    }

    pub fn length(&self) -> usize {
        SignupResp::fixed_size()
    }

    fn fixed_size() -> usize {
        ERR_CODE_LEN + TOKEN_LEN
    } 
}

impl fmt::Debug for SignupResp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SignupResp {{ status_code: {}, token: {} }}",
            self.status_code.to_string(),
            String::from_utf8_lossy(&self.token).to_string()
        )
    }
}


