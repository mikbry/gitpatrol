use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct ZipConnector {
    archive: ZipArchive<File>,
    has_package_json: bool,
}

impl ZipConnector {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        let has_package_json = (0..archive.len()).any(|i| {
            archive
                .by_index(i)
                .map(|file| file.name().ends_with("package.json"))
                .unwrap_or(false)
        });

        Ok(Self {
            archive,
            has_package_json,
        })
    }
}

#[async_trait]
impl super::Connector for ZipConnector {
    async fn scan(&self) -> Result<bool> {
        let mut found_suspicious = false;
        let mut archive = &self.archive;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();

            if !name.ends_with(".js") && !name.ends_with(".ts") 
               && !name.ends_with(".jsx") && !name.ends_with(".tsx") {
                continue;
            }

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Use Scanner's analyze_content method through trait object
            let scanner = super::super::scanner::Scanner::new(self);
            if scanner.analyze_content(&contents, &name.to_string(), false) {
                found_suspicious = true;
            }
        }

        Ok(found_suspicious)
    }

    fn has_package_json(&self) -> bool {
        self.has_package_json
    }

    async fn get_file_content(&self, path: &str) -> Result<String> {
        let mut archive = &self.archive;
        let mut contents = String::new();
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == path {
                file.read_to_string(&mut contents)?;
                return Ok(contents);
            }
        }
        
        anyhow::bail!("File not found in zip: {}", path)
    }
}
