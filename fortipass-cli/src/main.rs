use std::path::Path;
use fortipass_core::models::Password;
use fortipass_core::passmanager::PasswordManager;
use utils::{set_password_file, generate_new_key_file, screen_clean, show_keys, show_pass, remove_file, initial_add_passwords, print_pass_file};

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
    println!("[ 8 ] Initial add passwords");
    println!("[ 9 ] Select key");
    println!("[ 10 ] Print pass file");
    println!("[ q ] Quit");
}


fn main() {
    let mut file_manager = FileManager::new();

    // TODO: TUI

    loop {
        screen_clean();

        show_menu();

        let inp = input("\nInput: ").expect("Failed read stdin `input`");

        if !file_manager.key_name.is_empty() {
            println!("Selected key: 🔑 {}", file_manager.key_name);
        }

        match inp.trim() {
            "q" => break,

            // Get password
            "1" => {
                match show_pass() {
                    Ok(passes) => {
                        for name in passes.iter() {
                            println!("🔒 {}", name)
                        }
                    },
                    Err(err) => println!("[ Failed ] failed show all passwords: {}", err.kind())
                }

                // Retrieve key [u8; 32]
                match get_key_file(&file_manager) {
                    Ok(key) => {
                        let site = input("\nSite: ").expect("Failed read stdin `keyname`");
                        let pass_manager = PasswordManager::new(key);

                        // Get password
                        match retrieve_password_file(&file_manager, &pass_manager, &site.trim()) {
                            Ok(pass) => println!("\nUsername: {}\nPassword: {}", pass.username, pass.password),
                            Err(err) => println!("[ Failed ] failed getting password `{}`: {}", site.trim(), err.kind())
                        }
                    },
                    Err(err) => println!("[ Failed ] failed getting key `{}`: {}", file_manager.key_name, err.kind())
                }
            },

            // Set password
            "2" => {
                // Retrieve key
                match get_key_file(&file_manager) {
                    Ok(key) => {
                        let site = input("\nSite: ").expect("Failed read stdin `site`");
                        let username = input("Username: ").expect("Failed read stdin `username`");
                        let pass = input("Password: ").expect("Failed read stdin `password`");

                        if !site.is_empty() {
                            let mut pass_manager = PasswordManager::new(key);
                            pass_manager.set_password(Password::new(&site.trim(), &username.trim(), &pass.trim()));
                            match set_password_file(&file_manager, &pass_manager) {
                                Ok(_) => println!("[ Success ] set new password successfully `{}`", site.trim()),
                                Err(err) => println!("[ Failed ] failed setting password `{}`: {}", site.trim(), err.kind())
                            }
                        }
                    },
                    Err(err) => println!("[ Failed ] failed getting key`{}`: {}", file_manager.key_name, err.kind())
                }
            },

            // Generate new key
            "3" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");

                if !keyname.is_empty() {
                    // set key name
                    file_manager.set_key(keyname.trim().to_owned());

                    match generate_new_key_file(&file_manager) {
                        Ok(_) => println!("\n[ Success ] generate new key successfully `{}`", keyname.trim()),
                        Err(err) => println!("\n[ Failed ] failed generate new key `{key}`: {err}", key=keyname.trim() ,err=err.kind())
                    }
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
                match show_pass() {
                    Ok(passes) => {
                        for name in passes.iter() {
                            println!("🔒 {}", name)
                        }
                    },
                    Err(err) => println!("[ Failed ] failed show all passwords: {}", err.kind())
                }
            },

            // Delete key
            "6" => {
                if let Err(err) = show_keys() {
                    println!("[ Failed ] failed show all keys: {}", err.kind());
                }

                let keyname = input("\nKey: ").expect("Failed read stdin `key`");
                let comfirm = input(&format!("Are you sure want to remove key `{}` [y/n]: ", keyname.trim())).expect("Failed read stdin `comfirm`");
                file_manager.set_key(keyname.trim().to_owned());

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
                match show_pass() {
                    Ok(passes) => {
                        for name in passes.iter() {
                            println!("🔒 {}", name)
                        }
                    },
                    Err(err) => println!("[ Failed ] failed show all passwords: {}", err.kind())
                }

                let pass = input("\nPass: ").expect("Failed read stdin `pass`");
                let comfirm = input(&format!("Are you sure want to remove password `{}` [y/n]: ", pass.trim())).expect("Failed read stdin `comfirm`");

                if comfirm.trim().to_lowercase() == "y" {
                    match remove_file(&file_manager, pass.trim().to_owned()) {
                        Ok(_) => println!("\n[ Success ] password remove successfully `{}`", pass.trim()),
                        Err(err) => println!("\n[ Failed ] failed remove password `{}`: {}", pass.trim(), err.kind())
                    }
                } else {
                    println!("cancled");
                }
            }

            // Initial add passwords
            "8" => {
                if let Err(err) = initial_add_passwords(&file_manager) {
                    println!("[ Failed ] failed initial set passwords: {}", err.kind());
                }
            },

            "9" => {
                if let Err(err) = show_keys() {
                    println!("[ Failed ] failed show all keys: {}", err.kind());
                }

                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");

                if !keyname.is_empty() {
                    let path = Path::new(&file_manager.secrets_path).join(keyname.trim()).with_extension("key");

                    if path.is_file() {
                        file_manager.set_key(keyname.trim().to_owned());
                        println!("[ Success ] key 🔑 `{}` selected successfully", keyname.trim());
                    } else {
                        println!("[ Failed ] failed select key: Key not found");
                    }
                }
            },

            // print all pass file
            "10" => {
                let passws = match show_pass() {
                    Ok(passes) => passes,
                    Err(err) => {
                        println!("[ Failed ] failed show all passwords: {}", err.kind());
                        vec![]
                    }
                };

                match get_key_file(&file_manager) {
                    Ok(key) => {
                        let pass_manager = PasswordManager::new(key);
                        let filename = input("filename: ").expect("Failed input");
                        let path = Path::new(filename.trim());

                        for site in passws.iter() {
                            match retrieve_password_file(&file_manager, &pass_manager, &site.trim()) {
                                Ok(pass) => print_pass_file(&site.trim(), &pass.username, &pass.password, path),
                                Err(err) => println!("[ Failed ] failed getting password `{}`: {}", site.trim(), err.kind())
                            }
                        }
                    },
                    Err(err) => println!("[ Failed ] failed getting key `{}`: {}", file_manager.key_name, err.kind())
                }
            },

            _ => continue
        }

        // Continue
        let _ = input("\nPass `Enter` to continue.").expect("Failed continue Key");
    }
}
