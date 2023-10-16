use std::io;
use std::path::Path;
use crypto::aes::KeySize;
use crypto::buffer::{WriteBuffer, ReadBuffer, BufferResult};
use crypto::symmetriccipher::SymmetricCipherError;

use crate::utils::{RwManager, Encryption};

pub struct PasswordManager;

impl RwManager for PasswordManager {
    fn read_file(&self, path: &Path) -> Result<[u8;32], io::Error> {
        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }
        Ok([0u8;32])
    }

    fn create_file(&self, _: &Path) -> Result<(), io::Error> {
        Ok(())
    }
}


impl Encryption for PasswordManager {
    fn encrypt(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut encryptor = crypto::aes::cbc_encryptor(KeySize::KeySize256, key, &[0u8; 16], crypto::blockmodes::PkcsPadding);

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

    fn decrypt(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut decryptor = crypto::aes::cbc_decryptor(
            KeySize::KeySize256,
            key, 
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
