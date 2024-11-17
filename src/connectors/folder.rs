use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::scanner::Connector;

#[derive(Clone)]
pub struct FolderConnector {
    root_path: PathBuf,
    has_package_json: bool,
}

impl FolderConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        let has_package_json = Path::new(&path).join("package.json").exists();
        
        Ok(Self {
            root_path: path,
            has_package_json,
        })
    }
}

impl Connector for FolderConnector {
    fn list_files(&self) -> Result<Vec<String>> {
        use walkdir::WalkDir;
        let mut files = Vec::new();
        
        for entry in WalkDir::new(&self.root_path)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    if let Ok(path) = entry.path().strip_prefix(&self.root_path) {
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            }
            
        Ok(files)
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
        let full_path = self.root_path.join(path);
        Ok(fs::read_to_string(full_path)?)
    }
}
