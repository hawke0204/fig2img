use std::io;
use std::process::Command;

pub struct ImageConverter;

impl ImageConverter {
  pub fn convert_to_webp(input_file: &str, output_file: &str) -> Result<(), io::Error> {
    let status = Command::new("cwebp")
      .arg("-quiet")
      .arg(input_file)
      .arg("-o")
      .arg(output_file)
      .status()?;

    if status.success() {
      println!("✅ Converted {} → {}", input_file, output_file);
    } else {
      eprintln!("❌ Failed to convert {}", input_file);
    }

    Ok(())
  }
}
