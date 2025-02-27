use std::path::PathBuf;

use downloader::ImageDownloader;
use extractor::FigmaImageExtractor;
use futures::future;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::config::FigmaConfig;
use crate::core::{downloader, extractor};
use crate::utils::filename;

#[derive(Default, Deserialize, Serialize)]
pub struct DownloadOptions {
  #[serde(default)]
  quiet: bool,
}

impl DownloadOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn quiet(mut self, quiet: bool) -> Self {
    self.quiet = quiet;
    self
  }
}

pub async fn execute(download_dir: PathBuf, options: DownloadOptions) {
  if let Err(e) = fs::create_dir_all(&download_dir).await {
    if !options.quiet {
      eprintln!("[❌]Failed to create download directory: {}", e);
    }
    return;
  }

  let config = FigmaConfig::new();
  let extractor = FigmaImageExtractor::new(Client::new(), config);

  match extractor.extract().await {
    Ok(images) => {
      let downloads = images
        .into_iter()
        .filter_map(|(_node_id, image_url, name)| {
          image_url.as_str().map(|url| {
            let url = url.to_string();

            let sanitized_name = filename::sanitize(&name);
            let png_filename = download_dir.join(format!("{}.png", sanitized_name));
            let png_path = png_filename.to_str().unwrap().to_string();

            let downloader = ImageDownloader::new();

            async move {
              match downloader.download(&url, &png_path).await {
                Ok(path) => {
                  if !options.quiet {
                    println!("✅ Downloaded: {}", path);
                  }
                  Ok(())
                }
                Err(error) => {
                  if !options.quiet {
                    eprintln!("❌ Failed to download {}: {}", png_path, error);
                  }
                  Err(error)
                }
              }
            }
          })
        })
        .collect::<Vec<_>>();

      if let Err(e) = future::try_join_all(downloads).await {
        if !options.quiet {
          eprintln!("[❌] Some downloads failed: {}", e);
        }
      }
    }
    Err(e) => {
      if !options.quiet {
        eprintln!("[❌] Failed to request figma API: {}", e);
      }
    }
  }
}
