use crate::core::minio_util;
use crate::utils::tree_serializer::FileNode;
use std::fs::File;
pub struct FileUpload {}

impl FileUpload {
    pub fn read_file_tree(&self, json_path: &str) -> Result<(), String> {
        let root_path = std::path::Path::new(json_path)
            .parent()
            .unwrap()
            .to_path_buf();
        let file_tree: FileNode =
            serde_json::from_reader(File::open(json_path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        println!("{}", root_path.display());
        Ok(())
    }

    pub fn upload_to_minio(file_path: &str, bucket: &str, object_name: &str) -> Result<(), String> {
        minio_util::MinioUtil::upload_file(file_path, bucket, object_name)
    }
}
