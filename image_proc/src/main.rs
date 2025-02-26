use clap::Parser;

mod cli;

use cli::{Cli, Commands};
use image_proc::commands;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Download { download_dir } => {
      commands::download::execute(download_dir).await;
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
