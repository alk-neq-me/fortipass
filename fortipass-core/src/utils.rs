use std::path::Path;
use std::io;

use crypto::symmetriccipher::SymmetricCipherError;


pub trait KeyFileManager {
    fn create_file(&self, path: &Path) -> Result<(), io::Error>;

    fn read_file(&self, path: &Path) -> Result<[u8; 32], io::Error>;
}


pub trait Encryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;
}
