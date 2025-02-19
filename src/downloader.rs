use std::error::Error;

use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct ImageDownloader;

impl ImageDownloader {
  pub async fn download_image(
    image_url: &str,
    filename: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let response = client.get(image_url).send().await?;
    let bytes = response.bytes().await?;

    let mut file = File::create(filename).await?;
    file.write_all(&bytes).await?;

    Ok(())
  }
}
