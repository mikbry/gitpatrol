use anyhow::Result;
use crate::connector::Connector;

pub const VERSION: &str = "1.0.0";
pub const MAX_LINE_LENGTH: usize = 500;
pub const SUSPICIOUS_PATTERNS: [&str; 6] = [
    "_0x",          // Hex variable names
    "eval(",        // eval usage
    "\\x",          // hex escape sequences
    "base64",       // base64 encoding
    "fromCharCode", // String.fromCharCode
    "unescape(",    // unescape usage
];

pub const SAFE_PATTERNS: [&str; 3] = [
    "!function(e,t)", // jQuery signature
    "/*! ",           // Common minified library header
    "(function(f)",   // Common module pattern
];

pub struct Scanner<T: Connector> {
    connector: T,
}

impl<T: Connector> Scanner<T> {
    pub fn new(connector: T) -> Self {
        Self { connector }
    }

    pub async fn scan(&self) -> Result<bool> {
        let mut found_suspicious = false;
        
        let files = self.connector.iter().await?;
        for file_path in files {
            if file_path.ends_with(".js") || file_path.ends_with(".ts") 
               || file_path.ends_with(".jsx") || file_path.ends_with(".tsx") {
                
                let content = self.connector.get_file_content(&file_path).await?;
                if self.analyze_content(&content, &file_path) {
                    found_suspicious = true;
                }
            }
        }
        
        Ok(found_suspicious)
    }

    pub fn analyze_content(&self, content: &str, file_path: &str) -> bool {
        let mut found_suspicious = false;

        for (line_num, line) in content.lines().enumerate() {
            if SAFE_PATTERNS.iter().any(|&pattern| line.contains(pattern)) {
                continue;
            }

            let is_minified = line.len() > MAX_LINE_LENGTH;
            let suspicious_patterns: Vec<&str> = SUSPICIOUS_PATTERNS
                .iter()
                .filter(|&&pattern| line.contains(pattern))
                .copied()
                .collect();

            if (!suspicious_patterns.is_empty() && suspicious_patterns.len() >= 2)
                || (is_minified && !suspicious_patterns.is_empty())
            {
                self.print_warning(file_path, line_num, line, &suspicious_patterns, is_minified);
                found_suspicious = true;
            }
        }

        found_suspicious
    }

    fn print_warning(&self, file_path: &str, line_num: usize, line: &str, patterns: &[&str], is_minified: bool) {
        println!(
            "\n    {}",
            "⚠️  WARNING: Suspicious code detected!"
        );
        println!("    {} {}", "📄 File:", file_path);
        println!(
            "    {} {}",
            "↳ Line:",
            (line_num + 1).to_string()
        );

        if is_minified {
            println!(
                "      {} {} {}",
                "⚡ Minified/obfuscated code (length:",
                line.len().to_string(),
                "chars)"
            );
        }

        if !patterns.is_empty() {
            println!("      {} {}", "⚠️", "Suspicious patterns found:");
            for pattern in patterns {
                println!("        {} {}", "↳", pattern);
            }
        }

        println!("{}", "─".repeat(50));
    }
}
