use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "figma-image-tool")]
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
    #[arg(long, default_value = "./downloads")]
    download_dir: PathBuf,
  },
  Convert {
    #[arg(long, default_value = "./downloads")]
    input_dir: PathBuf,
    #[arg(long, default_value = "./output")]
    output_dir: PathBuf,
    #[arg(long, default_value = "webp")]
    format: String,
  },
}
