use std::io::Error;
use std::process::Command;

pub(super) struct WebPConverter;

impl WebPConverter {
  pub async fn convert(input_path: &str, output_path: &str) -> Result<bool, Error> {
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
    // println!("Ubuntu: sudo apt-get install webp");
    // println!("Windows: choco install webp");
    println!("Or download from: https://developers.google.com/speed/webp/download");
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use tempfile::NamedTempFile;

  use super::*;

  fn create_test_image() -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let mut img = image::RgbaImage::new(100, 100);

    for pixel in img.pixels_mut() {
      *pixel = image::Rgba([255, 0, 0, 255]);
    }

    let img = image::DynamicImage::ImageRgba8(img);
    img
      .save_with_format(temp_file.path(), image::ImageFormat::Png)
      .unwrap();
    temp_file
  }

  #[tokio::test]
  async fn test_webp_conversion() {
    let input_file = create_test_image();
    let output_file = NamedTempFile::new().unwrap();

    let checked_cwebp = WebPConverter::check_cwebp_installed();
    assert!(checked_cwebp, "❌ cwebp is not installed");

    let result = WebPConverter::convert(
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
}
