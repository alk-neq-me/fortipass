use std::{path::Path, io};

use crypto::symmetriccipher::SymmetricCipherError;

pub trait RwManager {
    fn create_file(&self, path: &Path) -> Result<(), io::Error>;

    fn read_file(&self, path: &Path) -> Result<[u8; 32], io::Error>;
}

pub trait Encryption {
    fn encrypt(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;

    fn decrypt(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;
}
