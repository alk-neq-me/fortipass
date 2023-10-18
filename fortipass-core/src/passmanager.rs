use crypto::aes::KeySize;
use crypto::buffer::{WriteBuffer, ReadBuffer, BufferResult};
use crypto::symmetriccipher::SymmetricCipherError;

use crate::models::Password;
use crate::utils::Encryption;




pub struct PasswordManager {
    pub key: [u8; 32],
    pub value: Option<Password>
}

impl PasswordManager {
    pub fn new(key: [u8; 32]) -> PasswordManager {
        PasswordManager {
            key,
            value: None
        }
    }
}


impl PasswordManager {
    pub fn read_data(&self, encrypted: &[u8], site: &str) -> Result<Password, SymmetricCipherError> {
        let decrypted = String::from_utf8(self.decrypt(&encrypted)?).expect("Failed decrypted data.");
        let username = decrypted.split(":").nth(0).expect("Username not found.");
        let password = decrypted.split(":").nth(1).expect("Password not found.");

        let decrypted = Password::new(
            site,
            &username,
            &password
        );

        Ok(decrypted)
    }

    pub fn encrypt_password(&self, data: &Password) -> Result<Vec<u8>, SymmetricCipherError> {
        let data = format!("{}:{}", data.username, data.password);
        let encrypted = self.encrypt(data.as_bytes())?;

        Ok(encrypted)
    }


    pub fn set_password(&mut self, data: Password) {
        self.value = Some(data)
    }
}


impl Encryption for PasswordManager {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut encryptor = crypto::aes::cbc_encryptor(KeySize::KeySize256, &self.key, &[0u8; 16], crypto::blockmodes::PkcsPadding);

        let mut final_result = Vec::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter());

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut decryptor = crypto::aes::cbc_decryptor(
            KeySize::KeySize256,
            &self.key, 
            &[0u8; 16], 
            crypto::blockmodes::PkcsPadding
        );

        let mut final_result = Vec::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter());

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }

        }
        Ok(final_result)
    }
}
