use std::path::Path;

use fortipass_core::models::Password;
use fortipass_core::passmanager::PasswordManager;
use utils::{set_password_file, generate_new_key_file, screen_clean};

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
    println!("[ q ] Quit");
}


fn main() {
    let secrets_path = Path::new("./.secrets");

    /// TODO: TUI
    /// TODO: Permission file

    loop {
        screen_clean();

        show_menu();

        let inp = input("\nInput: ").expect("Failed read stdin `input`");

        match inp.trim() {
            "q" => break,

            // Get password
            "1" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");
                let site = input("\nSite: ").expect("Failed read stdin `keyname`");

                // Retrieve key
                let file_manager = FileManager::new(secrets_path, &keyname.trim());
                let key = get_key_file(&file_manager, file_manager.key_name).expect("Failed getting key");
                let pass_manager = PasswordManager::new(key);

                // Get password
                let pass = retrieve_password_file(&file_manager, &pass_manager, &site.trim()).expect("Failed retrieve pass");
                println!("\nUsername: {}\nPassword: {}", pass.username, pass.password);

                // Continue
                let _ = input("Pass `Enter` to continue.").expect("Failed continue Key");
            },

            // Set password
            "2" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");
                let site = input("\nKeyname: ").expect("Failed read stdin `site`");
                let username = input("\nUsername: ").expect("Failed read stdin `username`");
                let pass = input("\nPassword: ").expect("Failed read stdin `password`");

                // Retrieve key
                let file_manager = FileManager::new(secrets_path, &keyname.trim());
                let key = get_key_file(&file_manager, file_manager.key_name).expect("Failed getting key");
                let mut pass_manager = PasswordManager::new(key);

                pass_manager.set_password(Password::new(&site.trim(), &username.trim(), &pass.trim()));

                set_password_file(&file_manager, &pass_manager).expect("Failed creating pass file");

                // Continue
                let _ = input("Pass `Enter` to continue.").expect("Failed continue Key");
            },

            // Generate new key
            "3" => {
                let keyname = input("\nKeyname: ").expect("Failed read stdin `keyname`");

                // set key name
                let file_manager = FileManager::new(secrets_path, &keyname.trim());

                generate_new_key_file(&file_manager).expect("Failed generating new key file.");

                // Continue
                let _ = input("Pass `Enter` to continue.").expect("Failed continue Key");
            },
            _ => continue
        }
    }
}
