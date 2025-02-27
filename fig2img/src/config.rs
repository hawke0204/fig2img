use std::env;

use config::{Config, File as ConfigFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FigmaConfig {
  pub figma_access_token: String,
  pub figma_file_key: String,
  // [Deprecated]
  // pub figma_api_url: String,
  // pub download_folder: String,
  // pub output_folder: String,
}

impl FigmaConfig {
  pub fn new() -> Self {
    let config_builder = Config::builder();

    let config_result = config_builder
      .add_source(ConfigFile::with_name("config"))
      .build();

    if let Some(settings) = config_result.ok() {
      if let Ok(config) = settings.get::<FigmaConfig>("figma") {
        return config;
      }
    }

    FigmaConfig {
      figma_access_token: env::var("FIGMA_ACCESS_TOKEN")
        .expect("FIGMA_ACCESS_TOKEN environment variable not set"),
      figma_file_key: env::var("FIGMA_FILE_KEY")
        .expect("FIGMA_FILE_KEY environment variable not set"),
    }
  }
}
