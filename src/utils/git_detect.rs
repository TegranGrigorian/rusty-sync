// this module detect for a .git directory and ignores it so we dont backup redunant data
// it will also check if the current directory is a git repository

use std::path::{Path, PathBuf};

pub struct GitDetector;

impl GitDetector {
    // Check if the given path is a git repository, input if a folder path
    pub fn is_git_repository(path: &Path) -> bool {
        let git_path = path.join(".git");
        git_path.exists() && git_path.is_dir()
    }
    pub fn print_directory(path: &Path) {
        if path.exists() {
            println!("The directory exists: {}", path.display());
        } else {
            println!("The directory does not exist: {}", path.display());
        }
    }
}
