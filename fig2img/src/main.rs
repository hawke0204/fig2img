use clap::Parser;

mod cli;

use cli::{Cli, Commands};
use fig2img::commands::download::DownloadOptions;
use fig2img::commands::{self};

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Download { output } => {
      commands::download::execute(output, DownloadOptions::default()).await;
    }
    Commands::Convert {
      input,
      output,
      format,
    } => {
      commands::convert::execute(input, output, format).await;
    }
  }
}
