use wasm_bindgen::prelude::*;
use repo_analyzer_core::{Scanner, Connector};
use anyhow::Result;
use async_trait::async_trait;

#[wasm_bindgen]
pub struct WasmScanner {
    content: String,
}

struct StringConnector {
    content: String,
}

#[async_trait]
impl Connector for StringConnector {
    type FileIter = std::iter::Once<String>;
    
    async fn iter(&self) -> Result<Self::FileIter> {
        Ok(std::iter::once("file.js".to_string()))
    }

    async fn get_file_content(&self, _path: &str) -> Result<String> {
        Ok(self.content.clone())
    }
}

#[wasm_bindgen]
impl WasmScanner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    #[wasm_bindgen]
    pub async fn scan(&self) -> Result<bool, JsValue> {
        let connector = StringConnector {
            content: self.content.clone(),
        };
        
        let scanner = Scanner::new(connector);
        scanner.scan().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
