use crate::connectors::Connector;
use anyhow::Result;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;
use url::Url;
use tokio::sync::mpsc;

pub struct GithubConnector {
    client: Client,
    owner: String,
    repo: String,
}

impl GithubConnector {
    pub async fn new(url: String) -> Result<Self> {
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
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        Ok(response.json().await?)
    }
}

impl Connector for GithubConnector {
    type FileIter = Pin<Box<dyn Stream<Item = String> + Send>>;

    fn iter(&self) -> Result<Self::FileIter> {
        let (tx, rx) = mpsc::channel(32);
        let client = self.client.clone();
        let owner = self.owner.clone();
        let repo = self.repo.clone();

        tokio::spawn(async move {
            let mut stack = vec![String::new()];
            
            while let Some(current_path) = stack.pop() {
                let api_url = format!(
                    "https://api.github.com/repos/{}/{}/contents/{}",
                    owner, repo, current_path
                );

                match client
                    .get(&api_url)
                    .header("User-Agent", "Ziiircom-Scanner")
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        if let Ok(contents) = response.json::<Vec<serde_json::Value>>().await {
                            for item in contents {
                                if let (Some(type_str), Some(path)) = (item["type"].as_str(), item["path"].as_str()) {
                                    match type_str {
                                        "dir" => stack.push(path.to_string()),
                                        "file" => {
                                            if tx.send(path.to_string()).await.is_err() {
                                                return;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    _ => continue,
                }
            }
            // Explicitly drop tx when done to close the channel
            drop(tx);
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
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

    async fn has_package_json(&self) -> Result<bool> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/package.json",
            self.owner, self.repo
        );

        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}
