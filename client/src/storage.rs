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
use std::io::Write;

pub const ROOT_DIR_NAME: &str = ".cli_chat";
pub const TOKEN_FN: &str = "token";
pub const UNAME_FN: &str = "username";
pub const CONN_DIR_NAME: &str = "connections";

/**
Checks for the existence of the client's '.cli_chat' directory
*/
pub fn dir_exists() -> Option<bool> {
    match home_dir() {
        Some(mut home) => {
            let dir_path = home.join(ROOT_DIR_NAME);
            return Some(dir_path.exists() && dir_path.is_dir());
        }
        None => {
            eprintln!("Unable to determine user's home directory.");
            return None;
        }
    }
}

/**
Creates a fresh '.cli_chat' directory for a new user.
*/
pub fn create_cli_chat_dir(uname: [u8; 50], token: [u8; 32]) -> Option<PathBuf> {
    match home_dir() {
        Some(mut home) => {
            let dir_path = home.join(ROOT_DIR_NAME);

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
        None => {
            eprintln!("Unable to determine the user's home directory.");
            return None;
        }
    }
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
