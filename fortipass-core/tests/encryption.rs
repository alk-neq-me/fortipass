#[cfg(test)]
mod tests {
    use std::path::Path;

    use fortipass_core::keymanager::KeyManager;
    use fortipass_core::models::Password;
    use fortipass_core::passmanager::PasswordManager;
    use fortipass_core::utils::{Encryption, KeyFileManager};


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
    fn read_key_file() {
        let key_path = Path::new("../.secrets/key");

        let key_manager = KeyManager::new();
        key_manager.create_file(&key_path).unwrap();
        let key = key_manager.read_file(&key_path).unwrap();

        let pass_manager = PasswordManager::new(key);

        let content = b"hello, world!";

        let encryped = pass_manager.encrypt(content).unwrap();
        let decryped = pass_manager.decrypt(&encryped).unwrap();

        assert_eq!(content.to_vec(), decryped);
    }


    #[test]
    fn read_encrypted_data() {
        let key_path = Path::new("../.secrets/key");

        let key_manager = KeyManager::new();
        let key = key_manager.read_file(&key_path).unwrap();

        let pass_manager = PasswordManager::new(key);

        let content = Password::new("facebook", "marco", "2004marco");
        let encrypted = pass_manager.set_password(&content).unwrap();

        let result = pass_manager.read_data(&encrypted).unwrap();

        assert_eq!(content.password, result.password);
    }
}

