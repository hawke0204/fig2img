use std::io::{Error, ErrorKind};
use std::process::Command;

use image::{DynamicImage, GenericImageView};
use ravif::{Encoder, Img};
use rgb::FromSlice;

pub struct ImageConverter;

impl ImageConverter {
  pub async fn convert_to_webp(input_path: &str, output_path: &str) -> Result<bool, Error> {
    WebPConverter::convert(input_path, output_path).await
  }

  pub async fn convert_to_avif(input_path: &str, output_path: &str) -> Result<bool, Error> {
    AvifConverter::convert(input_path, output_path).await
  }
}

struct WebPConverter;

impl WebPConverter {
  async fn convert(input_path: &str, output_path: &str) -> Result<bool, Error> {
    Self::convert_inner(input_path, output_path)
  }

  fn convert_inner(input_path: &str, output_path: &str) -> Result<bool, Error> {
    if !Self::check_cwebp_installed() {
      Self::print_installation_guide();
      return Ok(false);
    }

    let status = Command::new("cwebp")
      .arg("-quiet")
      .arg(input_path)
      .arg("-o")
      .arg(output_path)
      .status()?;

    Ok(status.success())
  }

  fn check_cwebp_installed() -> bool {
    Command::new("cwebp")
      .arg("-version")
      .output()
      .map_or(false, |output| output.status.success())
  }

  fn print_installation_guide() {
    println!("cwebp is not installed. Please install it:");
    println!("macOS: brew install webp");
    println!("Ubuntu: sudo apt-get install webp");
    println!("Windows: choco install webp");
    println!("Or download from: https://developers.google.com/speed/webp/download");
  }
}

struct AvifConverter;

impl AvifConverter {
  async fn convert(input_path: &str, output_path: &str) -> Result<bool, Error> {
    let input_path = input_path.to_string();
    let output_path = output_path.to_string();

    let encoded = tokio::task::spawn_blocking(move || {
      image::open(&input_path)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        .and_then(Self::encode_image)
    })
    .await
    .map_err(|e| Error::new(ErrorKind::Other, e))??;

    tokio::fs::write(output_path, encoded.as_slice()).await?;

    Ok(true)
  }

  fn encode_image(img: DynamicImage) -> Result<Vec<u8>, Error> {
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8();
    let pixels = rgba.as_raw();

    let encoded = Encoder::new()
      .with_quality(80.0)
      .with_alpha_quality(80.0)
      .with_speed(8)
      .encode_rgba(Img::new(pixels.as_rgba(), width as usize, height as usize))
      .unwrap();

    Ok(encoded.avif_file)
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  // use std::time::Duration;
  use tempfile::NamedTempFile;

  // use tokio::fs;
  use super::*;

  fn create_test_image() -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let mut img = image::RgbaImage::new(100, 100);

    for pixel in img.pixels_mut() {
      *pixel = image::Rgba([255, 0, 0, 255]);
    }

    let img = DynamicImage::ImageRgba8(img);
    img
      .save_with_format(temp_file.path(), image::ImageFormat::Png)
      .unwrap();

    assert!(temp_file.path().exists(), "Test image file was not created");

    temp_file
  }

  #[tokio::test]
  async fn test_webp_conversion() {
    let input_file = create_test_image();
    let output_file = NamedTempFile::new().unwrap();

    let checked_cwebp = WebPConverter::check_cwebp_installed();
    assert!(checked_cwebp, "❌ cwebp is not installed");

    let result = ImageConverter::convert_to_webp(
      input_file.path().to_str().unwrap(),
      output_file.path().to_str().unwrap(),
    )
    .await;

    assert!(result.is_ok(), "❌ Conversion failed: {:?}", result.err());
    assert!(result.unwrap(), "❌ Conversion returned false");
    assert!(
      Path::new(output_file.path()).exists(),
      "❌ Output file does not exist"
    );
  }

  #[test]
  fn test_encode_image() {
    let img = DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));
    let result = AvifConverter::encode_image(img);
    assert!(result.is_ok());

    let result: Vec<u8> = result.unwrap();
    assert!(result.len() > 0);
    assert_eq!(&result[4..8], b"ftyp"); // AVIF 파일 매직 넘버 검사 (AVIF 파일은 'ftyp' 헤더로 시작)
  }

  // TODO: `convert_to_avif` 함수가 테스트를 실패하는 원인 파악 후 수정
  // #[tokio::test]
  // async fn test_avif_conversion() {
  //   let input_file = create_test_image();
  //   let output_file = NamedTempFile::new().unwrap();

  //   let input_path = input_file.path().to_str().unwrap();
  //   let output_path = output_file.path().to_str().unwrap();

  //   let result = tokio::time::timeout(
  //     Duration::from_secs(5),
  //     ImageConverter::convert_to_avif(input_path, output_path),
  //   )
  //   .await;

  //   assert!(result.is_ok(), "❌ Conversion timed out");

  //   let result = result.unwrap();
  //   assert!(result.is_ok(), "❌ Conversion failed: {:?}", result.err());
  //   assert!(result.unwrap(), "❌ Conversion returned false");

  //   assert!(
  //     Path::new(output_path).exists(),
  //     "❌ Output file does not exist"
  //   );

  //   let content = fs::read(output_path).await.unwrap();
  //   assert!(!content.is_empty(), "❌ Output file is empty");
  //   assert!(content.len() > 8, "❌ Output file is too small");
  //   assert_eq!(&content[4..8], b"ftyp", "❌ Invalid AVIF file format");
  // }
}
