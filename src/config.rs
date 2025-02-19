use config::{Config, File as ConfigFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FigmaConfig {
  pub figma_access_token: String,
  pub figma_api_url: String,
  pub figma_file_key: String,
  // [Deprecated]
  // pub download_folder: String,
  // pub output_folder: String,
}

impl FigmaConfig {
  pub fn new() -> Self {
    let settings = Config::builder()
      .add_source(ConfigFile::with_name("config"))
      .build()
      .unwrap();

    settings.get::<FigmaConfig>("figma").unwrap()
  }
}
