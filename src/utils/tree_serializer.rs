// this module will be able to read and write json files
// it willl read the files in the correct format and return an output
// it will also write the files in the correct format
// this format will be stated in a documentation file

//kinda weird but we will have a json module in tihs file since i hope this is the only one that will needd it (im prolly worng)
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read};

// definition sof structs for the nodes
#[derive(Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub r#type: String, // "file" or "folder"
    pub children: Option<Vec<FileNode>>,
    pub git_remote: Option<String>,
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
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON format or structure"))
        }
    }
}

pub struct ReadFileTree;

impl ReadFileTree {
    pub fn generate_tree(path: &str) -> io::Result<FileNode> {
        use crate::utils::git_util::GitDetector;

        let root_path = std::path::Path::new(path);

        // Check if the root folder is a Git repository
        if GitDetector::is_git_repository(root_path) {
            let git_remotes = GitDetector::get_git_remotes(root_path);
            return Ok(FileNode {
                name: root_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                r#type: "folder".to_string(),
                children: None,
                git_remote: git_remotes.get(0).cloned(), // Use the first remote if available
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
                        children: None,
                        git_remote: git_remotes.get(0).cloned(),
                    });
                } else {
                    // Recursively process non-Git folders
                    children.push(ReadFileTree::generate_tree(&entry_path.to_string_lossy())?);
                }
            } else if entry_path.is_file() {
                // Add files to the children list
                children.push(FileNode {
                    name: entry_name,
                    r#type: "file".to_string(),
                    children: None,
                    git_remote: None,
                });
            }
        }

        Ok(FileNode {
            name: root_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            r#type: "folder".to_string(),
            children: Some(children),
            git_remote: None,
        })
    }
    pub fn generate_tree_as_string(path: &str) -> io::Result<String> {
        let tree = Self::generate_tree(path)?;
        let json_string = serde_json::to_string_pretty(&tree)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(json_string)
    }
}
pub struct JsonGenerator;

impl JsonGenerator{
    // generate a json in the format of the examples but take inputs of actual file paths and folder paths
    
}