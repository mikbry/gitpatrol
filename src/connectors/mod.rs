use anyhow::Result;
use std::pin::Pin;
use futures::Stream;

pub trait Connector {
    type FileIter: Stream<Item = String> + Send + 'static;
    
    fn iter(&self) -> Result<Pin<Box<Self::FileIter>>>;
    async fn get_file_content(&self, path: &str) -> Result<String>;
    async fn has_package_json(&self) -> Result<bool>;
}

pub mod github;
pub mod zip;
pub mod folder;

pub use github::GithubConnector;
pub use zip::ZipConnector;
pub use folder::FolderConnector;
