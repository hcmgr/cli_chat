use crate::field_lens::{ UNAME_LEN, TOKEN_LEN};

/**
Protocol message: client verifying itself upon connecting with server

NOTE: server-to-client (S2c):
    acceptance: empty packet with unique 'verify-accept' method header
    denial: empty packet with unqique 'verify-deny' method header
*/
pub struct C2sVerify {
    cli_uname: [u8; UNAME_LEN],
    token: [u8; TOKEN_LEN],
}

impl C2sVerify {
    pub fn empty() -> Self {
        C2sVerify {
            cli_uname: [0u8; UNAME_LEN],
            token: [0u8; TOKEN_LEN]
        }
    }

    pub fn new(c_uname: &str, token: &[u8]) -> Self {
        let mut verify = C2sVerify::empty();
        crate::set_uname(&mut verify.cli_uname, c_uname);
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

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != C2sVerify::fixed_size() {
            return None;
        }

        let mut cli_uname = [0u8; UNAME_LEN];
        let mut token = [0u8; TOKEN_LEN];
        cli_uname.copy_from_slice(&bytes[..UNAME_LEN]);
        token.copy_from_slice(&bytes[UNAME_LEN .. C2sVerify::fixed_size()]);

        Some (C2sVerify {
            cli_uname,
            token
        })
    }

    fn fixed_size() -> usize {
        UNAME_LEN + TOKEN_LEN
    }
}