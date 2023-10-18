use std::path::Path;

use fortipass_core::passmanager::PasswordManager;

use crate::file_manager::FileManager;
use crate::utils::{get_key_file, retrieve_password_file};

mod utils;
mod file_manager;
mod password_creator;
mod key_creator;



fn main() {
    let secrets_path = Path::new("./.secrets");
    let file_manager = FileManager::new(secrets_path, "key");

    let key = get_key_file(&file_manager, file_manager.key_name).expect("Failed getting key");
    let pass_manager = PasswordManager::new(key);

    // pass_manager.set_password(Password::new("ig", "marco", "pass123"));

    // set_password_file(&file_manager, &pass_manager).expect("Failed setting password");

    let pass = retrieve_password_file(&file_manager, &pass_manager, "ig").expect("Failed retrieve pass");

    println!("{:?}", pass);
}
