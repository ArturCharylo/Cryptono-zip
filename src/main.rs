use clap::Parser;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use cryptono_zip::compress_data;

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

    let content = fs::read_to_string(&cli.input)
    .with_context(|| format!("Could not read file `{}`", cli.input.display()))?;
    println!("Original size: {} bytes", content.len());

    let compressed = compress_data(&content);

    println!("Compressed data length: {:?}", compressed.len());

    let ratio = (1.0 - (compressed.len() as f64 / content.len() as f64)) * 100.0;
    println!("Compression ratio: {:.2}%", ratio);



    Ok(())
}

