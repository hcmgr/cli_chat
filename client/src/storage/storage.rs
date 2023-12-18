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
    - stores usernames for each valid connection
    - from this, construct HashMap<String, X>
        where X is an id used to uniquely identify connection file (conn_X)
    - format:
        {conn_uname_1} (50 bytes)
        {conn_uname_2} (50 bytes)
        ...

connX:
    - stores all messages and metadata for a given connection (X)
    - the format is as follows:
        {Message 1}
        Magic bytes (4 bytes)
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
use std::io::{self, Read, Write, BufRead};
use protocol::{self, field_lens, ChatMessage};
use super::conn_map;

pub const ROOT_DIR_NAME: &str = ".cli_chat";
pub const TOKEN_FN: &str = "token";
pub const UNAME_FN: &str = "username";
pub const CONN_LIST_FN: &str = "connections-list";
pub const CONN_DIR_NAME: &str = "connections";
pub const CONN_FILE_PREFIX: &str = "conn";

pub const NUM_MAGIC_BYTES: usize = 4;
// pub const MAGIC_BYTES: [u8; NUM_MAGIC_BYTES] = [114, 97, 99, 107];
pub const MAGIC_BYTES: [u8; NUM_MAGIC_BYTES] = [45, 45, 45, 45];

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

    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
    {
        Err(err) => {
            eprintln!("Error opening {} file: {}", name, err);
            None
        }
        Ok(file) => Some(file),
    }
}

fn open_cli_chat_file(name: &str) -> Option<File> {
    let cli_root = get_root_dir().unwrap();
    let path = cli_root.join(name);
    let mut cli_chat_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
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

/**
Reads username from .cli_chat/username
*/
pub fn read_username() -> io::Result<[u8; field_lens::UNAME_LEN]> {
    let mut uname_file = open_cli_chat_file(UNAME_FN).unwrap();
    let mut uname_buffer = [0u8; field_lens::UNAME_LEN];
    uname_file.read(&mut uname_buffer);

    Ok(uname_buffer)
}

/**
Reads token from .cli_chat/token
*/
pub fn read_token() -> io::Result<[u8; field_lens::TOKEN_LEN]> {
    let mut token_file = open_cli_chat_file(TOKEN_FN).unwrap();
    let mut token_buffer = [0u8; field_lens::TOKEN_LEN];
    token_file.read(&mut token_buffer);

    Ok(token_buffer)
}

/**
Initialises the connections map from the 'connections-list' file
*/
pub fn init_conn_map() {
    let mut conn_list_file = open_cli_chat_file(CONN_LIST_FN).unwrap();
    let reader = io::BufReader::new(conn_list_file);
    for (i, line) in reader.lines().enumerate() {
        let uname = line.unwrap();
        conn_map::insert(uname.clone(), uname.clone());
    }
}

/**
Adds a new connection
    - appending the corresponding username to 'connections-list' AND;
    - creating a new connections/connX file AND;
    - adding an entry to the connections map

NOTE: - here, we assume adding this connection is valid
*/
pub fn add_new_connection(uname: String) -> Result<(), io::Error> {
    
    // add to 'connections-list'
    let mut conn_list_file = open_cli_chat_file(CONN_LIST_FN).unwrap();
    writeln!(conn_list_file, "{}", &uname);

    // create connections/connX file
    let base_path = get_root_dir().unwrap();
    create_cli_chat_file(base_path.join(CONN_DIR_NAME), 
        &get_conn_file_name(uname.clone()));

    // add entry to connections map
    conn_map::insert(uname.clone(), uname.clone());

    Ok(())
}

fn get_conn_file_name(uname: String) -> String {
    return format!("{}_{}", CONN_FILE_PREFIX, uname);
}

fn get_conn_file_path(uname: String) -> PathBuf {
    let base_path = get_root_dir().unwrap();
    let file_name = get_conn_file_name(uname);
    return base_path.join(CONN_DIR_NAME).join(file_name);
}

/**
Writes given message to corresponding connX file
*/
pub fn write_message(chat_message: ChatMessage, conn_uname: String) -> Option<usize> {
    let file_path = get_conn_file_path(conn_uname);
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path).unwrap();

    // write magic bytes
    let mut bytes_written = file.write(&MAGIC_BYTES).unwrap();
    if bytes_written != NUM_MAGIC_BYTES {
        println!("Error writting message: magic bytes");
        return None;
    }

    // write message
    let ser_chat_message = chat_message.serialize();
    bytes_written += file.write(&ser_chat_message).unwrap();
    if bytes_written != chat_message.length() + NUM_MAGIC_BYTES {
        println!("Error writting message: message");
        return None;
    }
    return Some(bytes_written);
}

/**
Reads all messages from connX file into list
*/
pub fn read_messages(uname: String) -> Option<Vec<ChatMessage>> {
    let file_path = get_conn_file_path(uname);
    let file = OpenOptions::new()
        .read(true)
        .open(file_path).unwrap();
    let mut file_reader = io::BufReader::new(file);
    let mut messages: Vec<ChatMessage> = Vec::new();

    loop {
        // read magic bytes
        let mut magic_bytes_buffer = [0u8; NUM_MAGIC_BYTES];
        match file_reader.read_exact(&mut magic_bytes_buffer) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {break;} // eof
            Err(e) => {return None;}
        }
        if !magic_bytes_buffer.iter().eq(MAGIC_BYTES.iter()) {
            println!("Error reading message: magic bytes");
            return None;
        }

        // read length field
        let mut length_buffer = [0u8; field_lens::MSGLEN_LEN];
        match file_reader.read_exact(&mut length_buffer) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {break;} // eof
            Err(e) => {return None;}
        }
        let msg_length: u32 = u32::from_be_bytes(length_buffer);
        if (msg_length as usize) == 0 {
            println!("Error reading message: length");
            return None;
        }

        // read message
        let chat_message_length = ChatMessage::fixed_size() + (msg_length as usize);
        let mut message_buffer = Vec::new();
        message_buffer.extend_from_slice(&length_buffer);
        message_buffer.resize(chat_message_length, 0);
        match file_reader.read_exact(&mut message_buffer[field_lens::MSGLEN_LEN..]) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {break;} // eof
            Err(e) => {return None;}
        }
        let chat_message = ChatMessage::deserialize(&message_buffer).unwrap();
        messages.push(chat_message);
    }
    return Some(messages);
}