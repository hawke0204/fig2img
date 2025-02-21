use std::io::{Error, ErrorKind};
use std::process::Command;

use image::GenericImageView;
use ravif::{Encoder, Img};
use rgb::FromSlice;
use tokio::fs;

pub struct ImageConverter;

impl ImageConverter {
  pub fn check_cwebp_installed() -> bool {
    Command::new("cwebp")
      .arg("-version")
      .output()
      .map_or(false, |output| output.status.success())
  }

  pub fn print_installation_guide() {
    println!("❗️cwebp is not installed. Please install it:");
    println!("❗️macOS: brew install webp");
    println!("❗️Ubuntu: sudo apt-get install webp");
    println!("❗️Windows: choco install webp");
    println!("❗️Or download from: https://developers.google.com/speed/webp/download");
  }

  pub async fn convert_to_webp(input_file: &str, output_file: &str) -> Result<bool, Error> {
    if !Self::check_cwebp_installed() {
      Self::print_installation_guide();
      return Err(Error::new(
        ErrorKind::NotFound,
        "[❌] cwebp command not found",
      ));
    }

    let status = Command::new("cwebp")
      .arg("-quiet")
      .arg(input_file)
      .arg("-o")
      .arg(output_file)
      .status()?;

    Ok(status.success())
  }

  pub async fn convert_to_avif(input_file: &str, output_file: &str) -> Result<bool, Error> {
    let input_file = input_file.to_string();
    let output_file = output_file.to_string();

    // Run CPU-intensive operations in a blocking thread
    let encoded = tokio::task::spawn_blocking(move || {
      let img = image::open(&input_file).map_err(|e| {
        Error::new(
          ErrorKind::Other,
          format!("[❌] Failed to open image: {}", e),
        )
      })?;

      let (width, height) = img.dimensions();
      let rgba = img.to_rgba8();
      let pixels = rgba.as_raw();

      Encoder::new()
        .with_quality(80.0)
        .with_alpha_quality(80.0)
        .with_speed(8)
        .encode_rgba(Img::new(pixels.as_rgba(), width as usize, height as usize))
        .map_err(|e| {
          Error::new(
            ErrorKind::Other,
            format!("[❌] Failed to encode AVIF: {}", e),
          )
        })
    })
    .await
    .map_err(|e| Error::new(ErrorKind::Other, format!("[❌] Task failed: {}", e)))??;

    fs::write(output_file, encoded.avif_file.as_slice())
      .await
      .map_err(|e| {
        Error::new(
          ErrorKind::Other,
          format!("[❌] Failed to write AVIF file: {}", e),
        )
      })?;

    Ok(true)
  }
}
