use std::time::Instant;

use image_proc::commands;

#[tokio::main]
async fn main() {
  let path = std::path::PathBuf::from("./downloads");

  let start = Instant::now();
  commands::download::execute(path).await;
  let download_duration = start.elapsed();

  let input_path = std::path::PathBuf::from("./downloads");
  let output_path = std::path::PathBuf::from("./output");
  let format = "webp".to_string();

  let start = Instant::now();
  commands::convert::execute(input_path, output_path, format).await;
  let convert_to_webp_duration = start.elapsed();

  println!(
    "Download execution took: {:.2}s",
    download_duration.as_secs_f64()
  );
  println!(
    "Convert to webp execution took: {:.2}s",
    convert_to_webp_duration.as_secs_f64()
  );

  let all_duration = download_duration + convert_to_webp_duration;
  println!("Total execution took: {:.2}s", all_duration.as_secs_f64());
}
