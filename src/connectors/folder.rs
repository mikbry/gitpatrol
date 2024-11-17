use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use async_trait::async_trait;

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

#[async_trait]
impl super::Connector for FolderConnector {
    async fn scan(&self) -> Result<bool> {
        // Implementation will be added in next step
        Ok(false)
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    async fn get_file_content(&self, path: &str) -> Result<String> {
        let full_path = self.root_path.join(path);
        Ok(fs::read_to_string(full_path)?)
    }
}
