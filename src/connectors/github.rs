use crate::scanner::Connector;
use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use url::Url;

#[derive(Clone)]
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
        let mut found_suspicious = false;
        let mut stack = vec![String::new()];

        while let Some(current_path) = stack.pop() {
            let api_url = format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                self.owner, self.repo, current_path
            );

            let response = self.client
                .get(&api_url)
                .header("User-Agent", "Ziiircom-Scanner")
                .send()
                .await?;

            if !response.status().is_success() {
                continue;
            }

            let contents: Vec<serde_json::Value> = response.json().await?;

            for item in contents {
                if let (Some(type_str), Some(path)) = (item["type"].as_str(), item["path"].as_str()) {
                    match type_str {
                        "dir" => stack.push(path.to_string()),
                        "file" => {
                            if path.ends_with(".js") || path.ends_with(".ts") 
                               || path.ends_with(".jsx") || path.ends_with(".tsx") {
                                if let Some(download_url) = item["download_url"].as_str() {
                                    let content = self.client.get(download_url)
                                        .send().await?
                                        .text().await?;
                                    
                                    let scanner = super::super::scanner::Scanner::new(self.clone());
                                    if scanner.analyze_content(&content, &path.to_string(), false) {
                                        found_suspicious = true;
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        Ok(found_suspicious)
    }

    async fn has_package_json(&self) -> bool {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/package.json",
            self.owner, self.repo
        );

        self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()
            .await
            .map(|response| response.status().is_success())
            .unwrap_or(false)
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
