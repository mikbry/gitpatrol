use anyhow::Result;

#[async_trait::async_trait]
pub trait Connector {
    type FileIter: Iterator<Item = String>;
    
    async fn iter(&self) -> Result<Self::FileIter>;
    async fn get_file_content(&self, path: &str) -> Result<String>;
    async fn has_package_json(&self) -> Result<bool>;
}

pub mod github;
pub mod zip;
pub mod folder;

pub use github::GithubConnector;
pub use zip::ZipConnector;
pub use folder::FolderConnector;
