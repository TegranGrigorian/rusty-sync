// this module detect for a .git directory and ignores it so we dont backup redunant data
// it will also check if the current directory is a git repository

use std::path::{Path, PathBuf};
use std::fs;

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
// a struct needs to be created that will climb a directroy for all the .git directories and return a vector of paths
// these paths will be ignored including their contents
// a high directory like documents is given which is then scanned through for all the .gits

pub struct GitIgnore {
    pub ignored_paths: Vec<PathBuf>,
}

impl GitIgnore {
    pub fn new() -> Self {
        GitIgnore {
            ignored_paths: Vec::new(),
        }
    }

    // go through a directory and add all the .git directories to the ignored paths
    pub fn find_git_directories(&mut self, base_path: &Path) {
        if !base_path.exists() || !base_path.is_dir() {
            return;
        }

        // If this directory is already ignored, skip it
        if self.ignored_paths.contains(&base_path.to_path_buf()) {
            return;
        }

        // Check if the current directory is a git repository
        if GitDetector::is_git_repository(base_path) {
            self.ignored_paths.push(base_path.to_path_buf());
            // Do not recurse into this directory
            return;
        }

        for entry in fs::read_dir(base_path).unwrap_or_else(|_| panic!("Failed to read directory: {}", base_path.display())) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Recursively search in subdirectories
                    self.find_git_directories(&path);
                }
            } else {
                eprintln!("Failed to read entry in directory: {}", base_path.display());
            }
        }
    }
        //         if path.is_dir() {
        //             if GitDetector::is_git_repository(&path) {
        //                 self.ignored_paths.push(path.clone());
        //             }
        //             // Recursively search in subdirectories
        //             self.find_git_directories(&path);
        //         }
        //     }
        // }
}
