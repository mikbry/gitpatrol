use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use anyhow::Result;
use std::path::PathBuf;

use crate::scanner::Connector;

use std::sync::{Arc, Mutex};

pub struct ZipConnector {
    archive: Arc<Mutex<ZipArchive<File>>>,
    has_package_json: bool,
}

pub struct ZipFileIterator {
    archive: Arc<Mutex<ZipArchive<File>>>,
    current_index: usize,
    total_files: usize,
}

impl Iterator for ZipFileIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.total_files {
            if let Ok(archive) = self.archive.lock() {
                if let Ok(file) = archive.by_index(self.current_index) {
                    let name = file.name().to_string();
                    self.current_index += 1;
                    return Some(name);
                }
            }
            self.current_index += 1;
        }
        None
    }
}

impl ZipConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let len = archive.len();
        
        let has_package_json = (0..len).any(|i| {
            archive
                .by_index(i)
                .map(|file| file.name().ends_with("package.json"))
                .unwrap_or(false)
        });

        Ok(Self {
            archive: Arc::new(Mutex::new(archive)),
            has_package_json,
        })
    }
}

impl Connector for ZipConnector {
    type FileIter = ZipFileIterator;

    fn iter(&self) -> Result<Self::FileIter> {
        let total_files = self.archive.lock()?.len();
        Ok(ZipFileIterator {
            archive: Arc::clone(&self.archive),
            current_index: 0,
            total_files,
        })
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
        let mut contents = String::new();
        let mut archive = self.archive.lock()?;
        
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
