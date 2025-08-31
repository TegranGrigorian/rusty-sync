// this code reads into the src/core/minio python suite and runs commands into it
use std::process::Command; // run python script
use std::path::PathBuf;
use crate::config::config_manager::RustySyncConfig;

/// Find the project root directory by looking for Cargo.toml
fn find_project_root() -> Result<PathBuf, String> {
    let mut current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(current_dir);
        }
        
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err("Could not find project root (Cargo.toml not found)".to_string());
        }
    }
}

/// Set up environment variables from config before running Python commands
fn setup_minio_env() -> Result<(), String> {
    let config = RustySyncConfig::load()?;
    config.export_to_env()?;
    Ok(())
}

/// Find MinIO Python scripts - check system installation first, then development location
fn find_minio_scripts() -> Result<(PathBuf, PathBuf, PathBuf), String> {
    // System installation path (for deb package)
    let system_minio_dir = PathBuf::from("/usr/local/share/rusty-sync");
    let system_python_exe = system_minio_dir.join(".venv/bin/python");
    let system_main_py = system_minio_dir.join("main.py");
    
    if system_minio_dir.exists() && system_main_py.exists() {
        return Ok((system_minio_dir, system_python_exe, system_main_py));
    }
    
    // Development path (fallback for source builds)
    let project_root = find_project_root()?;
    let dev_minio_dir = project_root.join("src/core/minio");
    let dev_python_exe = dev_minio_dir.join(".venv/bin/python");
    let dev_main_py = dev_minio_dir.join("main.py");
    
    if dev_minio_dir.exists() && dev_main_py.exists() {
        return Ok((dev_minio_dir, dev_python_exe, dev_main_py));
    }
    
    Err("Could not find MinIO Python scripts in system (/usr/local/share/rusty-sync) or development (src/core/minio) locations".to_string())
}

pub struct MinioUtil {}

impl MinioUtil {
    // example python command python main.py --upload /home/tegran-grigorian/Documents/Projects/rusty-sync/hi.mp3 rusty-sync hi.mp3

    pub fn upload_file(file_path: &str, bucket: &str, object_name: &str) -> Result<(), String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir) // Set working directory
            .arg(&main_py)
            .arg("--upload")
            .arg(file_path)
            .arg(bucket)
            .arg(object_name)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            println!("Successfully uploaded {} to bucket {}", object_name, bucket);
            Ok(())
        } else {
            Err(format!(
                "Python script error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    /// Create a bucket if it doesn't exist
    pub fn create_bucket(bucket: &str) -> Result<(), String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir)
            .arg(&main_py)
            .arg("--create-bucket")
            .arg(bucket)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            println!("Bucket '{}' is ready", bucket);
            Ok(())
        } else {
            Err(format!(
                "Python script error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    /// Check if bucket exists
    pub fn check_bucket_exists(bucket: &str) -> Result<bool, String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir)
            .arg(&main_py)
            .arg("--check-bucket")
            .arg(bucket)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.contains("exists"))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("does not exist") {
                Ok(false)
            } else {
                Err(format!("Python script error: {}", stderr))
            }
        }
    }

    /// Download a file from MinIO
    pub fn download_file(bucket: &str, object_name: &str, local_path: &str) -> Result<(), String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir)
            .arg(&main_py)
            .arg("--download")
            .arg(bucket)
            .arg(object_name)
            .arg(local_path)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            println!("Successfully downloaded {} from bucket {}", object_name, bucket);
            Ok(())
        } else {
            Err(format!(
                "Python script error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    /// List all buckets available on the MinIO server
    pub fn list_buckets() -> Result<Vec<String>, String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir)
            .arg(&main_py)
            .arg("--list-buckets")
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse bucket names from output (assuming they're listed one per line)
            let buckets: Vec<String> = stdout
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.contains("Connection test"))
                .filter_map(|line| {
                    // Extract bucket names from various output formats
                    if line.contains("Available buckets:") {
                        // Parse JSON-like format: Available buckets: ['bucket1', 'bucket2']
                        let start = line.find('[')? + 1;
                        let end = line.find(']')?;
                        let buckets_str = &line[start..end];
                        Some(buckets_str.split(',')
                            .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
                            .collect::<Vec<String>>())
                    } else {
                        // Single bucket per line
                        Some(vec![line.trim().to_string()])
                    }
                })
                .flatten()
                .filter(|bucket| !bucket.is_empty())
                .collect();
            
            Ok(buckets)
        } else {
            Err(format!(
                "Python script error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    /// List all files in a bucket
    pub fn list_files_in_bucket(bucket: &str) -> Result<Vec<String>, String> {
        setup_minio_env()?; // Setup config before running Python
        
        let (minio_dir, python_exe, main_py) = find_minio_scripts()?;
        
        let output = Command::new(&python_exe)
            .current_dir(&minio_dir)
            .arg(&main_py)
            .arg("--list")
            .arg(bucket)
            .output()
            .map_err(|e| format!("Failed to execute python script: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse file names from output
            let files: Vec<String> = stdout
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.contains("Connection test"))
                .filter_map(|line| {
                    if line.contains("Files in bucket:") {
                        // Parse JSON-like format: Files in bucket: ['file1', 'file2']
                        let start = line.find('[')? + 1;
                        let end = line.find(']')?;
                        let files_str = &line[start..end];
                        Some(files_str.split(',')
                            .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
                            .collect::<Vec<String>>())
                    } else {
                        None
                    }
                })
                .flatten()
                .filter(|file| !file.is_empty())
                .collect();
            
            Ok(files)
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
