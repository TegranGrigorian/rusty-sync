// use rusty_sync::utils::folder;
use rusty_sync::cloud::upload_files::FileUpload;
use rusty_sync::core::minio_util::MinioTests;
use rusty_sync::utils::git_util::GitDetector;
use rusty_sync::utils::git_util::GitIgnore;
use rusty_sync::utils::tree_serializer;
use std::path::PathBuf;
pub fn test_git_ignore() {
    let mut git_ignore = GitIgnore::new();
    // Use a relative path or current directory for cross-platform testing
    let test_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
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
    // Use a relative path for cross-platform compatibility
    let json_path = PathBuf::from("src/utils/EXAMPLE.struct_git.json");
    match tree_serializer::JsonValidator::validate_json_format(
        &json_path.as_path().to_string_lossy(),
    ) {
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

pub fn test_tree_gen() {
    // Use current directory for cross-platform testing
    let folder_path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .to_string();
    println!("Generating file tree for folder: {}", folder_path);
    match tree_serializer::ReadFileTree::generate_tree_as_string(&folder_path) {
        Ok(json_string) => println!("Generated JSON:\n{}", json_string),
        Err(e) => println!("Error generating file tree: {}", e),
    }
}

pub fn test_tree_gen_to_json(save_in_target: bool) {
    // Use test directory for cross-platform testing  
    let folder_path = "test";
    match tree_serializer::ReadFileTree::generate_tree(folder_path) {
        Ok(file_tree) => {
            let json_path = if save_in_target {
                format!("{}/EXAMPLE.struct_git.json", folder_path)
            } else {
                format!("EXAMPLE.struct_git.json")
            };
            match tree_serializer::JsonManager::write_to_json(&json_path, &file_tree) {
                Ok(()) => println!("Successfully wrote JSON to {}", json_path),
                Err(e) => println!("Error writing JSON to file: {}", e),
            }
        }
        Err(e) => println!("Error generating file tree: {}", e),
    }
}

pub fn test_mini_upload() {
    MinioTests::test_upload();
}

pub fn test_upload_from_json() {
    let file_upload = FileUpload {};
    let json_path = "";
    if let Err(e) = file_upload.read_file_tree(json_path) {
        eprintln!("Error reading file tree from JSON: {}", e);
    }
}
