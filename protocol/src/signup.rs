struct C2sSignup {
   cli_uname: [0u8; 50];
}

impl C2sSignup {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.cli_uname);

        buffer
    }
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 50 {
            return None;
        }

        let mut cli_uname = [0u8; 50];
        cli_uname.copy_from_slice(&bytes[..50]);
        Some (C2sSignup {
            cli_uname
        })
    }
}

struct S2cSignup {
    token: [0u8; 32];
}




