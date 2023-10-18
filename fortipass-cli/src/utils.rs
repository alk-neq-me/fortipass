use std::io;

use crate::file_manager::FileManager;


pub trait Creator {
    type Manager;
    type Return;

    fn create(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str, content: Option<&str>) -> io::Result<()>;

    fn retrieve(&self, manager: &Self::Manager, file_manager: &FileManager, filename: &str) -> io::Result<Self::Return>;
}


