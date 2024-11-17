use crate::scanner::Connector;
use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use url::Url;

pub struct GithubConnector {
    url: String,
    client: reqwest::Client,
    owner: String,
    repo: String,
}

impl GithubConnector {
    pub fn new(url: String) -> Result<Self> {
        let parsed_url = Url::parse(&url)?;
        let path_segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();
        if path_segments.len() < 2 {
            anyhow::bail!("Invalid GitHub URL format. Expected: https://github.com/owner/repo");
        }

        Ok(Self {
            url,
            client: reqwest::Client::new(),
            owner: path_segments[0].to_string(),
            repo: path_segments[1].to_string(),
        })
    }
}

#[async_trait]
impl Connector for GithubConnector {
    async fn scan(&self) -> Result<bool> {
        // Implementation here
        Ok(false)
    }

    fn has_package_json(&self) -> bool {
        // Implementation here
        false
    }

    async fn get_file_content(&self, path: &str) -> Result<String> {
        let download_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );
        let response = self.client
            .get(&download_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch file contents: {}", response.status());
        }

        Ok(response.text().await?)
    }
}
