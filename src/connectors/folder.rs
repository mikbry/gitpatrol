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

impl Connector for FolderConnector {
    type FileIter = FolderFileIterator;

    fn files(&self) -> Result<Self::FileIter> {
        Ok(FolderFileIterator {
            walker: WalkDir::new(&self.root_path).into_iter(),
            root_path: self.root_path.clone(),
        })
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
        let full_path = self.root_path.join(path);
        Ok(fs::read_to_string(full_path)?)
    }
}
