pub mod test;
fn main() {
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
    test::test_tree_gen_to_json(false);
}
