use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("client connected");
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    println!("read {} bytes from client", bytes_read);
    stream.write_all(&buffer[..bytes_read]).unwrap();
    Ok(())
}
