#[cfg(test)]
mod tests {
    use std::path::Path;

    use fortipass::{keymanager::KeyManager, passmanager::PasswordManager, utils::{Encryption, RwManager}};

    #[test]
    fn generate_new_key() {
        let key_manager = KeyManager;
        let pass_manager = PasswordManager;

        let key = key_manager.generate_key();
        let content = b"hello, world!";

        let encryped = pass_manager.encrypt(&key, content).unwrap();
        let decryped = pass_manager.decrypt(&key, &encryped).unwrap();

        assert_eq!(content.to_vec(), decryped);
    }

    #[test]
    fn read_key_file() {
        let key_path = Path::new("../fish.key");

        let key_manager = KeyManager;
        let pass_manager = PasswordManager;

        let key = key_manager.read_file(&key_path).unwrap();
        let content = b"hello, world!";

        let encryped = pass_manager.encrypt(&key, content).unwrap();
        let decryped = pass_manager.decrypt(&key, &encryped).unwrap();

        assert_eq!(content.to_vec(), decryped);
    }
}

