mod connectors;
mod scanner;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

use crate::connectors::{FolderConnector, GithubConnector, ZipConnector};
use crate::scanner::{Scanner, VERSION};

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

async fn analyze_zip_file(zip_path: &PathBuf) -> Result<bool> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "📦 Analyzing zip file:".bright_blue().bold(),
        zip_path.display().to_string().yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = ZipConnector::new(zip_path.clone())?;
    let scanner = Scanner::new(connector);

    let found_suspicious = scanner.scan().await?;

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

async fn analyze_folder(folder_path: &PathBuf) -> Result<bool> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "📁 Analyzing folder:".bright_blue().bold(),
        folder_path.display().to_string().yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = FolderConnector::new(folder_path.clone())?;
    let scanner = Scanner::new(connector);

    let found_suspicious = scanner.scan().await?;

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

async fn analyze_github_repo(url: &str) -> Result<bool> {
    println!("\n{}", "━".repeat(80).bright_blue());
    println!(
        "{} {}",
        "🔍 Analyzing GitHub repository:".bright_blue().bold(),
        url.yellow()
    );
    println!("{}", "━".repeat(80).bright_blue());

    let connector = GithubConnector::new(url.to_string())?;
    let scanner = Scanner::new(connector);

    let found_suspicious = scanner.scan().await?;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "\n{}",
        "🔍 Ziiircom : Git repository malware scanner"
            .bright_cyan()
            .bold()
    );
    println!("{} {}\n", "Version:".bright_blue(), VERSION.yellow());

    if let Some(url) = cli.url {
        match analyze_github_repo(&url).await {
            Ok(_) => {
                // Result already printed in analyze_github_repo
            }
            Err(e) => {
                println!("\n{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else if let Some(path) = cli.path {
        if path.is_dir() {
            analyze_folder(&path).await?;
        } else if path.extension().map_or(false, |ext| ext == "zip") {
            analyze_zip_file(&path).await?;
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
        println!(
            "  {} {}",
            "Scan a directory:".yellow(),
            "cargo run -- -p ./path/to/dir".bright_blue()
        );
        println!(
            "  {} {}",
            "Scan a zip file:".yellow(),
            "cargo run -- -p ./path/to/file.zip".bright_blue()
        );
        println!(
            "  {} {}",
            "Scan a GitHub repo:".yellow(),
            "cargo run -- -u https://github.com/owner/repo".bright_blue()
        );
        println!(
            "\nFor more options, run: {}",
            "cargo run -- --help".bright_blue()
        );
    }

    Ok(())
}
