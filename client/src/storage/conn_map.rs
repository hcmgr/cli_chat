/*
Defines a singelton hashmap to store a mapping from 
connection username to a unique identifier
*/


use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

pub struct ConnMap {
    pub conn_map: HashMap<String, String>,
}

lazy_static! {
    static ref MODULE_DATA: Mutex<ConnMap> = Mutex::new(ConnMap {
        conn_map: HashMap::new(),
    });
}

pub fn get_map() -> HashMap<String, String> {
    MODULE_DATA.lock().unwrap().conn_map.clone()
}

pub fn insert(key: String, value: String) {
    MODULE_DATA.lock().unwrap().conn_map.insert(key, value);
}

pub fn remove(key: &str) {
    MODULE_DATA.lock().unwrap().conn_map.remove(key);
}