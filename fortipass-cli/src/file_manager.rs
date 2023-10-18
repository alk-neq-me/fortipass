use std::fs;
use std::path::Path;


pub struct FileManager<'a> {
    pub secrets_path: &'a Path,
    pub key_name: &'a str
}

impl<'a> FileManager<'a> {
    pub fn new(path: &'a Path, key_name: &'a str) -> FileManager<'a> {
        if !path.is_dir() {
            fs::create_dir(path).expect("Failed create secrets dir.");
        }
        FileManager { secrets_path: path, key_name }
    }
}


// impl<'a, T> PasswordCreator for FileManager<'a, T> {
//     fn read_file_pass(site: &str, path: &Path, pass_manager: &PasswordManager) -> io::Result<Password> {
//         let mut fp = fs::File::open(&path)?;
//         let mut buffer = Vec::new();

//         fp.read_to_end(&mut buffer)?;

//         let decrypted = pass_manager.decrypt(&buffer).expect("Failed decrypt buffer.");

//         let content = String::from_utf8(decrypted).expect("Failed bytes extract.");

//         let username = &content.split(":").collect::<Vec<&str>>()[0];
//         let password = &content.split(":").collect::<Vec<&str>>()[1];

//         let pass = Password::new(site, username, password);

//         Ok(pass)
//     }

//     fn set_file_pass(pass: &str, path: &Path, pass_manager: &PasswordManager) -> io::Result<()> {
//         let mut fp = fs::File::create(&path)?;
//         let encrypted = pass_manager.encrypt(pass.as_bytes()).expect("Failed encrypted data.");

//         fp.write_all(&encrypted)?;

//         Ok(())
//     }
// }

