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
