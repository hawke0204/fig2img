mod avif;
mod webp;

use std::io::Error;

pub struct ImageConverter;

impl ImageConverter {
  pub async fn convert_to_webp(input_path: &str, output_path: &str) -> Result<bool, Error> {
    webp::WebPConverter::convert(input_path, output_path).await
  }

  pub async fn convert_to_avif(input_path: &str, output_path: &str) -> Result<bool, Error> {
    avif::AvifConverter::convert(input_path, output_path).await
  }
}
