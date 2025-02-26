use std::time::Instant;

use image_proc::commands;

#[tokio::main]
async fn main() {
  let path = std::path::PathBuf::from("./downloads");

  let start = Instant::now();
  commands::download::execute(path).await;
  let duration = start.elapsed();

  println!("Download execution took: {:?}", duration);
}
