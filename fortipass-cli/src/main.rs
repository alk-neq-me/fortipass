use std::path::Path;

use fortipass_core::keymanager::KeyManager;
use fortipass_core::models::{EncryptedPassword, Password};
use fortipass_core::passmanager::PasswordManager;
use fortipass_core::utils::KeyFileManager;


fn get_passwords(pass_manager: &PasswordManager) -> Vec<EncryptedPassword> {
    let my_pass = [
        Password::new("facebook", "marco", "marco123"),
        Password::new("instagram", "some", "2324"),
    ];
    let encrypted = my_pass.iter().map(|p| pass_manager.set_password(&p).unwrap());
    encrypted.collect()
}


fn main() {
    let key_path = Path::new("./fish.key");

    let key_manager = KeyManager::new();
    let key = key_manager.read_file(&key_path).unwrap();

    let pass_manager = PasswordManager::new(&key);

    let my_pass = get_passwords(&pass_manager);
    println!("{:?}", my_pass.iter().next().unwrap());
}
