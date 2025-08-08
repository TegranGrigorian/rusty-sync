// file manager, this will create, read, write and delete files

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct FileManager;

impl FileManager {
    pub fn create_file(path: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn read_file(path: &str) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn write_file(path: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn delete_file(path: &str) -> io::Result<()> {
        if Path::new(path).exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}