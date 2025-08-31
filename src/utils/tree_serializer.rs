// this module will be able to read and write json files
// it willl read the files in the correct format and return an output
// it will also write the files in the correct format
// this format will be stated in a documentation file

//kinda weird but we will have a json module in tihs file since i hope this is the only one that will needd it (im prolly worng)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::SystemTime;

/// Generate a unique machine ID based on hostname and user
fn get_machine_id() -> String {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    format!("{}@{}", username, hostname)
}

/// Convert absolute path to relative path from sync root
fn to_relative_path(absolute_path: &str, sync_root: &str) -> String {
    let abs_path = Path::new(absolute_path);
    let root_path = Path::new(sync_root);

    match abs_path.strip_prefix(root_path) {
        Ok(relative) => {
            // Use forward slashes for cross-platform compatibility
            relative.to_string_lossy().replace('\\', "/")
        }
        Err(_) => {
            // Fallback: just use the filename
            abs_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        }
    }
}

/// Generate a unique sync ID for this sync session
fn generate_sync_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("sync_{}", timestamp)
}

// definition sof structs for the nodes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub r#type: String,        // "file" or "folder"
    pub path: String,          // Full absolute path to the file/folder
    pub relative_path: String, // Relative path from sync root (cross-platform)
    pub children: Option<Vec<FileNode>>,
    pub git_remote: Option<String>,
    // File-specific metadata for sync tracking
    pub size: Option<u64>,     // File size in bytes (None for folders)
    pub modified: Option<u64>, // Last modified time as Unix timestamp (None for folders or if unavailable)
    pub hash: Option<String>,  // File hash for change detection (future use)
    // Sync metadata
    pub last_synced: Option<u64>,   // Last sync timestamp
    pub sync_id: Option<String>,    // Unique ID for this sync folder
    pub machine_id: Option<String>, // Machine that last modified this file
}

impl FileNode {
    /// Check if this node represents a file
    pub fn is_file(&self) -> bool {
        self.r#type == "file"
    }

    /// Check if this node represents a folder
    pub fn is_folder(&self) -> bool {
        self.r#type == "folder"
    }

    /// Get all file nodes recursively from this tree
    pub fn get_all_files(&self) -> Vec<&FileNode> {
        let mut files = Vec::new();
        self.collect_files(&mut files);
        files
    }

    fn collect_files<'a>(&'a self, files: &mut Vec<&'a FileNode>) {
        if self.is_file() {
            files.push(self);
        }

        if let Some(children) = &self.children {
            for child in children {
                child.collect_files(files);
            }
        }
    }

    /// Find a file/folder by path
    pub fn find_by_path(&self, target_path: &str) -> Option<&FileNode> {
        if self.path == target_path {
            return Some(self);
        }

        if let Some(children) = &self.children {
            for child in children {
                if let Some(found) = child.find_by_path(target_path) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Check if this file has been modified since the given timestamp
    pub fn is_modified_since(&self, timestamp: u64) -> bool {
        if let Some(modified) = self.modified {
            modified > timestamp
        } else {
            false // If we can't determine modification time, assume not modified
        }
    }

    /// Check if this file needs to be synced (has changes since last sync)
    pub fn needs_sync(&self) -> bool {
        if self.is_folder() {
            return false; // Only sync files, not folders
        }

        // Don't sync the structure file itself
        if self.relative_path == "rusty-sync-structure.json" {
            return false;
        }

        match (self.modified, self.last_synced) {
            (Some(modified), Some(last_synced)) => modified > last_synced,
            (Some(_), None) => true, // Never synced before
            _ => false,              // Can't determine, assume no sync needed
        }
    }

    /// Get the relative path for cross-platform compatibility
    pub fn get_cross_platform_path(&self) -> &str {
        &self.relative_path
    }

    /// Check if this file is newer than another version
    pub fn is_newer_than(&self, other: &FileNode) -> bool {
        match (self.modified, other.modified) {
            (Some(self_mod), Some(other_mod)) => self_mod > other_mod,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => false,
        }
    }

    /// Compare two file nodes for conflict resolution
    pub fn compare_for_sync(&self, other: &FileNode) -> SyncAction {
        // Same file (by relative path)
        if self.relative_path != other.relative_path {
            return SyncAction::NoAction;
        }

        // Check if sizes are different
        if self.size != other.size {
            return if self.is_newer_than(other) {
                SyncAction::Upload
            } else {
                SyncAction::Download
            };
        }

        // If sizes are same, check modification time
        match (self.modified, other.modified) {
            (Some(self_mod), Some(other_mod)) => {
                if self_mod > other_mod {
                    SyncAction::Upload
                } else if other_mod > self_mod {
                    SyncAction::Download
                } else {
                    SyncAction::NoAction
                }
            }
            (Some(_), None) => SyncAction::Upload,
            (None, Some(_)) => SyncAction::Download,
            (None, None) => SyncAction::NoAction,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncAction {
    Upload,   // Local file is newer, upload to cloud
    Download, // Cloud file is newer, download from cloud
    NoAction, // Files are in sync or no change needed
}

pub struct JsonManager;

impl JsonManager {
    pub fn write_to_json<T: Serialize>(path: &str, data: &T) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, data)?; // Use pretty formatting
        Ok(())
    }

    pub fn read_from_json<T: for<'de> Deserialize<'de>>(path: &str) -> io::Result<T> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data: T = serde_json::from_str(&content)?;
        Ok(data)
    }
}

// need module to determine if the json file is valid and in the correct format for this program to read
pub struct JsonValidator;
impl JsonValidator {
    pub fn validate_json_format(path: &str) -> io::Result<bool> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let result: Result<FileNode, serde_json::Error> = serde_json::from_str(&content);
        match result {
            Ok(_node) => {
                // Optionally, add more checks here (e.g., node.type is \"folder\" at root)
                Ok(true)
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid JSON format or structure",
            )),
        }
    }
}

pub struct ReadFileTree;

impl ReadFileTree {
    pub fn generate_tree(path: &str) -> io::Result<FileNode> {
        Self::generate_tree_with_sync_data(path, &generate_sync_id())
    }

    /// Generate tree and merge with existing sync metadata if available
    pub fn generate_tree_preserving_sync_data(path: &str) -> io::Result<FileNode> {
        let mut new_tree = Self::generate_tree(path)?;

        // Try to load existing sync metadata
        let structure_file = format!("{}/rusty-sync-structure.json", path);
        if let Ok(existing_tree) = JsonManager::read_from_json::<FileNode>(&structure_file) {
            Self::merge_sync_metadata(&mut new_tree, &existing_tree);
        }

        Ok(new_tree)
    }

    /// Merge sync metadata from existing tree into new tree
    fn merge_sync_metadata(new_tree: &mut FileNode, existing_tree: &FileNode) {
        // Merge metadata for this node if it's a file
        if new_tree.is_file()
            && existing_tree.is_file()
            && new_tree.relative_path == existing_tree.relative_path
        {
            // Only preserve sync metadata if the file hasn't been modified since last sync
            if new_tree.modified == existing_tree.modified {
                new_tree.last_synced = existing_tree.last_synced;
                new_tree.sync_id = existing_tree.sync_id.clone();
                new_tree.machine_id = existing_tree.machine_id.clone();
            }
        }

        // Recursively merge children
        if let (Some(new_children), Some(existing_children)) =
            (&mut new_tree.children, &existing_tree.children)
        {
            for new_child in new_children {
                if let Some(existing_child) = existing_children
                    .iter()
                    .find(|c| c.relative_path == new_child.relative_path)
                {
                    Self::merge_sync_metadata(new_child, existing_child);
                }
            }
        }
    }

    pub fn generate_tree_with_sync_data(path: &str, sync_id: &str) -> io::Result<FileNode> {
        use crate::utils::git_util::GitDetector;

        let root_path = std::path::Path::new(path);
        let machine_id = get_machine_id();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // // write the real file path that the json is targetting
        let json_file_path = format!("{}/EXAMPLE.struct_git.json", path);

        // Write the real file path at the top of the json so when we upload we know where the root folder is on the system
        let json_content = format!(r#"{{ "root": "{}" }}"#, root_path.display());

        //push content to json file, dont save it and stop writting
        std::fs::write(&json_file_path, json_content)?;

        // Check if the root folder is a Git repository
        if GitDetector::is_git_repository(root_path) {
            let git_remotes = GitDetector::get_git_remotes(root_path);
            return Ok(FileNode {
                name: root_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                r#type: "folder".to_string(),
                path: root_path.to_string_lossy().to_string(),
                relative_path: ".".to_string(),
                children: None,
                git_remote: git_remotes.get(0).cloned(), // Use the first remote if available
                size: None,                              // Folders don't have size
                modified: None, // Could add folder modification time if needed
                hash: None,     // Git repos don't need file hashes
                last_synced: Some(current_time),
                sync_id: Some(sync_id.to_string()),
                machine_id: Some(machine_id),
            });
        }

        let mut children = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name().to_string_lossy().to_string();

            if entry_path.is_dir() {
                if GitDetector::is_git_repository(&entry_path) {
                    let git_remotes = GitDetector::get_git_remotes(&entry_path);
                    children.push(FileNode {
                        name: entry_name,
                        r#type: "folder".to_string(),
                        path: entry_path.to_string_lossy().to_string(),
                        relative_path: to_relative_path(&entry_path.to_string_lossy(), path),
                        children: None,
                        git_remote: git_remotes.get(0).cloned(),
                        size: None,
                        modified: None,
                        hash: None,
                        last_synced: Some(current_time),
                        sync_id: Some(sync_id.to_string()),
                        machine_id: Some(machine_id.clone()),
                    });
                } else {
                    // Recursively process non-Git folders
                    children.push(Self::generate_tree_with_sync_data(
                        &entry_path.to_string_lossy(),
                        sync_id,
                    )?);
                }
            } else if entry_path.is_file() {
                // Get file metadata
                let metadata = entry_path.metadata().ok();
                let size = metadata.as_ref().map(|m| m.len());
                let modified = metadata
                    .as_ref()
                    .and_then(|m| m.modified().ok())
                    .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|duration| duration.as_secs());

                // Add files to the children list
                children.push(FileNode {
                    name: entry_name,
                    r#type: "file".to_string(),
                    path: entry_path.to_string_lossy().to_string(),
                    relative_path: to_relative_path(&entry_path.to_string_lossy(), path),
                    children: None,
                    git_remote: None,
                    size,
                    modified,
                    hash: None,        // Will be computed later when needed for sync
                    last_synced: None, // No files are synced initially - will be set after successful upload
                    sync_id: Some(sync_id.to_string()),
                    machine_id: Some(machine_id.clone()),
                });
            }
        }

        Ok(FileNode {
            name: root_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            r#type: "folder".to_string(),
            path: root_path.to_string_lossy().to_string(),
            relative_path: ".".to_string(),
            children: Some(children),
            git_remote: None,
            size: None,
            modified: None,
            hash: None,
            last_synced: Some(current_time),
            sync_id: Some(sync_id.to_string()),
            machine_id: Some(machine_id),
        })
    }
    pub fn generate_tree_as_string(path: &str) -> io::Result<String> {
        let tree = Self::generate_tree(path)?;
        let json_string = serde_json::to_string_pretty(&tree)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(json_string)
    }

    /// Generate tree and save it to a JSON file
    pub fn generate_tree_to_file(path: &str, output_file: &str) -> io::Result<()> {
        let tree = Self::generate_tree(path)?;
        JsonManager::write_to_json(output_file, &tree)?;
        println!("Successfully wrote JSON to {}", output_file);
        Ok(())
    }
}

pub struct JsonGenerator;

impl JsonGenerator {
    // generate a json in the format of the examples but take inputs of actual file paths and folder paths
}

/// Sync manager for handling file synchronization with MinIO
pub struct SyncManager;

impl SyncManager {
    /// Upload files that need syncing to MinIO
    pub fn upload_changed_files(
        file_tree: &mut FileNode,
        bucket: &str,
    ) -> Result<Vec<String>, String> {
        let mut uploaded_files = Vec::new();

        // Get all files that need syncing
        let files_to_sync: Vec<String> = file_tree
            .get_all_files()
            .into_iter()
            .filter(|f| f.needs_sync())
            .map(|f| f.relative_path.clone())
            .collect();

        println!("Found {} files that need syncing", files_to_sync.len());

        for relative_path in files_to_sync {
            // Find the file again to get the reference
            if let Some(file) = Self::find_file_by_relative_path(file_tree, &relative_path) {
                match Self::upload_file_to_minio(file, bucket) {
                    Ok(_) => {
                        println!("✓ Uploaded: {}", relative_path);
                        // Mark file as synced after successful upload
                        Self::mark_file_as_synced(file_tree, &relative_path)?;
                        uploaded_files.push(relative_path);
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to upload {}: {}", relative_path, e);
                        return Err(format!("Failed to upload {}: {}", relative_path, e));
                    }
                }
            }
        }

        Ok(uploaded_files)
    }

    /// Upload a single file to MinIO using the existing MinioUtil
    fn upload_file_to_minio(file: &FileNode, bucket: &str) -> Result<(), String> {
        use crate::core::minio_util::MinioUtil;

        // Use relative path as the object name for cross-platform compatibility
        let object_name = &file.relative_path;

        MinioUtil::upload_file(&file.path, bucket, object_name)
    }

    /// Mark a file as synced by updating its last_synced timestamp
    pub fn mark_file_as_synced(
        file_tree: &mut FileNode,
        relative_path: &str,
    ) -> Result<(), String> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get current time: {}", e))?
            .as_secs();

        if let Some(file) = Self::find_file_by_relative_path_mut(file_tree, relative_path) {
            file.last_synced = Some(current_time);
            Ok(())
        } else {
            Err(format!("File not found: {}", relative_path))
        }
    }

    /// Helper function to find a file by relative path (mutable version)
    fn find_file_by_relative_path_mut<'a>(
        tree: &'a mut FileNode,
        relative_path: &str,
    ) -> Option<&'a mut FileNode> {
        if tree.relative_path == relative_path {
            return Some(tree);
        }

        if let Some(children) = &mut tree.children {
            for child in children {
                if let Some(found) = Self::find_file_by_relative_path_mut(child, relative_path) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Compare local file tree with a remote file tree for sync planning
    pub fn plan_sync(local_tree: &FileNode, remote_tree: &FileNode) -> Vec<(String, SyncAction)> {
        let mut sync_plan = Vec::new();

        let local_files = local_tree.get_all_files();
        let remote_files = remote_tree.get_all_files();

        // Create a map of remote files by relative path
        let remote_map: HashMap<String, &FileNode> = remote_files
            .iter()
            .map(|f| (f.relative_path.clone(), *f))
            .collect();

        // Check each local file against remote
        for local_file in local_files {
            if let Some(remote_file) = remote_map.get(&local_file.relative_path) {
                let action = local_file.compare_for_sync(remote_file);
                if action != SyncAction::NoAction {
                    sync_plan.push((local_file.relative_path.clone(), action));
                }
            } else {
                // File doesn't exist remotely, should upload
                sync_plan.push((local_file.relative_path.clone(), SyncAction::Upload));
            }
        }

        // Check for files that exist remotely but not locally
        for remote_file in remote_files {
            if !local_tree.find_by_path(&remote_file.path).is_some() {
                sync_plan.push((remote_file.relative_path.clone(), SyncAction::Download));
            }
        }

        sync_plan
    }

    /// Execute a sync plan
    pub fn execute_sync_plan(
        sync_plan: Vec<(String, SyncAction)>,
        local_tree: &FileNode,
        bucket: &str,
    ) -> Result<(), String> {
        for (relative_path, action) in sync_plan {
            match action {
                SyncAction::Upload => {
                    if let Some(file) = Self::find_file_by_relative_path(local_tree, &relative_path)
                    {
                        Self::upload_file_to_minio(file, bucket)?;
                        println!("✓ Uploaded: {}", relative_path);
                    }
                }
                SyncAction::Download => {
                    // TODO: Implement download functionality
                    println!(
                        "⚠ Download needed for: {} (not implemented yet)",
                        relative_path
                    );
                }
                SyncAction::NoAction => {
                    // Should not happen in a sync plan, but handle gracefully
                }
            }
        }
        Ok(())
    }

    /// Helper function to find a file by relative path
    fn find_file_by_relative_path<'a>(
        tree: &'a FileNode,
        relative_path: &str,
    ) -> Option<&'a FileNode> {
        if tree.relative_path == relative_path {
            return Some(tree);
        }

        if let Some(children) = &tree.children {
            for child in children {
                if let Some(found) = Self::find_file_by_relative_path(child, relative_path) {
                    return Some(found);
                }
            }
        }

        None
    }
}
