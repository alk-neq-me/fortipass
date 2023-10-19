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

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager) -> io::Result<()> {
        println!("Creating...");
        match manager.value {
            Some(ref ctx) => {
                let path = file_manager.secrets_path.join(&ctx.site);
                let mut buf = fs::File::options().write(true).create_new(true).open(&path)?;
                let encrypted = manager.encrypt_password(&ctx).expect("Failed encrypted data.");

                buf.write_all(&encrypted)?;
            },
            None => panic!("Must password provide")
        }

        Ok(())
    }
}


impl PasswordCreator {
    pub fn retrieve(&self, manager: &PasswordManager, file_manager: &FileManager, filename: &str) -> io::Result<Password> {
        let mut fp = fs::File::open(file_manager.secrets_path.join(filename))?;
        let mut buffer = Vec::new();

        fp.read_to_end(&mut buffer)?;

        match manager.decrypt(&buffer) {
            Ok(decrypted) => {
                let content = String::from_utf8(decrypted).expect("Failed bytes extract.");

                let username = content.split(":").collect::<Vec<&str>>()[0];
                let password = content.split(":").collect::<Vec<&str>>()[1];

                let pass = Password::new(filename, username, password);

                Ok(pass)
            },
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }
}
