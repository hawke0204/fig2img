use std::time::Instant;

use fig2img::commands::download::DownloadOptions;
use fig2img::commands::{self};

#[tokio::main]
async fn main() {
  let path = std::path::PathBuf::from("./benchmarks/downloads");

  let start = Instant::now();
  let options = DownloadOptions::new().quiet(true);

  commands::download::execute(path, options).await;
  let download_duration = start.elapsed();

  let input_path = std::path::PathBuf::from("./benchmarks/downloads");
  let output_path = std::path::PathBuf::from("./benchmarks/output");
  let format = "webp".to_string();

  let start = Instant::now();
  commands::convert::execute(input_path.clone(), output_path.clone(), format).await;
  let convert_to_webp_duration = start.elapsed();

  let format = "avif".to_string();
  let start = Instant::now();
  commands::convert::execute(input_path, output_path, format).await;
  let convert_to_avif_duration = start.elapsed();

  println!(
    "Download execution took: {:.2}s",
    download_duration.as_secs_f64()
  );
  println!(
    "Convert to webp execution took: {:.2}s",
    convert_to_webp_duration.as_secs_f64()
  );
  println!(
    "Convert to avif execution took: {:.2}s",
    convert_to_avif_duration.as_secs_f64()
  );

  let all_duration = download_duration + convert_to_webp_duration + convert_to_avif_duration;
  println!("Total execution took: {:.2}s", all_duration.as_secs_f64());
}
