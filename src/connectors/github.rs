use crate::scanner::Connector;
use anyhow::Result;
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

pub struct GithubFileIterator {
    client: reqwest::Client,
    owner: String,
    repo: String,
    stack: Vec<String>,
    current_files: Vec<String>,
}

impl Iterator for GithubFileIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_files.is_empty() && !self.stack.is_empty() {
            if let Some(current_path) = self.stack.pop() {
                if let Ok(contents) = self.fetch_contents(&current_path) {
                    for item in contents {
                        if let (Some(type_str), Some(path)) = (item["type"].as_str(), item["path"].as_str()) {
                            match type_str {
                                "dir" => self.stack.push(path.to_string()),
                                "file" => self.current_files.push(path.to_string()),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        self.current_files.pop()
    }
}

impl GithubFileIterator {
    fn fetch_contents(&self, path: &str) -> Result<Vec<serde_json::Value>> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()
            .blocking()?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        Ok(response.json()?)
    }
}

impl Connector for GithubConnector {
    type FileIter = GithubFileIterator;

    fn iter(&self) -> Result<Self::FileIter> {
        Ok(GithubFileIterator {
            client: self.client.clone(),
            owner: self.owner.clone(),
            repo: self.repo.clone(),
            stack: vec![String::new()],
            current_files: Vec::new(),
        })
    }

    fn has_package_json(&self) -> bool {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/package.json",
            self.owner, self.repo
        );

        self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()
            .blocking()
            .map(|response| response.status().is_success())
            .unwrap_or(false)
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
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
