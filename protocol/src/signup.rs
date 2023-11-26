use crate::field_lens::{ UNAME_LEN, TOKEN_LEN };
use rand::Rng;
use std::fmt;
use crate::errors::LengthError;
use std::error::Error;

/**
Protocol message: client attempting to sign up with a given username
*/
pub struct C2sSignup {
   cli_uname: [u8; UNAME_LEN]
}

impl C2sSignup {
    pub fn empty() -> Self {
        C2sSignup {
            cli_uname: [0u8; UNAME_LEN]
        }
    }

    pub fn new(new_uname: &str) -> Self {
        let mut signup = C2sSignup::empty();
        crate::shared::set_uname(&mut signup.cli_uname, new_uname);

        signup
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.cli_uname);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != UNAME_LEN {
            return Err(Box::new(LengthError));
        }

        let mut cli_uname = [0u8; UNAME_LEN];
        cli_uname.copy_from_slice(&bytes[..UNAME_LEN]);

        Ok (C2sSignup {
            cli_uname
        })
    }

    pub fn length(&self) -> usize {
        return UNAME_LEN;
    }
}

impl fmt::Debug for C2sSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C2sSignup {{ cli_uname: \"{}\" }}",
            String::from_utf8_lossy(&self.cli_uname).to_string()
        )
    }
}

/**
Protocol message: server responding to successful signup attempt with unique token
*/
pub struct S2cSignup {
    token: [u8; TOKEN_LEN]
}

impl S2cSignup {
    pub fn new() -> Self {
        S2cSignup {
            token: S2cSignup::generate_token(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.token);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != TOKEN_LEN {
            return Err(Box::new(LengthError));
        }

        let mut token = [0u8; TOKEN_LEN];
        token.copy_from_slice(&bytes[..TOKEN_LEN]);

        Ok (S2cSignup {
            token
        })
    }

    pub fn generate_token() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let token: [u8; 32] = rng.gen();

        token
    }

    pub fn length(&self) -> usize {
        return TOKEN_LEN;
    }
}

impl fmt::Debug for S2cSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S2cSignup {{ token: \"{}\" }}",
            String::from_utf8_lossy(&self.token).to_string()
        )
    }
}


