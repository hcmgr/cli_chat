/**
Module - storage

Handles storing and retreiving of locally stored user data.

Client data is stored locally in the cli_chat directory, which
itself is created in the client's system root directory (~).

The directory structure looks like:
    cli_chat
        | username
        | token
        | connection-list
        | connections
            conn1
            conn2
            ...

connection-list:
    - stores all user connections
    - the format is as follows:
        conn_uname_1 (50 bytes)
        conn_uname_2 (50 bytes)
        ...

    - NOTE: make hashmap of this list for quick access

connX:
    - stores all messages and metadata for a given connection (X)
    - the format is as follows:
        {Header}:
            conn_uname (50 bytes)
            \n
        {Body}:
            {Message 1}
            msg_length (4 bytes)
            send_uname (50 bytes)
            msg_buffer (variable)
            \n
            {Message 2}
            ...
*/

use std::fs::{self, File, OpenOptions};
use home::home_dir;
use std::path::PathBuf;
use std::io::{self, Read, Write, BufRead, Error};
use std::collections::HashMap;
use protocol::field_lens;

pub const ROOT_DIR_NAME: &str = ".cli_chat";
pub const TOKEN_FN: &str = "token";
pub const UNAME_FN: &str = "username";
pub const CONN_LIST_FN: &str = "connections-list";
pub const CONN_FILE_PREFIX: &str = "conn";
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
    let mut token_file = create_cli_chat_file(dir_path.clone(), TOKEN_FN);
    if token_file.is_none() {
        println!("Error creating token file");
        return None;
    }
    token_file.unwrap().write(&token);

    // create username file
    let mut uname_file = create_cli_chat_file(dir_path.clone(), UNAME_FN);
    if uname_file.is_none() {
        println!("Error creating username file");
        return None;
    }
    uname_file.unwrap().write(&uname);

    // create connections-list file
    let mut conn_list_file = create_cli_chat_file(dir_path.clone(), CONN_LIST_FN);
    if conn_list_file.is_none() {
        println!("Error creating connections-list file");
        return None;
    }

    // create connections directory
    let conn_path = dir_path.join(CONN_DIR_NAME);
    if let Err(err) = fs::create_dir(&conn_path) {
        eprintln!("Error creating {} directory: {}", ROOT_DIR_NAME, err);
        return None;
    }

    return Some(dir_path);
}

fn create_cli_chat_file(base_path: PathBuf, name: &str) -> Option<File> {
    let path = base_path.join(name);
    match fs::File::create(&path) {
        Err(err) => {
            eprintln!("Error creating {} file: {}", name, err);
            return None;
        }, 
        Ok(file) => Some(file)
    }
}

fn open_cli_chat_file(name: &str) -> Option<File> {
    let cli_root = get_root_dir().unwrap();
    let path = cli_root.join(name);
    let mut cli_chat_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);
    
    match cli_chat_file {
        Err(err) => {
            eprintln!("Error opening {} file: {}", name, err);
            return None;

        }
        Ok(file) => Some(file)
    }
}

/*
Reads username from .cli_chat/username
*/
pub fn read_username() -> io::Result<[u8; field_lens::UNAME_LEN]> {
    let mut uname_file = open_cli_chat_file(UNAME_FN).unwrap();
    let mut uname_buffer = [0u8; field_lens::UNAME_LEN];
    uname_file.read(&mut uname_buffer);

    Ok(uname_buffer)
}

/*
Reads token from .cli_chat/token
*/
pub fn read_token() -> io::Result<[u8; field_lens::TOKEN_LEN]> {
    let mut token_file = open_cli_chat_file(TOKEN_FN).unwrap();
    let mut token_buffer = [0u8; field_lens::TOKEN_LEN];
    token_file.read(&mut token_buffer);

    Ok(token_buffer)
}

/*
From the connections-list file, build up a HashMap<String, usize>
that maps connection usernames to a unique index
*/
pub fn get_connections_map() -> io::Result<HashMap<String, usize>> {
    let mut conn_list_file = open_cli_chat_file(CONN_LIST_FN).unwrap();
    let mut uname_map = HashMap::new();
    let reader = io::BufReader::new(conn_list_file);
    
    for (i, line) in reader.lines().enumerate() {
        let uname = line?;
        uname_map.insert(uname, i);
    }

    Ok(uname_map)
}

/*
Adds a new connection
    - appending the corresponding username to 'connections-list' AND;
    - creating a new connections/connX file AND;
    - adding an entry to the connections map

NOTE: - here, we assume adding this connection is valid
*/
pub fn add_new_connection(uname: String, 
    conn_map: &mut HashMap<String, usize>,
    conn_list_file: &mut File) -> Result<(), io::Error> {

    // add to 'connections-list'
    writeln!(conn_list_file, "{}", &uname);

    // create connections/connX file
    let base_path = get_root_dir().unwrap();
    let conn_id = conn_map.len();
    let new_conn_file_name = format!("{}{}", CONN_FILE_PREFIX, conn_id);
    create_cli_chat_file(base_path.join("connections"), &new_conn_file_name);

    // add entry to connections map
    conn_map.insert(uname, conn_id);
    println!("{:?}", conn_map);

    Ok(())
}

// TEST //

pub fn test_add_conn() {
    let mut conn_list_file = open_cli_chat_file(CONN_LIST_FN).unwrap();
    let mut conn_map = get_connections_map().unwrap();
    let mut unames: Vec<String> = Vec::new();
    unames.push(String::from("Kerry"));
    unames.push(String::from("Eddie"));
    unames.push(String::from("Harry"));
    for uname in unames {
        add_new_connection(uname, &mut conn_map, &mut conn_list_file);
    }
}

pub fn test_get_conn_map() {
    let mut uname_list: Vec<String> = Vec::new();
    uname_list.push(String::from("harry"));
    uname_list.push(String::from("kerry"));
    uname_list.push(String::from("eddie"));

    let conn_map = get_connections_map().unwrap();
    for uname in uname_list {
        println!("{:?} -> {:?}", uname, conn_map.get(&uname).unwrap());
    }
}







