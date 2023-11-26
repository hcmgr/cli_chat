/**
Client data is stored locally in the cli_chat directory, which
itself is created in the client's system root directory (~).

The directory structure looks like:
    cli_chat
        | username
        | token
        | connections
            | conn1
                | metadata
                | msgs
            | conn2
                | metadata
                | msgs
        ...

where:
    - connX represents a connection (friend) AND;
    - connX/msgs represents the messages of the given connection
    - connX/metadata represents the metadata of that connection 
*/

use std::fs::{self, File};
use home::home_dir;
use std::path::PathBuf;
use std::io::{self, Read, Write};
use protocol::field_lens;

pub const ROOT_DIR_NAME: &str = ".cli_chat";
pub const TOKEN_FN: &str = "token";
pub const UNAME_FN: &str = "username";
pub const CONN_DIR_NAME: &str = "connections";

/**
Returns path of cli_chat root directory
*/
fn get_root_dir() -> Option<PathBuf> {
    match home_dir() {
        Some (mut home) => {
            Some(home.join(ROOT_DIR_NAME))
        }
        None => {
            eprintln!("Unable to determine user's home directory.");
            return None;
        }
    }
}

/**
Checks for the existence of the client's '.cli_chat' directory
*/
pub fn dir_exists() -> bool {
    let dir_path = get_root_dir().unwrap();
    return dir_path.exists() && dir_path.is_dir();
}

/**
Creates a fresh '.cli_chat' directory for a new user.
*/
pub fn create_cli_chat_dir(uname: [u8; field_lens::UNAME_LEN], 
    token: [u8; field_lens::TOKEN_LEN]) -> Option<PathBuf> {

    let dir_path = get_root_dir()?;
    // create cli_chat directory
    if let Err(err) = fs::create_dir(&dir_path) {
        eprintln!("Error creating {} directory: {}", ROOT_DIR_NAME, err);
        return None;
    }

    // create token file
    let mut token_file = create_file(dir_path.clone(), TOKEN_FN);
    if token_file.is_none() {
        return None;
    }
    token_file.unwrap().write(&token);

    // create username file
    let mut uname_file = create_file(dir_path.clone(), UNAME_FN);
    if uname_file.is_none() {
        return None;
    }
    uname_file.unwrap().write(&uname);

    // create connections directory
    let conn_path = dir_path.join(CONN_DIR_NAME);
    if let Err(err) = fs::create_dir(&conn_path) {
        eprintln!("Error creating {} directory: {}", ROOT_DIR_NAME, err);
        return None;
    }

    return Some(dir_path);
}

fn create_file(base_path: PathBuf, name: &str) -> Option<File> {
    let path = base_path.join(name);
    match fs::File::create(&path) {
        Err(err) => {
            eprintln!("Error creating {} file: {}", name, err);
            return None;
        }, 
        Ok(file) => Some(file)
    }
}

/**
Methods:
- read username
- read token
- read all messages from one connection
*/
pub fn read_username() -> io::Result<[u8; field_lens::UNAME_LEN]> {
    let cli_root = get_root_dir().unwrap();
    let uname_path = cli_root.join(UNAME_FN);
    let mut uname_file = File::open(&uname_path)?;
    let mut uname_buffer = [0u8; field_lens::UNAME_LEN];
    uname_file.read(&mut uname_buffer);

    Ok(uname_buffer)
}

pub fn read_token() -> io::Result<[u8; field_lens::TOKEN_LEN]> {
    let cli_root = get_root_dir().unwrap();
    let token_path = cli_root.join(TOKEN_FN);
    let mut token_file = File::open(&token_path)?;
    let mut token_buffer = [0u8; field_lens::TOKEN_LEN];
    token_file.read(&mut token_buffer);

    Ok(token_buffer)
}