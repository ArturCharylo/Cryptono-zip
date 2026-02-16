use clap::Parser;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use cryptono_zip::{compress, Strategy};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = fs::read(&cli.input)
        .with_context(|| format!("Could not read file `{}`", cli.input.display()))?;

    println!("Original size: {} bytes", content.len());

    let compressed = compress(&content, Strategy::Deflate);

    println!("Compressed data length: {} bytes", compressed.len());

    if !content.is_empty() {
        let ratio = (1.0 - (compressed.len() as f64 / content.len() as f64)) * 100.0;
        println!("Compression ratio: {:.2}%", ratio);
    }

    let output_path = cli.output.unwrap_or_else(|| {
        let mut path = cli.input.clone();
        if let Some(extension) = path.extension() {
            let mut ext = extension.to_os_string();
            ext.push(".czip");
            path.set_extension(ext);
        } else {
            path.set_extension("czip");
        }
        path
    });

    fs::write(&output_path, &compressed)
        .with_context(|| format!("Failed to write output to `{}`", output_path.display()))?;

    println!("Successfully saved to: {}", output_path.display());

    Ok(())
}