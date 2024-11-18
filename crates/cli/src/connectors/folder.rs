use std::path::PathBuf;
use anyhow::Result;

use crate::connectors::Connector;

#[derive(Clone)]
pub struct FolderConnector {
    root_path: PathBuf,
}

impl FolderConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            root_path: path,
        })
    }
}

use walkdir::{WalkDir, IntoIter as WalkDirIter};

pub struct FolderFileIterator {
    walker: WalkDirIter,
    root_path: PathBuf,
}

impl Iterator for FolderFileIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in &mut self.walker {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    if let Ok(path) = entry.path().strip_prefix(&self.root_path) {
                        if let Some(path_str) = path.to_str() {
                            return Some(path_str.to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

#[async_trait::async_trait]
impl Connector for FolderConnector {
    type FileIter = FolderFileIterator;

    async fn iter(&self) -> Result<Self::FileIter> {
        Ok(FolderFileIterator {
            walker: WalkDir::new(&self.root_path).into_iter(),
            root_path: self.root_path.clone(),
        })
    }


    async fn get_file_content(&self, path: &str) -> Result<String> {
        let full_path = self.root_path.join(path);
        tokio::fs::read_to_string(full_path).await.map_err(Into::into)
    }
}
