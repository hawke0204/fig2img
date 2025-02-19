use std::io::{Error, ErrorKind};
use std::process::Command;

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

  pub fn convert_to_webp(input_file: &str, output_file: &str) -> Result<bool, Error> {
    if !Self::check_cwebp_installed() {
      Self::print_installation_guide();
      return Err(Error::new(ErrorKind::NotFound, "cwebp command not found"));
    }

    let status = Command::new("cwebp")
      .arg("-quiet")
      .arg(input_file)
      .arg("-o")
      .arg(output_file)
      .status()?;

    Ok(status.success())
  }
}
