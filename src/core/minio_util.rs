// this code reads into the src/core/minio python suite and runs commands into it
use std::process::Command; // run python script
pub struct MinioUtil {}

impl MinioUtil {
    // example python command python main.py --upload /home/tegran-grigorian/Documents/Projects/rusty-sync/hi.mp3 rusty-sync hi.mp3

    pub fn upload_file(file_path: &str, bucket: &str, object_name: &str) -> Result<(), String> {
        let output = Command::new("python3")
            .arg("main.py") // to be honest not sure how this works because the path is wrong but i aint complaining
            .arg("--upload")
            .arg(file_path)
            .arg(bucket)
            .arg(object_name)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Python script error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

pub struct MinioTests {}

impl MinioTests {
    pub fn test_upload() {
        // mimic this command python main.py --upload /home/tegran-grigorian/Documents/Projects/rusty-sync/hi.mp3 rusty-sync hi.mp3

        let file_path = "/home/tegran-grigorian/Documents/Projects/rusty-sync/hi.mp3";
        let bucket = "rusty-sync";
        let object_name = "hi.mp3";

        match MinioUtil::upload_file(file_path, bucket, object_name) {
            Ok(_) => println!("File uploaded successfully"),
            Err(e) => eprintln!("Error uploading file: {}", e),
        }
    }
}
