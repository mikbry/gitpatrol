use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;

const MAX_LINE_LENGTH: usize = 500; // Maximum allowed line length

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the zip file
    #[arg(short, long)]
    zip_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let file = File::open(&cli.zip_path)
        .context("Failed to open zip file")?;
    
    let mut archive = ZipArchive::new(file)
        .context("Failed to read zip archive")?;
    
    // Check for package.json
    let has_package_json = (0..archive.len()).any(|i| {
        archive.by_index(i)
            .map(|file| file.name().ends_with("package.json"))
            .unwrap_or(false)
    });

    println!("Repository contains package.json: {}", has_package_json);
    println!("\nAnalyzing files for suspicious content...\n");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Skip non JavaScript/TypeScript files
        if !name.ends_with(".js") && !name.ends_with(".ts") {
            continue;
        }

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Analyze each line
        for (line_num, line) in contents.lines().enumerate() {
            if line.len() > MAX_LINE_LENGTH {
                println!("WARNING: Suspicious long line detected!");
                println!("File: {}", name);
                println!("Line number: {}", line_num + 1);
                println!("Line length: {} characters", line.len());
                println!("--------------------");
            }
        }
    }

    Ok(())
}
