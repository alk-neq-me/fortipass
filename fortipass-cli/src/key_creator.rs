use std::io;

use fortipass_core::keymanager::KeyManager;
use fortipass_core::utils::KeyFileManager;

use crate::file_manager::FileManager;
use crate::utils::Creator;


pub struct KeyCreator;

impl Creator for KeyCreator {
    type Manager = KeyManager;
    type Return = [u8; 32];

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str, _: Option<&str>) -> io::Result<()> {
        let path = file_manager.secrets_path.join(filename);
        manager.create_file(&path)?;

        Ok(())
    }

    fn retrieve(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str) -> io::Result<Self::Return> {
        let path = file_manager.secrets_path.join(filename);

        manager.read_file(&path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fortipass_core::keymanager::KeyManager;

    use crate::{file_manager::FileManager, key_creator::KeyCreator, utils::Creator};

    #[test]
    fn getting_key() {
        let file_manager = FileManager::new(&Path::new("../.secrets"));
        let key_manager = KeyManager;
        let key_creator = KeyCreator;

        let mut key = [0u8; 32];

        if !file_manager.secrets_path.is_dir() {
            key_creator.create(&key_manager, &file_manager, "key", None).expect("Failed creating key file.");
            key = key_creator.retrieve(&key_manager, &file_manager, "key").expect("Faled getting key.");
        }

        assert_eq!(32, key.len());
    }
}
