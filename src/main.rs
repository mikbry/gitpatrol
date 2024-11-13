use anyhow::{Context, Result};
use clap::Parser;
use glob::glob;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;

const MAX_LINE_LENGTH: usize = 500; // Maximum allowed line length

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to analyze (zip file or directory)
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn analyze_zip_file(zip_path: &PathBuf) -> Result<()> {
    println!("\nAnalyzing zip file: {}", zip_path.display());
    
    let file = File::open(zip_path)
        .context("Failed to open zip file")?;
    
    let mut archive = ZipArchive::new(file)
        .context("Failed to read zip archive")?;

    analyze_archive(&mut archive)?;
    Ok(())
}

fn analyze_archive(archive: &mut ZipArchive<File>) -> Result<()> {
    
    let has_package_json = (0..archive.len()).any(|i| {
        archive.by_index(i)
            .map(|file| file.name().ends_with("package.json"))
            .unwrap_or(false)
    });

    println!("Repository contains package.json: {}", has_package_json);
    println!("Analyzing files for suspicious content...");

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

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if let Some(path) = cli.path {
        if path.is_dir() {
            // Scan all zip files in the directory
            let pattern = path.join("**/*.zip");
            let pattern_str = pattern.to_str().unwrap_or("**/*.zip");
            
            for entry in glob(pattern_str)? {
                match entry {
                    Ok(path) => {
                        if let Err(e) = analyze_zip_file(&path) {
                            println!("Error analyzing {}: {}", path.display(), e);
                        }
                    }
                    Err(e) => println!("Error in glob pattern: {}", e),
                }
            }
        } else if path.extension().map_or(false, |ext| ext == "zip") {
            analyze_zip_file(&path)?;
        } else {
            println!("Error: Path must be either a directory or a zip file");
        }
    } else {
        // Default to scanning assets directory if no path provided
        let assets_path = PathBuf::from("assets");
        if assets_path.is_dir() {
            let pattern = assets_path.join("*.zip");
            let pattern_str = pattern.to_str().unwrap_or("assets/*.zip");
            
            for entry in glob(pattern_str)? {
                match entry {
                    Ok(path) => {
                        if let Err(e) = analyze_zip_file(&path) {
                            println!("Error analyzing {}: {}", path.display(), e);
                        }
                    }
                    Err(e) => println!("Error in glob pattern: {}", e),
                }
            }
        } else {
            println!("Error: assets directory not found");
        }
    }

    Ok(())
}
