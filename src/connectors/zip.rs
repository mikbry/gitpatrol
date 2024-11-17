use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;

use crate::connectors::Connector;

use std::sync::{Arc, Mutex};

pub struct ZipConnector {
    archive: Arc<Mutex<ZipArchive<File>>>,
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
            let result = self.archive.lock()
                .ok()
                .and_then(|mut archive| {
                    archive.by_index(self.current_index)
                        .ok()
                        .map(|file| file.name().to_string())
                });
            
            self.current_index += 1;
            
            if let Some(name) = result {
                return Some(name);
            }
        }
        None
    }
}

impl ZipConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self {
            archive: Arc::new(Mutex::new(archive)),
        })
    }
}

#[async_trait::async_trait]
impl Connector for ZipConnector {
    type FileIter = ZipFileIterator;

    async fn iter(&self) -> Result<Self::FileIter> {
        let archive = self.archive.lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;
        let total_files = archive.len();
        Ok(ZipFileIterator {
            archive: Arc::clone(&self.archive),
            current_index: 0,
            total_files,
        })
    }


    async fn get_file_content(&self, path: &str) -> Result<String> {
        let mut contents = String::new();
        let mut archive = self.archive.lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;

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
