use clap::Parser;

mod cli;

use cli::{Cli, Commands};
use fig2img::commands::download::DownloadOptions;
use fig2img::commands::{self};

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Download { download_dir } => {
      commands::download::execute(download_dir, DownloadOptions::default()).await;
    }
    Commands::Convert {
      input_dir,
      output_dir,
      format,
    } => {
      commands::convert::execute(input_dir, output_dir, format).await;
    }
  }
}
