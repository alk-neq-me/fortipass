extern crate crypto;
extern crate rand;

use rand::Rng;


pub struct KeyManager;

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager
    }
}


impl KeyManager {
    pub fn generate_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        key
    }
}
