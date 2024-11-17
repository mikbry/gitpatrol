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

impl Connector for GithubConnector {
    fn list_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
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
                .blocking()?;

            if !response.status().is_success() {
                continue;
            }

            let contents: Vec<serde_json::Value> = response.json()?;

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

        Ok(files)
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
