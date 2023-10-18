#[cfg(test)]
mod tests {
    use fortipass_core::keymanager::KeyManager;
    use fortipass_core::models::Password;
    use fortipass_core::passmanager::PasswordManager;
    use fortipass_core::utils::Encryption;


    #[test]
    fn generate_new_key() {
        let key_manager = KeyManager::new();
        let key = key_manager.generate_key();

        let pass_manager = PasswordManager::new(key);

        let content = b"hello, world!";

        let encryped = pass_manager.encrypt(content).unwrap();
        let decryped = pass_manager.decrypt(&encryped).unwrap();

        assert_eq!(content.to_vec(), decryped);
    }


    #[test]
    fn encrypt_decrypt_content() {
        let key_manager = KeyManager::new();
        let key = key_manager.generate_key();

        let pass_manager = PasswordManager::new(key);

        let content = b"hello, world!";

        let encryped = pass_manager.encrypt(content).unwrap();
        let decryped = pass_manager.decrypt(&encryped).unwrap();

        assert_eq!(content.to_vec(), decryped);
    }


    #[test]
    fn read_encrypted_data() {
        let key_manager = KeyManager::new();
        let key = key_manager.generate_key();

        let pass_manager = PasswordManager::new(key);

        let content = Password::new("facebook", "marco", "2004marco");
        let encrypted = pass_manager.encrypt_password(&content).unwrap();

        let result = pass_manager.read_data(&encrypted, &content.site).unwrap();

        assert_eq!(content.password, result.password);
    }
}

