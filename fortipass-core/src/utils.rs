use crypto::symmetriccipher::SymmetricCipherError;


pub trait Encryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError>;
}
