use std::fs;
use std::path::Path;

use crate::utils::set_owner_perm;


pub struct FileManager<'a> {
    pub secrets_path: &'a Path,
    pub key_name: &'a str
}

impl<'a> FileManager<'a> {
    pub fn new(key_name: &'a str) -> FileManager<'a> {
        let path = Path::new("~/.secrets");
        if !path.is_dir() {
            fs::create_dir(path).expect("Failed create secrets dir.");
        }
        set_owner_perm(&path).expect("Failed set permission");
        FileManager { secrets_path: path, key_name }
    }
}
