use anyhow::Result;

pub trait Connector {
    type FileIter: Iterator<Item = String>;
    
    fn iter(&self) -> Result<Self::FileIter>;
    fn get_file_content(&self, path: &str) -> Result<String>;
}

pub mod github;
pub mod zip;
pub mod folder;

pub use github::GithubConnector;
pub use zip::ZipConnector;
pub use folder::FolderConnector;
