pub mod test;
use rusty_sync::cli::interface_init::InitInterface;

fn main() {
    // Check if we're running as a CLI tool
    let args: std::vec::Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // Run CLI interface
        InitInterface::run();
        return;
    }

    //older code
    // Default behavior for testing (when no args provided)
    // println!("Running in test mode...");
    
    // FileManager::create_file("./hello.txt",String::from("Hello, world!").as_str()).unwrap();

    // let folder_path: &'static str = "/home/tegran-grigorian/Documents/project/rusty-sync";

    // if GitDetector::is_git_repository(Path::new(file_path)) {
    //     println!("The directory is a git repository");
    // } else {
    //     println!("The directory is not a git repository");
    // }

    //test git detector
    // if GitDetector::is_git_repository(Path::new(folder_path)) {
    //     println!("The directory is a git repository");
    // } else {
    //     println!("The directory is not a git repository");
    // }

    // test::test_json_structure();
    // test::test_tree_gen_to_json(true);
    // test::test_mini_upload(); //Test works
}
