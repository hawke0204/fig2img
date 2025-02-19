use std::fs;

use clap::Parser;

mod cli;
mod config;
mod converter;
mod downloader;
mod figma;

use cli::{Cli, Commands};
use converter::ImageConverter;
use downloader::ImageDownloader;
use figma::FigmaImageExtractor;

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Download { download_dir } => {
      // Create download directory
      if let Err(e) = fs::create_dir_all(&download_dir) {
        eprintln!("❌ Failed to create download directory: {}", e);
        return;
      }

      // Fetch and download images
      match FigmaImageExtractor::fetch_figma_images() {
        Ok(Some(images)) => {
          for (node_id, image_url) in images {
            if let Some(url) = image_url.as_str() {
              let png_filename = download_dir.join(format!("{}.png", node_id));
              let png_path = png_filename.to_str().unwrap();

              match ImageDownloader::download_image(url, png_path) {
                Ok(_) => println!("✅ Downloaded: {}", png_path),
                Err(e) => eprintln!("❌ Failed to download: {}", e),
              }
            }
          }
        }
        Ok(None) => eprintln!("No images found."),
        Err(e) => eprintln!("❌ Figma API request failure: {}", e),
      }
    }
    Commands::Convert {
      input_dir,
      output_dir,
      format,
    } => {
      // Create output directory
      if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("❌ Failed to create output directory: {}", e);
        return;
      }

      // Read PNG files from input directory
      match fs::read_dir(&input_dir) {
        Ok(entries) => {
          for entry in entries {
            if let Ok(entry) = entry {
              let path = entry.path();
              if path.extension().map_or(false, |ext| ext == "png") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let output_path = output_dir.join(format!("{}.{}", file_stem, format));

                match ImageConverter::convert_to_webp(
                  path.to_str().unwrap(),
                  output_path.to_str().unwrap(),
                ) {
                  Ok(_) => println!("✅ Converted: {}", output_path.display()),
                  Err(e) => eprintln!("❌ Conversion failed: {}", e),
                }
              }
            }
          }
        }
        Err(e) => eprintln!("❌ Failed to read input directory: {}", e),
      }
    }
  }
}
