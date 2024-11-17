mod scanner;
mod connectors;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;
use tokio::runtime::Runtime;

use crate::scanner::{Scanner, VERSION};
use crate::connectors::{ZipConnector, FolderConnector, GithubConnector};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to analyze (zip file or directory)
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// GitHub repository URL to analyze (e.g. https://github.com/owner/repo)
    #[arg(short, long)]
    url: Option<String>,
}

fn analyze_zip_file(zip_path: &PathBuf) -> Result<bool> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "📦 Analyzing zip file:".bright_blue().bold(),
        zip_path.display().to_string().yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = ZipConnector::new(zip_path.clone())?;
    let scanner = Scanner::new(connector);
    
    let found_suspicious = scanner.scan()?;

    // Show final status
    println!("\n{}", "┄".repeat(80).bright_blue());
    println!(
        "  {} {}",
        "📊 Analysis Result:".bright_blue().bold(),
        if found_suspicious {
            "🔴 Suspicious patterns detected".red().bold()
        } else {
            "🟢 No suspicious patterns found".green().bold()
        }
    );
    println!("{}", "━".repeat(80).bright_blue());

    Ok(found_suspicious)
}

fn analyze_folder(folder_path: &PathBuf) -> Result<bool> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "📁 Analyzing folder:".bright_blue().bold(),
        folder_path.display().to_string().yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = FolderConnector::new(folder_path.clone())?;
    let scanner = Scanner::new(connector);
    
    let found_suspicious = scanner.scan()?;

    // Show final status
    println!("\n{}", "┄".repeat(80).bright_blue());
    println!(
        "  {} {}",
        "📊 Analysis Result:".bright_blue().bold(),
        if found_suspicious {
            "🔴 Suspicious patterns detected".red().bold()
        } else {
            "🟢 No suspicious patterns found".green().bold()
        }
    );
    println!("{}", "━".repeat(80).bright_blue());

    Ok(found_suspicious)
}

async fn analyze_github_repo(url: &str) -> Result<()> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "🔍 Analyzing GitHub repository:".bright_blue().bold(),
        url.yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = GithubConnector::new(url.to_string())?;
    let scanner = Scanner::new(connector);
    
    let found_suspicious = scanner.scan()?;

    // Show final status
    println!("\n{}", "┄".repeat(80).bright_blue());
    println!(
        "  {} {}",
        "📊 Analysis Result:".bright_blue().bold(),
        if found_suspicious {
            "🔴 Suspicious patterns detected".red().bold()
        } else {
            "🟢 No suspicious patterns found".green().bold()
        }
    );
    println!("{}", "━".repeat(80).bright_blue());
    Ok(())
}

fn main() -> Result<()> {
    let runtime = Runtime::new()?;
    let cli = Cli::parse();

    println!(
        "\n{}",
        "🔍 Ziiircom : Git repository malware scanner"
            .bright_cyan()
            .bold()
    );
    println!("{} {}\n", "Version:".bright_blue(), VERSION.yellow());

    if let Some(url) = cli.url {
        runtime.block_on(analyze_github_repo(&url))?;
    } else if let Some(path) = cli.path {
        if path.is_dir() {
            analyze_folder(&path)?;
        } else if path.extension().map_or(false, |ext| ext == "zip") {
            analyze_zip_file(&path)?;
        } else {
            println!(
                "{}",
                "Error: Path must be either a directory or a zip file"
                    .red()
                    .bold()
            );
        }
    } else {
        println!("\n{}", "Usage:".bright_blue().bold());
        println!("  {} {}", "Scan a directory:".yellow(), "cargo run -- -p ./path/to/dir".bright_blue());
        println!("  {} {}", "Scan a zip file:".yellow(), "cargo run -- -p ./path/to/file.zip".bright_blue());
        println!("  {} {}", "Scan a GitHub repo:".yellow(), "cargo run -- -u https://github.com/owner/repo".bright_blue());
        println!("\nFor more options, run: {}", "cargo run -- --help".bright_blue());
    }

    Ok(())
}
