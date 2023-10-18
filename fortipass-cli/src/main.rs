use std::{path::Path, io};

mod utils;
mod file_manager;
mod password_creator;
mod key_creator;

use file_manager::FileManager;
use fortipass_core::{keymanager::KeyManager, passmanager::PasswordManager};
use key_creator::KeyCreator;
use password_creator::PasswordCreator;
use utils::Creator;


fn get_key(path: &Path) -> io::Result<[u8; 32]> {
    let key_creator = KeyCreator;
    key_creator.retrieve(&KeyManager, &FileManager::new(path), "key")
}


fn set_password(path: &Path, key: [u8; 32], site: &str) -> io::Result<()> {
    let password_creator = PasswordCreator;
    password_creator.create(&PasswordManager::new(key), &FileManager::new(path), "fb", Some("marco:some"))
}


fn main() {
    let secrets_path = Path::new("../.secrets");
    let key = get_key(&secrets_path).unwrap();
    set_password(&secrets_path, key, "fb").unwrap();
}
