use std::io::{self};
use std::net::TcpStream;

mod storage;
mod comms;
mod helpers;

fn main() -> io::Result<()> {
    if !storage::storage_methods::dir_exists() {
        let username = "snacky";
        let mut username_bytes = [0u8; 50];
        username_bytes[..username.len()].copy_from_slice(username.as_bytes());
        storage::storage_methods::create_cli_chat_dir(username_bytes, protocol::shared::generate_token());
    }

    // let mut stream = TcpStream::connect("127.0.0.1:8081")?;
    // comms::test_verify_message(stream);
    // storage::test_get_conn_map();
    // storage::test_add_conn();
    // let username = storage::read_username().unwrap();
    // let digest = helpers::sha256_from_bytes(username.to_vec());
    // println!("{}", digest);
    // storage::storage_methods::test_singelton_map();
    storage::storage_methods::test_write_read_message();
    Ok(())
}
