// this module will manage folders, create, delete and list folders

use std::fs;
use std::path::Path;
use std::io;
pub struct FolderManager;

impl FolderManager {
    pub fn create_folder(path: &str) -> io::Result<()> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn delete_folder(path: &str) -> io::Result<()> {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn list_folders(path: &str) -> io::Result<Vec<String>> {
        let mut folders = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                folders.push(entry.file_name().into_string().unwrap());
            }
        }
        Ok(folders)
    }
}