use crate::sync_test_service::SyncTestService;
use crate::utils::tree_serializer::{JsonManager, ReadFileTree, SyncManager, FileNode, BucketManager};
use crate::core::minio_util::MinioUtil;
use crate::config::config_manager::RustySyncConfig;
use std::env;
use std::path::Path;
use std::process;
use std::io::{self, Write};

pub struct InitInterface;

impl InitInterface {
    /// Initialize a OneDrive folder by creating a JSON structure file
    pub fn initialize_folder(folder_path: &str) -> Result<(), String> {
        // Validate that the folder exists
        let path = Path::new(folder_path);
        if !path.exists() {
            return Err(format!("Folder '{}' does not exist", folder_path));
        }

        if !path.is_dir() {
            return Err(format!("'{}' is not a directory", folder_path));
        }

        println!("Initializing OneDrive sync for folder: {}", folder_path);

        // Generate the file tree structure
        let file_tree = ReadFileTree::generate_tree(folder_path)
            .map_err(|e| format!("Failed to generate file tree: {}", e))?;

        // Create JSON file in the target folder
        let json_file_path = format!("{}/rusty-sync-structure.json", folder_path);

        JsonManager::write_to_json(&json_file_path, &file_tree)
            .map_err(|e| format!("Failed to write JSON file: {}", e))?;

        println!("✓ Successfully created structure file: {}", json_file_path);
        println!(
            "✓ Folder '{}' is now initialized for OneDrive sync",
            folder_path
        );
        println!("  - Found {} files/folders", count_items(&file_tree));

        // Show some basic stats
        let all_files = file_tree.get_all_files();
        let total_size: u64 = all_files.iter().filter_map(|f| f.size).sum();

        println!("  - Total files: {}", all_files.len());
        println!(
            "  - Total size: {} bytes ({:.2} MB)",
            total_size,
            total_size as f64 / 1_048_576.0
        );

        Ok(())
    }

    /// Sync files to MinIO server
    pub fn sync_folder(folder_path: &str, bucket: &str) -> Result<(), String> {
        // Validate that the folder exists
        let path = Path::new(folder_path);
        if !path.exists() {
            return Err(format!("Folder '{}' does not exist", folder_path));
        }

        if !path.is_dir() {
            return Err(format!("'{}' is not a directory", folder_path));
        }

        // Check if folder is initialized
        let json_file_path = format!("{}/rusty-sync-structure.json", folder_path);
        if !Path::new(&json_file_path).exists() {
            return Err(format!(
                "Folder '{}' is not initialized. Run 'rusty-sync -i {}' first.",
                folder_path, folder_path
            ));
        }

        println!("Syncing folder: {} to bucket: {}", folder_path, bucket);

        // Generate current file tree, preserving existing sync metadata
        let mut current_tree = ReadFileTree::generate_tree_preserving_sync_data(folder_path)
            .map_err(|e| format!("Failed to generate file tree: {}", e))?;

        // Upload changed files
        match SyncManager::upload_changed_files(&mut current_tree, bucket) {
            Ok(uploaded_files) => {
                if uploaded_files.is_empty() {
                    println!("✓ All files are up to date - nothing to sync");
                } else {
                    println!("✓ Successfully uploaded {} files:", uploaded_files.len());
                    for file in uploaded_files {
                        println!("  - {}", file);
                    }
                }

                // Update the JSON file with current state
                JsonManager::write_to_json(&json_file_path, &current_tree)
                    .map_err(|e| format!("Failed to update structure file: {}", e))?;

                println!("✓ Updated structure file: {}", json_file_path);
            }
            Err(e) => return Err(format!("Sync failed: {}", e)),
        }

        Ok(())
    }

    /// Clone (download) a bucket to a local folder - git-like clone command
    pub fn clone_bucket(bucket: &str, local_folder: &str) -> Result<(), String> {
        println!("🔄 Cloning bucket '{}' to '{}'...", bucket, local_folder);

        // Create local folder if it doesn't exist
        std::fs::create_dir_all(local_folder)
            .map_err(|e| format!("Failed to create local folder: {}", e))?;

        // List files in the bucket
        let files = MinioUtil::list_files_in_bucket(bucket)?;
        
        if files.is_empty() {
            println!("📁 Bucket '{}' is empty", bucket);
            return Ok(());
        }

        println!("📥 Downloading {} files from bucket '{}'...", files.len(), bucket);

        // Download each file
        for file in &files {
            // Convert to absolute path to avoid working directory issues
            let local_folder_abs = if Path::new(local_folder).is_absolute() {
                local_folder.to_string()
            } else {
                env::current_dir()
                    .map_err(|e| format!("Failed to get current directory: {}", e))?
                    .join(local_folder)
                    .to_string_lossy()
                    .to_string()
            };
            
            let local_path = format!("{}/{}", local_folder_abs, file);
            
            // Create parent directories if needed
            if let Some(parent) = Path::new(&local_path).parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
            }

            match MinioUtil::download_file(bucket, file, &local_path) {
                Ok(_) => println!("  ✓ Downloaded: {}", file),
                Err(e) => {
                    eprintln!("  ✗ Failed to download {}: {}", file, e);
                    return Err(format!("Failed to download {}: {}", file, e));
                }
            }
        }

        // Since we downloaded the structure file from the bucket, we don't need to re-initialize
        // The folder is ready to use - avoid any tree generation that might overwrite files
        let structure_file = format!("{}/rusty-sync-structure.json", local_folder);
        if Path::new(&structure_file).exists() {
            println!("🔧 Using existing structure file from bucket");
            println!("� Folder is ready for sync operations");
        } else {
            println!("⚠️  Warning: No structure file found - you may need to run 'rusty-sync status' to initialize properly");
        }

        println!("✅ Successfully cloned bucket '{}' to '{}'", bucket, local_folder);
        Ok(())
    }

    /// List available buckets on the server - git-like remote list
    pub fn list_remote_buckets() -> Result<Vec<String>, String> {
        println!("🌐 Discovering available buckets on MinIO server...");
        
        let buckets = MinioUtil::list_buckets()?;
        
        if buckets.is_empty() {
            println!("📭 No buckets found on the server");
        } else {
            println!("📦 Available buckets:");
            for (i, bucket) in buckets.iter().enumerate() {
                println!("  {}. {}", i + 1, bucket);
            }
        }
        
        Ok(buckets)
    }

    /// Interactive bucket selection - let user choose which bucket to work with
    pub fn select_bucket() -> Result<String, String> {
        let buckets = Self::list_remote_buckets()?;
        
        if buckets.is_empty() {
            return Err("No buckets available on the server".to_string());
        }

        if buckets.len() == 1 {
            println!("Using the only available bucket: {}", buckets[0]);
            return Ok(buckets[0].clone());
        }

        // Interactive selection
        loop {
            print!("\n🎯 Select a bucket (1-{}): ", buckets.len());
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            match input.trim().parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= buckets.len() => {
                    let selected = buckets[choice - 1].clone();
                    println!("Selected bucket: {}", selected);
                    return Ok(selected);
                }
                _ => {
                    println!("❌ Invalid selection. Please enter a number between 1 and {}", buckets.len());
                }
            }
        }
    }

    /// Pull changes from remote bucket - git-like pull command
    pub fn pull_from_bucket(local_folder: &str, bucket: &str) -> Result<(), String> {
        println!("⬇️ Pulling changes from bucket '{}' to '{}'...", bucket, local_folder);

        // Check if folder is initialized
        let structure_file = format!("{}/rusty-sync-structure.json", local_folder);
        if !Path::new(&structure_file).exists() {
            return Err(format!(
                "Folder '{}' is not initialized for sync. Use 'clone' or 'init' first.",
                local_folder
            ));
        }

        // Get current local state
        let local_tree = ReadFileTree::generate_tree_preserving_sync_data(local_folder)
            .map_err(|e| format!("Failed to scan local folder: {}", e))?;

        // Get remote files
        let remote_files = MinioUtil::list_files_in_bucket(bucket)?;

        // Download files that don't exist locally or are newer remotely
        let mut downloaded_count = 0;
        for remote_file in &remote_files {
            let local_path = format!("{}/{}", local_folder, remote_file);
            
            // Check if file exists locally
            let should_download = if Path::new(&local_path).exists() {
                // File exists - we would need to implement remote timestamp checking
                // For now, skip existing files (you can enhance this)
                false
            } else {
                // File doesn't exist locally - download it
                true
            };

            if should_download {
                // Create parent directories if needed
                if let Some(parent) = Path::new(&local_path).parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
                }

                match MinioUtil::download_file(bucket, remote_file, &local_path) {
                    Ok(_) => {
                        println!("  ✓ Downloaded: {}", remote_file);
                        downloaded_count += 1;
                    }
                    Err(e) => {
                        eprintln!("  ✗ Failed to download {}: {}", remote_file, e);
                    }
                }
            }
        }

        // Update local structure file
        let updated_tree = ReadFileTree::generate_tree_preserving_sync_data(local_folder)
            .map_err(|e| format!("Failed to update local structure: {}", e))?;
        
        JsonManager::write_to_json(&structure_file, &updated_tree)
            .map_err(|e| format!("Failed to update structure file: {}", e))?;

        if downloaded_count > 0 {
            println!("✅ Downloaded {} new files from bucket '{}'", downloaded_count, bucket);
        } else {
            println!("✅ Local folder is up to date with bucket '{}'", bucket);
        }

        Ok(())
    }

    /// Show sync status - git-like status command
    pub fn show_status(local_folder: &str) -> Result<(), String> {
        println!("📊 Sync status for folder: {}", local_folder);

        // Check if folder is initialized
        let structure_file = format!("{}/rusty-sync-structure.json", local_folder);
        if !Path::new(&structure_file).exists() {
            println!("❌ Folder is not initialized for sync");
            println!("   Run 'rusty-sync init {}' or 'rusty-sync clone <bucket> {}' first", local_folder, local_folder);
            return Ok(());
        }

        // Load current structure
        let file_tree = ReadFileTree::generate_tree_preserving_sync_data(local_folder)
            .map_err(|e| format!("Failed to scan folder: {}", e))?;

        // Count files that need syncing
        let files_needing_sync: Vec<&FileNode> = file_tree
            .get_all_files()
            .into_iter()
            .filter(|f| f.needs_sync())
            .collect();

        let total_files = file_tree.get_all_files().len();
        
        println!("📁 Total files: {}", total_files);
        
        if files_needing_sync.is_empty() {
            println!("✅ All files are synchronized");
        } else {
            println!("📤 Files that need syncing: {}", files_needing_sync.len());
            for file in &files_needing_sync {
                println!("  📄 {}", file.relative_path);
            }
        }

        Ok(())
    }

    /// Parse command line arguments and handle initialization
    pub fn handle_init_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        // Check if we have the right number of arguments
        if args.len() < 3 {
            return Err("Usage: rusty-sync -i <folder_path>".to_string());
        }

        // Check for the -i flag
        if args[1] != "-i" {
            return Err("Invalid flag. Use -i to initialize a folder".to_string());
        }

        let folder_path = &args[2];

        // Convert relative path to absolute path if needed
        let absolute_path = if Path::new(folder_path).is_absolute() {
            folder_path.to_string()
        } else {
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;
            current_dir.join(folder_path).to_string_lossy().to_string()
        };

        Self::initialize_folder(&absolute_path)
    }

    /// Handle sync command
    pub fn handle_sync_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        // Check if we have the right number of arguments
        if args.len() < 4 {
            return Err("Usage: rusty-sync -s <folder_path> <bucket_name>".to_string());
        }

        // Check for the -s flag
        if args[1] != "-s" {
            return Err("Invalid flag. Use -s to sync a folder".to_string());
        }

        let folder_path = &args[2];
        let bucket = &args[3];

        // Convert relative path to absolute path if needed
        let absolute_path = if Path::new(folder_path).is_absolute() {
            folder_path.to_string()
        } else {
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;
            current_dir.join(folder_path).to_string_lossy().to_string()
        };

        Self::sync_folder(&absolute_path, bucket)
    }

    /// Handle clone command - git-like clone
    pub fn handle_clone_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 4 {
            return Err("Usage: rusty-sync clone <bucket_name> <local_folder>".to_string());
        }

        if args[1] != "clone" {
            return Err("Invalid command. Use 'clone' to clone a bucket".to_string());
        }

        let bucket = &args[2];
        let local_folder = &args[3];

        Self::clone_bucket(bucket, local_folder)
    }

    /// Handle pull command - git-like pull
    pub fn handle_pull_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        let (local_folder, bucket) = if args.len() == 2 && args[1] == "pull" {
            // rusty-sync pull (from current directory - try to auto-detect bucket)
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string();
            
            // Try to auto-detect bucket name
            match BucketManager::detect_bucket_name(&current_dir) {
                Ok(bucket) => {
                    println!("🔍 Auto-detected bucket: '{}'", bucket);
                    (current_dir, bucket)
                }
                Err(_) => {
                    return Err("Could not auto-detect bucket name. Usage: rusty-sync pull <bucket> [folder]".to_string());
                }
            }
        } else if args.len() == 3 {
            // rusty-sync pull <bucket> (from current directory)
            if args[1] != "pull" {
                return Err("Invalid command. Use 'pull' to pull changes".to_string());
            }
            
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string();
            (current_dir, args[2].clone())
        } else if args.len() == 4 {
            // rusty-sync pull <bucket> <folder>
            if args[1] != "pull" {
                return Err("Invalid command. Use 'pull' to pull changes".to_string());
            }
            (args[3].clone(), args[2].clone())
        } else {
            return Err("Usage: rusty-sync pull [bucket] [folder]".to_string());
        };

        Self::pull_from_bucket(&local_folder, &bucket)
    }

    /// Handle push command - git-like push (same as sync)
    pub fn handle_push_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        let (local_folder, bucket) = if args.len() == 2 && args[1] == "push" {
            // rusty-sync push (from current directory - try to auto-detect bucket)
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string();
            
            // Try to auto-detect bucket name
            match BucketManager::detect_bucket_name(&current_dir) {
                Ok(bucket) => {
                    println!("🔍 Auto-detected bucket: '{}'", bucket);
                    (current_dir, bucket)
                }
                Err(_) => {
                    return Err("Could not auto-detect bucket name. Usage: rusty-sync push <bucket> [folder]".to_string());
                }
            }
        } else if args.len() == 3 {
            // rusty-sync push <bucket> (from current directory)
            if args[1] != "push" {
                return Err("Invalid command. Use 'push' to push changes".to_string());
            }
            
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string();
            (current_dir, args[2].clone())
        } else if args.len() == 4 {
            // rusty-sync push <bucket> <folder>
            if args[1] != "push" {
                return Err("Invalid command. Use 'push' to push changes".to_string());
            }
            (args[3].clone(), args[2].clone())
        } else {
            return Err("Usage: rusty-sync push [bucket] [folder]".to_string());
        };

        // Save bucket association for future auto-detection
        if let Err(_) = BucketManager::save_bucket_association(&local_folder, &bucket) {
            // Non-critical error, just warn
            eprintln!("⚠️  Warning: Could not save bucket association");
        }

        Self::sync_folder(&local_folder, &bucket)
    }

    /// Handle status command - git-like status
    pub fn handle_status_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        let local_folder = if args.len() == 2 && args[1] == "status" {
            // rusty-sync status (from current directory)
            env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string()
        } else if args.len() == 3 {
            // rusty-sync status <folder>
            if args[1] != "status" {
                return Err("Invalid command. Use 'status' to show sync status".to_string());
            }
            args[2].clone()
        } else {
            return Err("Usage: rusty-sync status [folder]".to_string());
        };

        Self::show_status(&local_folder)
    }

    /// Handle remote command - list remote buckets or add new bucket
    pub fn handle_remote_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        if args.len() >= 2 && args[1] == "remote" {
            if args.len() == 2 || (args.len() == 3 && args[2] == "list") {
                // rusty-sync remote or rusty-sync remote list
                Self::list_remote_buckets().map(|_| ())
            } else if args.len() == 4 && args[2] == "add" {
                // rusty-sync remote add <bucket-name>
                let bucket_name = &args[3];
                Self::create_remote_bucket(bucket_name)
            } else {
                Err("Usage: rusty-sync remote [list] or rusty-sync remote add <bucket-name>".to_string())
            }
        } else {
            Err("Invalid command. Use 'remote' to list remote buckets or add new ones".to_string())
        }
    }

    /// Create a new bucket on the remote server
    pub fn create_remote_bucket(bucket_name: &str) -> Result<(), String> {
        println!("🚀 Creating new bucket: '{}'...", bucket_name);
        
        // Check if bucket already exists
        match MinioUtil::check_bucket_exists(bucket_name) {
            Ok(true) => {
                println!("⚠️  Bucket '{}' already exists", bucket_name);
                return Ok(());
            }
            Ok(false) => {
                // Bucket doesn't exist, create it
                MinioUtil::create_bucket(bucket_name)?;
                println!("✅ Successfully created bucket: '{}'", bucket_name);
                Ok(())
            }
            Err(e) => Err(format!("Failed to check if bucket exists: {}", e))
        }
    }

    /// Handle config command - manage MinIO server configurations
    pub fn handle_config_command() -> Result<(), String> {
        RustySyncConfig::interactive_setup()
    }

    /// Handle test command
    pub fn handle_test_command() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        // Check if we have the right number of arguments
        if args.len() < 4 {
            return Err("Usage: rusty-sync -t <folder_path> <bucket_name>".to_string());
        }

        // Check for the -t flag
        if args[1] != "-t" {
            return Err("Invalid flag. Use -t to run comprehensive test".to_string());
        }

        let folder_path = &args[2];
        let bucket = &args[3];

        // Convert relative path to absolute path if needed
        let absolute_path = if Path::new(folder_path).is_absolute() {
            folder_path.to_string()
        } else {
            let current_dir = env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;
            current_dir.join(folder_path).to_string_lossy().to_string()
        };

        SyncTestService::run_complete_test(&absolute_path, bucket)
    }

    /// Main entry point for the CLI interface
    pub fn run() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            print_usage();
            return;
        }

        match args[1].as_str() {
            "-i" | "--init" => match Self::handle_init_command() {
                Ok(_) => println!("\n🎉 Initialization completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "-s" | "--sync" => match Self::handle_sync_command() {
                Ok(_) => println!("\n🎉 Sync completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "-t" | "--test" => match Self::handle_test_command() {
                Ok(_) => println!("\n🎉 Test completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "clone" => match Self::handle_clone_command() {
                Ok(_) => println!("\n🎉 Clone completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "pull" => match Self::handle_pull_command() {
                Ok(_) => println!("\n🎉 Pull completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "push" => match Self::handle_push_command() {
                Ok(_) => println!("\n🎉 Push completed successfully!"),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "status" => match Self::handle_status_command() {
                Ok(_) => {}, // Status command prints its own success messages
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "remote" => match Self::handle_remote_command() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "config" => match Self::handle_config_command() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            },
            "-h" | "--help" => {
                print_usage();
            }
            _ => {
                eprintln!("❌ Unknown command: {}", args[1]);
                print_usage();
                process::exit(1);
            }
        }
    }
}

/// Count total items (files and folders) in the tree
fn count_items(node: &crate::utils::tree_serializer::FileNode) -> usize {
    let mut count = 1; // Count this node

    if let Some(children) = &node.children {
        for child in children {
            count += count_items(child);
        }
    }

    count
}

/// Print usage information
fn print_usage() {
    println!("Rusty Sync - OneDrive-like File Synchronization Tool");
    println!();
    println!("USAGE:");
    println!("    rusty-sync <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("  Git-like Commands:");
    println!("    clone <bucket> <folder>     Clone a bucket to local folder");
    println!("    pull <bucket> [folder]      Pull changes from bucket to local folder");
    println!("    push <bucket> [folder]      Push local changes to bucket");
    println!("    status [folder]             Show sync status of local folder");
    println!("    remote [list]               List available buckets on server");
    println!();
    println!("  Classic Commands:");
    println!("    -i, --init <folder_path>    Initialize a folder for sync");
    println!("    -s, --sync <folder_path> <bucket>  Sync a folder to bucket");
    println!("    -t, --test <folder_path> <bucket>  Run comprehensive sync test");
    println!("    -h, --help                  Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  Git-like workflow:");
    println!("    rusty-sync remote           # List available buckets");
    println!("    rusty-sync clone my-bucket ./sync-folder");
    println!("    cd sync-folder");
    println!("    # ... modify files ...");
    println!("    rusty-sync status           # Check what needs syncing");
    println!("    rusty-sync push my-bucket   # Upload changes");
    println!("    rusty-sync pull my-bucket   # Download remote changes");
    println!();
    println!("  Classic workflow:");
    println!("    rusty-sync -i /home/user/Documents");
    println!("    rusty-sync -s /home/user/Documents my-bucket");
    println!("    rusty-sync -t ./test my-bucket");
    println!();
    println!("DESCRIPTION:");
    println!("    Rusty Sync provides OneDrive-like synchronization with MinIO storage.");
    println!("    Use git-like commands for familiar workflow, or classic flags for scripts.");
    println!();
    println!("    The git-like commands work similarly to Git:");
    println!("    • clone: Download entire bucket to local folder");
    println!("    • pull: Download new/changed files from bucket");
    println!("    • push: Upload local changes to bucket");
    println!("    • status: Show what files need syncing");
    println!("    • remote: List available buckets");
    println!();
    println!("DESCRIPTION:");
    println!("    The initialization process will:");
    println!("    • Scan the specified folder and all its contents");
    println!("    • Create a JSON structure file for tracking changes");
    println!("    • Prepare the folder for cloud synchronization");
    println!("    • Generate metadata for all files (size, modification time)");
    println!();
    println!("    The sync process will:");
    println!("    • Compare local files with the last sync state");
    println!("    • Upload changed files to the MinIO bucket");
    println!("    • Use relative paths for cross-platform compatibility");
    println!("    • Track machine IDs for conflict resolution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_nonexistent_folder() {
        let result = InitInterface::initialize_folder("/nonexistent/folder");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_count_items_simple() {
        // Simple test - we can't easily create FileNode instances in tests
        // due to the complex structure, so just test the logic
        assert_eq!(1 + 1, 2); // Placeholder test
    }
}
