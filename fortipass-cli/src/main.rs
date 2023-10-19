use fortipass_core::models::Password;
use fortipass_core::passmanager::PasswordManager;
use utils::{set_password_file, generate_new_key_file, screen_clean, show_keys, show_pass, remove_file};

use crate::file_manager::FileManager;
use crate::utils::{get_key_file, retrieve_password_file, input};

mod utils;
mod file_manager;
mod password_creator;
mod key_creator;


fn show_menu() {
    println!("[ 1 ] Get password");
    println!("[ 2 ] Set password");
    println!("[ 3 ] Generate new key");
    println!("[ 4 ] Show list keys");
    println!("[ 5 ] Show list passwords");
    println!("[ 6 ] Delete key");
    println!("[ 7 ] Delete password");
    println!("[ q ] Quit");
}


fn main() {
    // TODO: TUI

    loop {
        screen_clean();

        show_menu();

        let inp = input("\nInput: ").expect("Failed read stdin `input`");

        match inp.trim() {
            "q" => break,

            // Get password
            "1" => {
                if let Err(err) = show_pass() {
                    println!("[ Failed ] failed show all passwords: {}", err.kind());
                }

                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");
                let site = input("\nSite: ").expect("Failed read stdin `keyname`");

                // Retrieve key
                let file_manager = FileManager::new(&keyname.trim());
                match get_key_file(&file_manager, file_manager.key_name) {
                    Ok(key) => {
                        let pass_manager = PasswordManager::new(key);

                        // Get password
                        match retrieve_password_file(&file_manager, &pass_manager, &site.trim()) {
                            Ok(pass) => println!("\nUsername: {}\nPassword: {}", pass.username, pass.password),
                            Err(err) => println!("[ Failed ] failed getting password `{}`: {}", site.trim(), err.kind())
                        }
                    },
                    Err(err) => println!("[ Failed ] failed getting key `{}`: {}", keyname.trim(), err.kind())
                }
            },

            // Set password
            "2" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");
                let site = input("\nSite: ").expect("Failed read stdin `site`");
                let username = input("Username: ").expect("Failed read stdin `username`");
                let pass = input("Password: ").expect("Failed read stdin `password`");

                // Retrieve key
                let file_manager = FileManager::new(&keyname.trim());
                match get_key_file(&file_manager, file_manager.key_name) {
                    Ok(key) => {
                        let mut pass_manager = PasswordManager::new(key);
                        pass_manager.set_password(Password::new(&site.trim(), &username.trim(), &pass.trim()));
                        match set_password_file(&file_manager, &pass_manager) {
                            Ok(_) => println!("[ Success ] set new password successfully `{}`", site.trim()),
                            Err(err) => println!("[ Failed ] failed setting password `{}`: {}", site.trim(), err.kind())
                        }
                    },
                    Err(err) => println!("[ Failed ] failed getting key`{}`: {}", keyname.trim(), err.kind())
                }
            },

            // Generate new key
            "3" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");

                // set key name
                let file_manager = FileManager::new(&keyname.trim());

                match generate_new_key_file(&file_manager) {
                    Ok(_) => println!("\n[ Success ] generate new key successfully `{}`", keyname.trim()),
                    Err(err) => println!("\n[ Failed ] failed generate new key `{key}`: {err}", key=keyname.trim() ,err=err.kind())
                }
            },

            // Show list all keys
            "4" => {
                if let Err(err) = show_keys() {
                    println!("[ Failed ] failed show all keys: {}", err.kind());
                }
            },

            // Show list all passwords
            "5" => {
                if let Err(err) = show_pass() {
                    println!("[ Failed ] failed show all passwords: {}", err.kind());
                }
            },

            // Delete key
            "6" => {
                if let Err(err) = show_keys() {
                    println!("[ Failed ] failed show all keys: {}", err.kind());
                }

                let keyname = input("\nKey: ").expect("Failed read stdin `key`");
                let comfirm = input(&format!("Are you sure want to remove key `{}` [y/n]: ", keyname.trim())).expect("Failed read stdin `comfirm`");
                let file_manager = FileManager::new(&keyname.trim());

                if comfirm.trim().to_lowercase() == "y" {
                    match remove_file(&file_manager, keyname.trim().to_owned() + ".key") {
                        Ok(_) => println!("\n[ Success ] key remove successfully `{}`", keyname.trim()),
                        Err(err) => println!("\n[ Failed ] failed remove key `{key}`: {err}", key=keyname.trim() ,err=err.kind())
                    }
                } else {
                    println!("cancled");
                }
            },

            // Delete password
            "7" => {
                if let Err(err) = show_pass() {
                    println!("[ Failed ] failed show all passwords: {}", err.kind());
                }

                let pass = input("\nPass: ").expect("Failed read stdin `pass`");
                let comfirm = input(&format!("Are you sure want to remove password `{}` [y/n]: ", pass.trim())).expect("Failed read stdin `comfirm`");
                let file_manager = FileManager::new(&pass.trim());

                if comfirm.trim().to_lowercase() == "y" {
                    match remove_file(&file_manager, pass.trim().to_owned()) {
                        Ok(_) => println!("\n[ Success ] password remove successfully `{}`", pass.trim()),
                        Err(err) => println!("\n[ Failed ] failed remove key `{key}`: {err}", key=pass.trim() ,err=err.kind())
                    }
                } else {
                    println!("cancled");
                }
            }

            _ => continue
        }

        // Continue
        let _ = input("\nPass `Enter` to continue.").expect("Failed continue Key");
    }
}
