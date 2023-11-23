mod message;
use message::Message;

/**
Represents the main wrapper for all protocol messages. 
*/
pub struct Packet {
    pub method: u8,
    pub length: u32,
    pub buffer: Vec<u8>,
}

fn main() {
    let message = Message::new("hcmgr", "jess", "Morning darling");
    let buffer: Vec<u8> = message.serialize();
    let test_message = match Message::deserialize(&buffer) {
        Some(tm) => tm,
        None => panic!("invalid message")
    };
    println!("{:?}", test_message);
}
