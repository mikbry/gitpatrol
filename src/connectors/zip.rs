use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use anyhow::Result;
use std::path::PathBuf;

use crate::scanner::Connector;

#[derive(Clone)]
pub struct ZipConnector {
    archive: ZipArchive<File>,
    has_package_json: bool,
}

impl ZipConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        let has_package_json = (0..archive.len()).any(|i| {
            archive
                .by_index(i)
                .map(|file| file.name().ends_with("package.json"))
                .unwrap_or(false)
        });

        Ok(Self {
            archive,
            has_package_json,
        })
    }
}

impl Connector for ZipConnector {
    fn list_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        for i in 0..self.archive.len() {
            if let Ok(file) = self.archive.by_index(i) {
                files.push(file.name().to_string());
            }
        }
        Ok(files)
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
        let mut archive = &self.archive;
        let mut contents = String::new();
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == path {
                file.read_to_string(&mut contents)?;
                return Ok(contents);
            }
        }
        
        anyhow::bail!("File not found in zip: {}", path)
    }
}
