use crate::connectors::Connector;
use anyhow::Result;
use reqwest::blocking::Client;
use url::Url;
use std::vec::IntoIter;

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

    fn fetch_contents(&self, path: &str) -> Result<Vec<serde_json::Value>> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        Ok(response.json()?)
    }

    fn collect_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let mut stack = vec![String::new()];
        
        while let Some(current_path) = stack.pop() {
            if let Ok(contents) = self.fetch_contents(&current_path) {
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
        }
        
        Ok(files)
    }
}

impl Connector for GithubConnector {
    type FileIter = GithubFileIterator;

    fn iter(&self) -> Result<Self::FileIter> {
        let files = self.collect_files()?;
        Ok(GithubFileIterator {
            files: files.into_iter()
        })
    }

    fn get_file_content(&self, path: &str) -> Result<String> {
        let download_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );
        
        let response = self.client
            .get(&download_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch file contents: {}", response.status());
        }

        Ok(response.text()?)
    }

    fn has_package_json(&self) -> Result<bool> {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/package.json",
            self.owner, self.repo
        );

        let response = self.client
            .get(&api_url)
            .header("User-Agent", "Ziiircom-Scanner")
            .send()?;

        Ok(response.status().is_success())
    }
}
