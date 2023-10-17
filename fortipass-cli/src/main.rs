#![allow(unused)]

use std::{fs, io};
use std::io::{Write, Read};
use std::path::Path;

use fortipass_core::keymanager::KeyManager;
use fortipass_core::models::{EncryptedPassword, Password};
use fortipass_core::passmanager::PasswordManager;
use fortipass_core::utils::{KeyFileManager, Encryption};


fn _mock_passwords(pass_manager: &PasswordManager) -> Vec<EncryptedPassword> {
    let my_pass = [
        Password::new("facebook", "marco", "marco123"),
        Password::new("instagram", "some", "2324"),
    ];
    let encrypted = my_pass.iter().map(|p| pass_manager.set_password(&p).unwrap());
    encrypted.collect()
}


fn set_file_pass(pass: &str, path: &Path, pass_manager: &PasswordManager) -> io::Result<()> {
    let mut fp = fs::File::create(&path)?;
    // let encrypted = pass_manager.encrypt(pass.as_bytes()).expect("Failed encrypted data.");
    let encrypted = pass_manager.encrypt(b"fb:marco").expect("Failed encrypted data.");

    fp.write_all(&encrypted)?;

    Ok(())
}


fn read_file_pass(site: &str, path: &Path, pass_manager: &PasswordManager) -> io::Result<Password> {
    let mut fp = fs::File::open(&path)?;
    let mut buffer = Vec::new();

    fp.read_to_end(&mut buffer)?;

    let decrypted = pass_manager.decrypt(&buffer).expect("Failed decrypt buffer.");

    let content = String::from_utf8(decrypted).expect("Failed bytes extract.");

    let username = &content.split(":").collect::<Vec<&str>>()[0];
    let password = &content.split(":").collect::<Vec<&str>>()[1];

    let pass = Password::new(site, username, password);

    Ok(pass)
}


fn main() {
    let key_path = Path::new("./key.bin");
    let fb_path = Path::new("./fbpass.bin");

    let key_manager = KeyManager::new();
    // key_manager.create_file(&key_path).unwrap();
    let key = key_manager.read_file(&key_path).unwrap();

    let pass_manager = PasswordManager::new(&key);

    // set_file_pass("marco:pass123", &fb_path, &pass_manager).unwrap();
    let pass = read_file_pass("fb", &fb_path, &pass_manager).unwrap();
    println!("{:?}", pass);
}
