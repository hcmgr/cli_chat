pub mod tests {
    use crate::storage::storage::{add_new_connection, read_messages, write_message};    
    use crate::storage::conn_map;
    use protocol::ChatMessage;

    pub const HARRY_UNAME: &str = "Harry";
    pub const EDDIE_UNAME: &str = "Eddie";
    pub const KERRY_UNAME: &str = "Kerry";

    pub const MSG1: &str = "im not grumba grandpa guy dude no and no";
    pub const MSG2: &str = "yes I am so that guy gramps oh and yes";
    pub const MSG3: &str = "I'm feeling quite cheerful today!";
    pub const MSG4: &str = "Well, I'm feeling a bit grumpy.";    

    pub fn test_singelton_map() {
        // Insert key-value pair into the singleton HashMap.
        conn_map::insert("key1".to_string(), "val1".to_string());
        conn_map::insert("key2".to_string(), "val2".to_string());

        // Access the singleton HashMap.
        let my_map = conn_map::get_map();
        println!("{:?}", my_map);

        // Remove a key from the singleton HashMap.
        conn_map::remove("key1");
    }

    pub fn test_read_message() {
        setup();
        let harryUname = String::from(HARRY_UNAME);
        let eddieUname = String::from(EDDIE_UNAME);
        let kerryUname = String::from(KERRY_UNAME);
        // read eddie messages
        let eddie_messages = read_messages(eddieUname.clone()).unwrap();
        for msg in eddie_messages {
            println!("{:?}", msg);
        }

        // read kerry messages
        let kerry_messages = read_messages(kerryUname.clone()).unwrap();    
        for msg in kerry_messages {
            println!("{:?}", msg);
        }   
    }

    fn setup() {
        // Create usernames
        let harryUname = String::from(HARRY_UNAME);
        let eddieUname = String::from(EDDIE_UNAME);
        let kerryUname = String::from(KERRY_UNAME);

        // Add new connections (users)
        add_new_connection(eddieUname.clone());
        add_new_connection(kerryUname.clone());

        // Create messages
        let chat_message1 = ChatMessage::new(&harryUname, &eddieUname, MSG1);
        let chat_message2 = ChatMessage::new(&eddieUname, &harryUname, MSG2);
        let chat_message3 = ChatMessage::new(&harryUname, &kerryUname, MSG3);
        let chat_message4 = ChatMessage::new(&kerryUname, &harryUname, MSG4);

        // Write messages
        write_message(chat_message1, eddieUname.clone()).unwrap();
        write_message(chat_message2, eddieUname.clone()).unwrap();
        write_message(chat_message3, kerryUname.clone()).unwrap();  
        write_message(chat_message4, kerryUname.clone()).unwrap();
    }
}
