use crypto::aes::KeySize;
use crypto::buffer::{WriteBuffer, ReadBuffer, BufferResult};
use crypto::symmetriccipher::SymmetricCipherError;

use crate::models::{Password, EncryptedPassword};
use crate::utils::Encryption;




pub struct PasswordManager<'a> {
    pub key: &'a [u8],
    pub value: Option<Password>
}

impl<'a> PasswordManager<'a> {
    pub fn new(key: &'a [u8]) -> PasswordManager<'a> {
        PasswordManager {
            key,
            value: None
        }
    }
}


impl<'a> PasswordManager<'a> {
    pub fn read_data(&self, encrypted: &EncryptedPassword) -> Result<Password, SymmetricCipherError> {
        let username = String::from_utf8(self.decrypt(&encrypted.username)?).expect("Failed username extract.");
        let password = String::from_utf8(self.decrypt(&encrypted.password)?).expect("Failed password extract.");

        let decrypted = Password::new(
            &encrypted.site,
            &username,
            &password
        );

        Ok(decrypted)
    }

    pub fn set_password(&self, data: &Password) -> Result<EncryptedPassword, SymmetricCipherError> {
        let username = self.encrypt(data.username.as_bytes())?;
        let password = self.encrypt(data.password.as_bytes())?;

        let encrypted = EncryptedPassword::new(&data.site, username, password);

        Ok(encrypted)
    }
}


impl<'a> Encryption for PasswordManager<'a> {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut encryptor = crypto::aes::cbc_encryptor(KeySize::KeySize256, self.key, &[0u8; 16], crypto::blockmodes::PkcsPadding);

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
            self.key, 
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
