use crate::field_lens::{ UNAME_LEN, TOKEN_LEN };
use rand::Rng;

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

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != UNAME_LEN {
            return None;
        }

        let mut cli_uname = [0u8; UNAME_LEN];
        cli_uname.copy_from_slice(&bytes[..UNAME_LEN]);

        Some (C2sSignup {
            cli_uname
        })
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

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < TOKEN_LEN {
            return None;
        }

        let mut token = [0u8; TOKEN_LEN];
        token.copy_from_slice(&bytes[..TOKEN_LEN]);

        Some (S2cSignup {
            token
        })
    }

    fn generate_token() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let token: [u8; 32] = rng.gen();

        token
    }
}




