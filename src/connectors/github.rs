use crate::connectors::Connector;
use anyhow::Result;
use reqwest::Client;
use url::Url;
use std::vec::IntoIter;
use base64::Engine;

pub struct GithubConnector {
    client: Client,
    owner: String,
    repo: String,
}

pub struct GithubFileIterator {
    files: IntoIter<String>,
}

impl Iterator for GithubFileIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.next()
    }
}

impl GithubConnector {
    pub fn new(url: String) -> Result<Self> {
        let parsed_url = Url::parse(&url)?;
        let path_segments: Vec<&str> = parsed_url.path_segments().unwrap().collect();
        if path_segments.len() < 2 {
            anyhow::bail!("Invalid GitHub URL format. Expected: https://github.com/owner/repo");
        }

        Ok(Self {
            client: Client::new(),
            owner: path_segments[0].to_string(),
            repo: path_segments[1].to_string(),
        })
    }

    async fn fetch_contents(&self, path: &str) -> Result<Vec<serde_json::Value>> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send().await?;

        let status = response.status();
        if !status.is_success() {
            match status.as_u16() {
                404 => anyhow::bail!("Repository not found: {}/{}", self.owner, self.repo),
                403 => anyhow::bail!("Access denied - Repository may be private"),
                _ => anyhow::bail!("GitHub API error: {}", status)
            }
        }

        let json = response.json().await?;
        Ok(json)
    }

    async fn collect_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let mut stack = vec![String::new()];
        
        while let Some(current_path) = stack.pop() {
            let contents = self.fetch_contents(&current_path).await?;
            for item in contents {
                if let (Some(type_str), Some(path)) = (item["type"].as_str(), item["path"].as_str()) {
                    match type_str {
                        "dir" => stack.push(path.to_string()),
                        "file" => files.push(path.to_string()),
                        _ => {}
                    }
                }
            }
        }
        
        if files.is_empty() {
            anyhow::bail!("No files found in repository");
        }
        
        Ok(files)
    }
}

#[async_trait::async_trait]
impl Connector for GithubConnector {
    type FileIter = GithubFileIterator;

    async fn iter(&self) -> Result<Self::FileIter> {
        let files = self.collect_files().await?;
        Ok(GithubFileIterator {
            files: files.into_iter()
        })
    }

    async fn get_file_content(&self, path: &str) -> Result<String> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );
        
        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch file contents: {}", response.status());
        }

        let content: serde_json::Value = response.json().await?;
        
        if let Some(content) = content["content"].as_str() {
            // GitHub API returns base64 encoded content
            let decoded = base64::engine::general_purpose::STANDARD.decode(content.replace("\n", ""))?;
            String::from_utf8(decoded).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
        } else {
            anyhow::bail!("No content found in GitHub response")
        }
    }

}
