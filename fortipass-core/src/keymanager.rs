extern crate crypto;
extern crate rand;

use std::io;
use std::fs;
use std::io::Read;
use std::path::Path;
use rand::Rng;

use crate::utils::KeyFileManager;


pub struct KeyManager;

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager
    }
}


impl KeyFileManager for KeyManager {
    fn read_file(&self, path: &Path) -> Result<[u8; 32], io::Error> {
        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }
        let mut key = [0u8; 32];
        let mut fp = fs::File::open(path)?;
        fp.read_exact(&mut key).expect("Failed read key.");
        Ok(key)
    }

    fn create_file(&self, path: &Path) -> Result<(), io::Error> {
        let key = self.generate_key();
        fs::write(path, &key)
    }
}


impl KeyManager {
    pub fn generate_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        key
    }
}
