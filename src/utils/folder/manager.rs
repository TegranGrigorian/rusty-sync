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
        let dir = std::path::Path::new(path);

        if !dir.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Path not found: {}", path)));
        }

        for entry in fs::read_dir(path)? {
            match entry {
                Ok(entry) => {
                    if entry.file_type()?.is_dir() {
                        folders.push(entry.path().to_string_lossy().to_string());
                    }
                }
                Err(e) => {
                    eprintln!("Error reading directory entry in {}: {}", path, e);
                }
            }
        }
        Ok(folders)
    }

    pub fn traverse_folder(path: &str) -> io::Result<Vec<String>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                entries.push(path.to_string_lossy().to_string());
                entries.extend(Self::traverse_folder(&path.to_string_lossy())?);
            } else {
                entries.push(path.to_string_lossy().to_string());
            }
        }
        Ok(entries)
    }
}