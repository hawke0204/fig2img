use std::io::{Error, ErrorKind};

use image::{DynamicImage, GenericImageView};
use ravif::{Encoder, Img};
use rgb::FromSlice;

pub(super) struct AvifConverter;

impl AvifConverter {
  pub async fn convert(input_path: &str, output_path: &str) -> Result<bool, Error> {
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
  use super::*;

  #[test]
  fn test_encode_image() {
    let img = DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));
    let result = AvifConverter::encode_image(img);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert!(result.len() > 0);
    assert_eq!(&result[4..8], b"ftyp");
  }

  // TODO: Uncomment and fix the async test when the conversion issue is resolved
  // #[tokio::test]
  // async fn test_avif_conversion() { ... }
}
