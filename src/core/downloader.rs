use std::error::Error;

use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct ImageDownloader {
  client: Client,
}

impl ImageDownloader {
  pub fn new() -> Self {
    Self {
      client: Client::new(),
    }
  }

  pub async fn download(
    &self,
    image_url: &str,
    filename: &str,
  ) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = self.client.get(image_url).send().await?;
    let bytes = response.bytes().await?;

    let mut file = File::create(filename).await?;
    file.write_all(&bytes).await?;

    Ok(filename.to_string())
  }
}
