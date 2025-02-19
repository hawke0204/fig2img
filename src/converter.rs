use std::io;
use std::process::Command;

pub struct ImageConverter;

impl ImageConverter {
  pub fn convert_to_webp(input_file: &str, output_file: &str) -> Result<bool, io::Error> {
    let status = Command::new("cwebp")
      .arg("-quiet")
      .arg(input_file)
      .arg("-o")
      .arg(output_file)
      .status()?;

    Ok(status.success())
  }
}
