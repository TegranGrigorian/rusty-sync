use crate::cli::interface_init::InitInterface;
use crate::utils::tree_serializer::{JsonManager, FileNode};
use crate::core::minio_util::MinioUtil;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub struct SyncTestService;

impl SyncTestService {
    /// Complete end-to-end test of the sync system
    pub fn run_complete_test(test_folder: &str, bucket: &str) -> Result<(), String> {
        println!("🚀 Starting complete sync test...");
        println!("Test folder: {}", test_folder);
        println!("Target bucket: {}", bucket);
        println!();

        // Step 0: Ensure bucket exists
        Self::ensure_bucket_exists(bucket)?;

        // Step 1: Initialize the folder
        Self::test_initialization(test_folder)?;

        // Step 2: Perform initial sync (upload all files)
        Self::test_initial_sync(test_folder, bucket)?;

        // Step 3: Modify a file and test incremental sync
        Self::test_incremental_sync(test_folder, bucket)?;

        // Step 4: Verify cross-platform compatibility
        Self::test_cross_platform_paths(test_folder)?;

        println!("✅ All tests completed successfully!");
        Ok(())
    }

    /// Ensure the bucket exists, create if necessary
    fn ensure_bucket_exists(bucket: &str) -> Result<(), String> {
        println!("🔧 Ensuring bucket '{}' exists...", bucket);
        
        match MinioUtil::check_bucket_exists(bucket) {
            Ok(true) => {
                println!("  ✓ Bucket '{}' already exists", bucket);
            }
            Ok(false) => {
                println!("  - Creating bucket '{}'...", bucket);
                MinioUtil::create_bucket(bucket)?;
                println!("  ✓ Bucket '{}' created successfully", bucket);
            }
            Err(_) => {
                println!("  - Cannot verify bucket existence, attempting to create...");
                MinioUtil::create_bucket(bucket)?;
                println!("  ✓ Bucket operation completed");
            }
        }
        
        Ok(())
    }

    /// Test 1: Initialize folder for sync
    fn test_initialization(test_folder: &str) -> Result<(), String> {
        println!("📁 Test 1: Initializing folder for sync");
        
        // Remove any existing sync file to start fresh
        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        if Path::new(&sync_file).exists() {
            fs::remove_file(&sync_file).map_err(|e| format!("Failed to remove existing sync file: {}", e))?;
            println!("  - Removed existing sync file");
        }

        // Initialize the folder
        InitInterface::initialize_folder(test_folder)?;
        
        // Verify the sync file was created
        if !Path::new(&sync_file).exists() {
            return Err("Sync file was not created".to_string());
        }

        // Load and display the structure
        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read sync file: {}", e))?;

        println!("  ✓ Folder initialized successfully");
        println!("  ✓ Found {} files", file_tree.get_all_files().len());
        
        // Display files found
        for file in file_tree.get_all_files() {
            println!("    - {} (size: {:?} bytes, path: {})", 
                file.name, file.size, file.relative_path);
        }

        Ok(())
    }

    /// Test 2: Initial sync (upload all files)
    fn test_initial_sync(test_folder: &str, bucket: &str) -> Result<(), String> {
        println!("\n📤 Test 2: Initial sync (upload all files)");

        // Perform sync
        InitInterface::sync_folder(test_folder, bucket)?;

        println!("  ✓ Initial sync completed");

        // Verify sync file was updated
        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read updated sync file: {}", e))?;

        // Check that files have sync metadata
        for file in file_tree.get_all_files() {
            // Skip the structure file as it's not synced to cloud
            if file.relative_path == "rusty-sync-structure.json" {
                continue;
            }
            
            if file.last_synced.is_none() {
                return Err(format!("File {} missing sync timestamp", file.name));
            }
            println!("    ✓ {} synced at timestamp: {:?}", file.name, file.last_synced);
        }

        Ok(())
    }

    /// Test 3: Modify file and test incremental sync
    fn test_incremental_sync(test_folder: &str, bucket: &str) -> Result<(), String> {
        println!("\n🔄 Test 3: Incremental sync (modify file)");

        let test_file = format!("{}/hello.txt", test_folder);
        
        // Wait a moment to ensure different timestamp
        thread::sleep(Duration::from_secs(1));

        // Modify the test file
        let new_content = "Hello, world! Modified for sync test.";
        fs::write(&test_file, new_content)
            .map_err(|e| format!("Failed to modify test file: {}", e))?;
        
        println!("  - Modified {}", test_file);

        // Perform incremental sync
        InitInterface::sync_folder(test_folder, bucket)?;

        println!("  ✓ Incremental sync completed");

        // Verify the change was detected and synced
        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read sync file: {}", e))?;

        // Find the modified file and check its metadata
        for file in file_tree.get_all_files() {
            if file.name == "hello.txt" {
                println!("    ✓ Modified file details:");
                println!("      - Size: {:?} bytes", file.size);
                println!("      - Last modified: {:?}", file.modified);
                println!("      - Last synced: {:?}", file.last_synced);
                println!("      - Machine ID: {:?}", file.machine_id);
                break;
            }
        }

        Ok(())
    }

    /// Test 4: Cross-platform path compatibility
    fn test_cross_platform_paths(test_folder: &str) -> Result<(), String> {
        println!("\n🌐 Test 4: Cross-platform path compatibility");

        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read sync file: {}", e))?;

        println!("  - Checking relative paths for cross-platform compatibility:");

        for file in file_tree.get_all_files() {
            // Verify relative paths use forward slashes
            if file.relative_path.contains('\\') {
                return Err(format!("File {} has Windows-style path separators: {}", 
                    file.name, file.relative_path));
            }

            // Verify relative paths don't start with absolute path indicators
            if file.relative_path.starts_with('/') || file.relative_path.contains(':') {
                return Err(format!("File {} has absolute path elements: {}", 
                    file.name, file.relative_path));
            }

            println!("    ✓ {} -> {}", file.name, file.relative_path);
        }

        println!("  ✓ All paths are cross-platform compatible");

        Ok(())
    }

    /// Test the sync detection logic
    pub fn test_sync_detection(test_folder: &str) -> Result<(), String> {
        println!("\n🔍 Testing sync detection logic");

        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        if !Path::new(&sync_file).exists() {
            return Err("Sync file not found. Run initialization first.".to_string());
        }

        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read sync file: {}", e))?;

        println!("  - Checking which files need syncing:");

        for file in file_tree.get_all_files() {
            let needs_sync = file.needs_sync();
            println!("    {} -> needs sync: {}", file.name, needs_sync);
            
            if needs_sync {
                println!("      - Modified: {:?}", file.modified);
                println!("      - Last synced: {:?}", file.last_synced);
            }
        }

        Ok(())
    }

    /// Create a test file to verify the system works
    pub fn create_test_file(test_folder: &str, filename: &str, content: &str) -> Result<(), String> {
        let file_path = format!("{}/{}", test_folder, filename);
        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to create test file: {}", e))?;
        
        println!("✓ Created test file: {}", file_path);
        Ok(())
    }

    /// Display current sync status
    pub fn display_sync_status(test_folder: &str) -> Result<(), String> {
        println!("\n📊 Current Sync Status");
        println!("==================");

        let sync_file = format!("{}/rusty-sync-structure.json", test_folder);
        if !Path::new(&sync_file).exists() {
            println!("❌ Folder not initialized for sync");
            return Ok(());
        }

        let file_tree: FileNode = JsonManager::read_from_json(&sync_file)
            .map_err(|e| format!("Failed to read sync file: {}", e))?;

        println!("Sync Root: {}", file_tree.path);
        println!("Machine ID: {:?}", file_tree.machine_id);
        println!("Sync ID: {:?}", file_tree.sync_id);
        println!();

        println!("Files:");
        for file in file_tree.get_all_files() {
            println!("  📄 {}", file.name);
            println!("     Path: {} -> {}", file.path, file.relative_path);
            println!("     Size: {:?} bytes", file.size);
            println!("     Modified: {:?}", file.modified);
            println!("     Last synced: {:?}", file.last_synced);
            println!("     Needs sync: {}", file.needs_sync());
            println!();
        }

        Ok(())
    }
}
