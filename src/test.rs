use rusty_sync::utils::git_util::GitIgnore;
use rusty_sync::utils::git_util::GitDetector;
use rusty_sync::utils::tree_serializer;
use std::path::PathBuf;

pub fn test_git_ignore() {
    let mut git_ignore = GitIgnore::new();
    let test_path = PathBuf::from("/home/tegran-grigorian/Documents");
    git_ignore.find_git_directories(&test_path);
    println!("Ignored .git directories:");
    // get remotes
    
    for path in git_ignore.ignored_paths {
        let remotes = GitDetector::get_git_remotes(&path);
        for remote in remotes {
            println!("Found remote '{}' in {}", remote, path.display());
        }
    }
}

pub fn test_json_structure() {
    let json_path = PathBuf::from("/home/tegran-grigorian/Documents/project/rusty-sync/src/utils/EXAMPLE.struct_git.json");
    match tree_serializer::JsonValidator::validate_json_format(&json_path.as_path().to_string_lossy()) {
        Ok(is_valid) => {
            if is_valid {
                println!("The JSON structure is valid.");
            } else {
                println!("The JSON structure is invalid.");
            }
        }
        Err(e) => println!("Error checking JSON structure: {}", e),
    }
}