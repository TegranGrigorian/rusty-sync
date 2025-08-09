use rusty_sync::utils::git_util::GitIgnore;
use rusty_sync::utils::git_util::GitDetector;
use std::path::PathBuf;

pub fn test_git_ignore() {
    let mut git_ignore = GitIgnore::new();
    let test_path = PathBuf::from("/home/tegran-grigorian/Documents");
    git_ignore.find_git_directories(&test_path);
    println!("Ignored .git directories:");
    for path in git_ignore.ignored_paths {
        println!("{}", path.display());
    }
}
