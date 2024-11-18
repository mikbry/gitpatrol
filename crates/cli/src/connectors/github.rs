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
    token: Option<String>,
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

        let token = std::env::var("GITHUB_TOKEN").ok();
        
        Ok(Self {
            client: Client::new(),
            owner: path_segments[0].to_string(),
            repo: path_segments[1].to_string(),
            token,
        })
    }

    async fn fetch_contents(&self, path: &str) -> Result<Vec<serde_json::Value>> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let mut request = self.client
            .get(&api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("token {}", token));
        }
            
        let response = request
            .send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await?;
            match status.as_u16() {
                404 => {
                    if error_body.contains("Not Found") {
                        anyhow::bail!("Repository not found: {}/{}", self.owner, self.repo)
                    } else {
                        anyhow::bail!("Path not found in repository")
                    }
                },
                403 => {
                    if error_body.contains("rate limit") {
                        anyhow::bail!("GitHub API rate limit exceeded. Please try again later. Consider setting a GITHUB_TOKEN environment variable.")
                    } else {
                        anyhow::bail!("Access denied - Repository may be private. If you have access, set GITHUB_TOKEN environment variable.")
                    }
                },
                _ => anyhow::bail!("GitHub API error ({}): {}", status, error_body)
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
        
        let mut request = self.client
            .get(&api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
            .header("Accept", "application/vnd.github.v3+json");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await?;

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
