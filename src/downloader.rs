use std::error;
use std::fs::File;
use std::io::Write;

pub struct ImageDownloader;

impl ImageDownloader {
  pub fn download_image(url: &str, filename: &str) -> Result<(), Box<dyn error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = File::create(filename)?;
    file.write_all(&response.bytes()?)?;
    Ok(())
  }
}
