use std::io;
use std::io::{Read, Write};
use std::fs;

use fortipass_core::passmanager::PasswordManager;
use fortipass_core::models::Password;

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
        let mut encrypted = Vec::new();

        fp.read_to_end(&mut encrypted)?;

        match manager.read_data(&encrypted, filename) {
            Ok(password) => Ok(password),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }
}
