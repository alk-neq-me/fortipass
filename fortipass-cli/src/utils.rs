use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::{io, fs};
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

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager) -> io::Result<()>;
}


pub fn get_key_file(
    file_manager: &FileManager,
) -> io::Result<[u8; 32]> {
    let key_creator = KeyCreator;
    key_creator.retrieve(file_manager)
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


pub fn set_owner_perm(path: &Path) -> io::Result<()> {
    let info = fs::metadata(&path)?;
    let mut perms = info.permissions();

    perms.set_mode(0o400);  // dr--------

    fs::set_permissions(&path, perms)?;

    Ok(())
}


pub fn show_pass() -> io::Result<()> {
    let entiries = fs::read_dir("/etc/fortipass/.secrets")?;

    for entry in entiries {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if !name.ends_with(".key") {
                    println!("🔒 {}", name);
                }
            }
        }
    }

    Ok(())
}


pub fn show_keys() -> io::Result<()> {
    let entiries = fs::read_dir("/etc/fortipass/.secrets")?;

    for entry in entiries {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".key") {
                    println!("🔑 {}", name);
                }
            }
        }
    }

    Ok(())
}


pub fn remove_file(
    file_manager: &FileManager,
    name: String
) -> io::Result<()> {
    let path = file_manager.secrets_path.join(name);

    if !path.is_file() {
        return Err(io::Error::from(io::ErrorKind::NotFound))
    }

    fs::remove_file(path)?;
    Ok(())
}


pub fn initial_add_passwords(
    file_manager: &FileManager,
) -> io::Result<()> {
    let initial = vec![
        Password::new("sitename", "username", "password"),
    ];

    let mut pass_manager = PasswordManager::new(get_key_file(&file_manager)?);
    let password_creator = PasswordCreator;

    for pass in initial.into_iter() {
        pass_manager.set_password(pass);
        password_creator.create(&pass_manager, &file_manager)?;
    }

    Ok(())
}
