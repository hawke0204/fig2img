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

  #[cfg(test)]
  pub fn with_client(client: Client) -> Self {
    Self { client }
  }

  pub async fn download(
    &self,
    image_url: &str,
    filename: &str,
  ) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = self.client.get(image_url).send().await?;

    if !response.status().is_success() {
      return Err(format!("HTTP error: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let mut file = File::create(filename).await?;
    file.write_all(&bytes).await?;

    Ok(filename.to_string())
  }
}

#[cfg(test)]
mod tests {
  use std::fs;

  use httpmock::prelude::*;
  use tempfile::tempdir;

  use super::*;

  #[tokio::test]
  async fn test_download_success() {
    let server = MockServer::start();
    let mock_image = vec![1, 2, 3, 4];

    let mock = server.mock(|when, then| {
      when.method("GET").path("/test.png");
      then
        .status(200)
        .header("content-type", "image/png")
        .body(mock_image.clone());
    });

    let temp_dir = tempdir().unwrap();
    let temp_file = temp_dir.path().join("downloaded.png");
    let temp_file_path = temp_file.to_str().unwrap();

    let client = Client::new();
    let downloader = ImageDownloader::with_client(client);

    let result = downloader
      .download(&server.url("/test.png"), temp_file_path)
      .await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), temp_file_path);
    assert!(temp_file.exists());

    let downloaded_content = fs::read(temp_file_path).unwrap();
    assert_eq!(downloaded_content, mock_image);

    mock.assert();
  }

  #[tokio::test]
  async fn test_download_server_error() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
      when.method("GET").path("/error.png");
      then.status(500);
    });

    let temp_dir = tempdir().unwrap();
    let temp_file = temp_dir.path().join("error.png");
    let temp_file_path = temp_file.to_str().unwrap();

    let client = Client::new();
    let downloader = ImageDownloader::with_client(client);
    let result = downloader
      .download(&server.url("/error.png"), temp_file_path)
      .await;

    assert!(result.is_err());
    assert!(!temp_file.exists());

    mock.assert();
  }
}
