use anyhow::Result;

#[async_trait::async_trait]
pub trait Connector {
    type FileIter: Iterator<Item = String>;
    
    async fn iter(&self) -> Result<Self::FileIter>;
    async fn get_file_content(&self, path: &str) -> Result<String>;
}
