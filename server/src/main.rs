use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

use protocol::{Packet, ChatMessage, SignupReq, SignupResp, VerifyReq, VerifyResp, C2cConnReq, C2cConnResp};
use protocol::{self, field_lens, message_types, errors, shared, status_codes};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("client connected");
    let mut buffer = [0u8; 82];
    let bytes_read = stream.read(&mut buffer)?;
    println!("read {} bytes from client", bytes_read);
    let verify_req = VerifyReq::deserialize(&buffer);
    println!("{:?}", verify_req);
    // stream.write_all(&buffer[..bytes_read]).unwrap();
    Ok(())
}
