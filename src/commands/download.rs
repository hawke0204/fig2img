use std::path::PathBuf;

use downloader::ImageDownloader;
use extractor::FigmaImageExtractor;
use futures::future;
use reqwest::Client;
use tokio::fs;

use crate::config::FigmaConfig;
use crate::core::{downloader, extractor};

pub async fn execute(download_dir: PathBuf) {
  if let Err(e) = fs::create_dir_all(&download_dir).await {
    eprintln!("[❌]Failed to create download directory: {}", e);
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
            let sanitized_name = name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
            let png_filename = download_dir.join(format!("{}.png", sanitized_name));
            let png_path = png_filename.to_str().unwrap().to_string();
            let url = url.to_string();

            async move {
              match ImageDownloader::download(&url, &png_path).await {
                Ok(_) => {
                  println!("[✅]Downloaded: {}", png_path);
                  Ok(())
                }
                Err(e) => {
                  eprintln!("❌ Failed to download: {}", e);
                  Err(e)
                }
              }
            }
          })
        })
        .collect::<Vec<_>>();

      if let Err(e) = future::try_join_all(downloads).await {
        eprintln!("[❌]Some downloads failed: {}", e);
      }
    }
    // Ok(None) => println!("✅ No images found."),
    Err(e) => eprintln!("[❌]Failed to request figma API: {}", e),
  }
}
