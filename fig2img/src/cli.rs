use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fig2img")]
#[command(author = "GeonHyeok Lee")]
#[command(version = "0.0.1")]
#[command(about = "Downloads images from Figma and converts them to WebP", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Download {
    #[arg(long)]
    output: PathBuf,
  },
  Convert {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value = "webp")]
    format: String,
  },
}
