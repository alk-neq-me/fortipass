use std::io;
use std::io::{Read, Write};
use std::fs;

use fortipass_core::passmanager::PasswordManager;
use fortipass_core::models::Password;
use fortipass_core::utils::Encryption;

use crate::file_manager::FileManager;
use crate::utils::Creator;


pub struct PasswordCreator;

impl Creator for PasswordCreator {
    type Manager = PasswordManager;
    type Return = Password;

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str, content: Option<&str>) -> io::Result<()> {
        match content {
            Some(ctn) => {
                let mut fp = fs::File::create(file_manager.secrets_path.join(filename))?;
                let encrypted = manager.encrypt(ctn.as_bytes()).expect("Failed encrypted data.");

                fp.write_all(&encrypted)?;
            },
            None => panic!("Must password provide")
        }

        Ok(())
    }

    fn retrieve(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str) -> io::Result<Self::Return> {
        let mut fp = fs::File::open(file_manager.secrets_path.join(filename))?;
        let mut buffer = Vec::new();

        fp.read_to_end(&mut buffer)?;

        let decrypted = manager.decrypt(&buffer).expect("Failed decrypt buffer.");

        let content = String::from_utf8(decrypted).expect("Failed bytes extract.");

        let username = &content.split(":").collect::<Vec<&str>>()[0];
        let password = &content.split(":").collect::<Vec<&str>>()[1];

        let pass = Password::new(filename, username, password);

        Ok(pass)
    }
}
