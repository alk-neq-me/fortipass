use std::io::{Write, Read};
use std::{io, fs};

use fortipass_core::keymanager::KeyManager;

use crate::file_manager::FileManager;
use crate::utils::Creator;


pub struct KeyCreator;

impl Creator for KeyCreator {
    type Manager = KeyManager;

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager) -> io::Result<()> {
        let path = file_manager.secrets_path.join(&file_manager.key_name).with_extension("key");

        if path.is_file() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists))
        }

        let key = manager.generate_key();

        let mut buf = fs::File::create(&path)?;

        buf.write_all(&key)
    }
}


impl KeyCreator {
    pub fn retrieve(&self, file_manager: &FileManager) -> io::Result<[u8; 32]> {
        let path = file_manager.secrets_path.join(&file_manager.key_name).with_extension("key");

        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }

        let mut key = [0u8; 32];
        let mut buf = fs::File::open(path)?;
        buf.read_exact(&mut key).expect("Failed read key.");
        Ok(key)
    }
}
