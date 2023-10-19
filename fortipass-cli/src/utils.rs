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
<<<<<<< HEAD
        Password::new("rapidapi", "toyko2001", "..."),
        Password::new("Google", "yuujunlee", "20justme->google::yuujunlee01"),
        Password::new("Google", "toyko2001", "justme->google::toyko01"),
        Password::new("Binance", "toyko2001", "20justme->Binance01"),
        Password::new("Payple", "cocolwin", "20justme->Payple01"),
        Password::new("Samsung", "toyko2001", "20alk!=me01"),
        Password::new("iPhone", "aungkokolwin1990", "20my!=other;me=me->iCloud01"),
        Password::new("Xiaomi", "6577665599", "Lwin12345"),
        Password::new("Facebook", "marco.exexx", "20justme->facebook::marco01"),
        Password::new("Twitch", "yoonjun", "20justme->facebook::yoonjun01"),
        Password::new("Pinterest", "toyko2001", "20pinterest01"),
        Password::new("Kakao", "toyko2001", "20justme->kakao::yujun01"),
        Password::new("Careerly", "kakao", "..."),
        Password::new("Mediafier", "toyko2001", "20mediafire01"),
        Password::new("Yoteshinportal.cc", "toyko2001", "..."),
        Password::new("Meganz", "toyko2001", "20meganz01"),
        Password::new("Protovpn", "toyko2001", "20protovpn01"),
        Password::new("Zenmate", "toyko2001", "20$zenmate=>$VPN01"),
        Password::new("Lifetime", "toyko2001", "20lifetimevpn01"),
        Password::new("Minecraft", "toyko2001", "20minecraft01"),
        Password::new("webshare.io", "toyko2001", "..."),
        Password::new("Expo", "marco.exexx", "20jutme->expo::marco01"),
        Password::new("Github", "toyko2001", "..."),
        Password::new("AWS", "toyko2001", "20my!=other;me=me->AWS01"),
=======
        Password::new("sitename", "username", "password"),
>>>>>>> d2f23ab (fixed: git error)
    ];

    let mut pass_manager = PasswordManager::new(get_key_file(&file_manager)?);
    let password_creator = PasswordCreator;

    for pass in initial.into_iter() {
        pass_manager.set_password(pass);
        password_creator.create(&pass_manager, &file_manager)?;
    }

    Ok(())
}
