use repo_analyzer_core::Connector;

pub mod github;
pub mod zip;
pub mod folder;

pub use github::GithubConnector;
pub use zip::ZipConnector;
pub use folder::FolderConnector;
