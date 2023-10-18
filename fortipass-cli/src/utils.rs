use std::io;
use std::io::Write;
use std::process::Command;

use fortipass_core::keymanager::KeyManager;
use fortipass_core::passmanager::PasswordManager;
use fortipass_core::models::Password;

use crate::{file_manager::FileManager, key_creator::KeyCreator, password_creator::PasswordCreator};


const BANNAR: &str = r#"
 ███▄ ▄███▓ ▄▄▄       ██▀███   ▄████▄   ▒█████  
▓██▒▀█▀ ██▒▒████▄    ▓██ ▒ ██▒▒██▀ ▀█  ▒██▒  ██▒
▓██    ▓██░▒██  ▀█▄  ▓██ ░▄█ ▒▒▓█    ▄ ▒██░  ██▒
▒██    ▒██ ░██▄▄▄▄██ ▒██▀▀█▄  ▒▓▓▄ ▄██▒▒██   ██░
▒██▒   ░██▒ ▓█   ▓██▒░██▓ ▒██▒▒ ▓███▀ ░░ ████▓▒░
░ ▒░   ░  ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░░ ░▒ ▒  ░░ ▒░▒░▒░ 
░  ░      ░  ▒   ▒▒ ░  ░▒ ░ ▒░  ░  ▒     ░ ▒ ▒░ 
░      ░     ░   ▒     ░░   ░ ░        ░ ░ ░ ▒  
       ░         ░  ░   ░     ░ ░          ░ ░  
                              ░                 
"#;


pub trait Creator {
    type Manager;
    type Return;

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager) -> io::Result<()>;

    fn retrieve(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str) -> io::Result<Self::Return>;
}


pub fn get_key_file(
    file_manager: &FileManager,
    filename: &str
) -> io::Result<[u8; 32]> {
    let key_creator = KeyCreator;
    key_creator.retrieve(&KeyManager, file_manager, filename)
}


pub fn generate_new_key_file(
    file_manager: &FileManager,
) -> io::Result<()> {
    let key_creator = KeyCreator;
    let key_manager = KeyManager;
    key_creator.create(&key_manager, file_manager)
}


pub fn set_password_file(
    file_manager: &FileManager,
    pass_manager: &PasswordManager,
) -> io::Result<()> {
    let password_creator = PasswordCreator;
    password_creator.create(pass_manager, file_manager)
}


pub fn retrieve_password_file(
    file_manager: &FileManager,
    pass_manager: &PasswordManager,
    filename: &str
) -> io::Result<Password> {
    let password_creator = PasswordCreator;
    password_creator.retrieve(pass_manager, file_manager, filename)
}


pub fn input(txt: &str) -> io::Result<String> {
    let mut inp = String::new();
    print!("{txt}");
    io::stdout().flush()?;
    io::stdin().read_line(&mut inp)?;

    Ok(inp)
}


pub fn screen_clean() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    };
    println!("{BANNAR}");
}
