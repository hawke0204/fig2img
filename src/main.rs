use clap::Parser;

mod cli;
mod commands;
mod config;
mod core;

use cli::{Cli, Commands};

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
