use std::io::{Write, Read};
use std::{io, fs};

use fortipass_core::keymanager::KeyManager;

use crate::file_manager::FileManager;
use crate::utils::Creator;


pub struct KeyCreator;

impl Creator for KeyCreator {
    type Manager = KeyManager;
    type Return = [u8; 32];

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager) -> io::Result<()> {
        let path = file_manager.secrets_path.join(file_manager.key_name);

        if path.is_file() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists))
        }

        let key = manager.generate_key();

        let mut buf = fs::File::create(&path)?;

        buf.write_all(&key)
    }

    fn retrieve(&self, _: &Self::Manager, file_manager: &FileManager, filename: &str) -> io::Result<Self::Return> {
        let path = file_manager.secrets_path.join(filename);

        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }

        let mut key = [0u8; 32];
        let mut buf = fs::File::open(path)?;
        buf.read_exact(&mut key).expect("Failed read key.");
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fortipass_core::keymanager::KeyManager;

    use crate::{file_manager::FileManager, key_creator::KeyCreator, utils::Creator};

    #[test]
    fn getting_key() {
        let file_manager = FileManager::new(&Path::new("../.secrets"), "key");
        let key_manager = KeyManager;
        let key_creator = KeyCreator;

        let mut key = [0u8; 32];

        if !file_manager.secrets_path.is_dir() {
            key_creator.create(&key_manager, &file_manager).expect("Failed creating key file.");
            key = key_creator.retrieve(&key_manager, &file_manager, file_manager.key_name).expect("Faled getting key.");
        }

        assert_eq!(32, key.len());
    }
}
