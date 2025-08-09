use rusty_sync::utils::file::manager::FileManager;
use rusty_sync::utils::folder::manager::FolderManager;
use rusty_sync::utils::git_detect::GitDetector;
use std::path::Path;
fn main() {
    // FileManager::create_file("./hello.txt",String::from("Hello, world!").as_str()).unwrap();

    let folder_path = "/home/tegran-grigorian/Documents/project/rusty-sync";
    // if GitDetector::is_git_repository(Path::new(file_path)) {
    //     println!("The directory is a git repository");
    // } else {
    //     println!("The directory is not a git repository");
    // }

    //test git detector
    if GitDetector::is_git_repository(Path::new(folder_path)) {
        println!("The directory is a git repository");
    } else {
        println!("The directory is not a git repository");
    }
}
