use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chrono::Local;
use cli_table::{print_stdout, Style, Table};
use colored::*;
use fig2img::commands::download::DownloadOptions;
use fig2img::commands::{self};

struct BenchmarkResult {
  input_count: usize,
  webp_count: usize,
  avif_count: usize,
  download_duration: Duration,
  convert_to_webp_duration: Duration,
  convert_to_avif_duration: Duration,
}

impl BenchmarkResult {
  fn total_duration(&self) -> Duration {
    self.download_duration + self.convert_to_webp_duration + self.convert_to_avif_duration
  }

  fn display_results(&self) {
    let rows = vec![
      vec![
        "📁 Files".into(),
        format!(
          "{} → {}(webp), {}(avif)",
          self.input_count, self.webp_count, self.avif_count
        ),
      ],
      vec![
        "⬇️  Download".into(),
        format!("{:.2}s", self.download_duration.as_secs_f64()),
      ],
      vec![
        "🔄 Convert(WebP)".into(),
        format!("{:.2}s", self.convert_to_webp_duration.as_secs_f64()),
      ],
      vec![
        "🔄 Convert(AVIF)".into(),
        format!("{:.2}s", self.convert_to_avif_duration.as_secs_f64()),
      ],
      vec![
        "📈 Total".bold().to_string(),
        format!("{:.2}s", self.total_duration().as_secs_f64())
          .bold()
          .to_string(),
      ],
    ];

    let table = rows
      .table()
      .title(vec![
        "Operation".bold().to_string(),
        "Time".bold().to_string(),
      ])
      .bold(true);

    println!("\n{}", "📊 Benchmark Results".bold().blue());
    print_stdout(table).unwrap();
    println!();
  }

  fn save_to_csv(&self) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let result = format!(
      "{},{},\"{}→{}(webp), {}(avif)\",{:.2}s,{:.2}s,{:.2}s,{:.2}s\n",
      timestamp,
      "Files",
      self.input_count,
      self.webp_count,
      self.avif_count,
      self.download_duration.as_secs_f64(),
      self.convert_to_webp_duration.as_secs_f64(),
      self.convert_to_avif_duration.as_secs_f64(),
      self.total_duration().as_secs_f64()
    );

    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open("benchmark_results.csv")
      .expect("Failed to open file");

    if file.metadata().unwrap().len() == 0 {
      file.write_all(
                b"timestamp,operation,value,download_time,webp_convert_time,avif_convert_time,total_time\n",
            ).expect("Failed to write header");
    }

    file
      .write_all(result.as_bytes())
      .expect("Failed to write results");
  }
}

async fn run_benchmark(
  downloads_path: &PathBuf,
  output_webp_path: &PathBuf,
  output_avif_path: &PathBuf,
) -> BenchmarkResult {
  let start = Instant::now();
  let options = DownloadOptions::new().quiet(true);
  commands::download::execute(downloads_path.clone(), options).await;
  let download_duration = start.elapsed();

  let start = Instant::now();
  commands::convert::execute(
    downloads_path.clone(),
    output_webp_path.clone(),
    "webp".to_string(),
  )
  .await;
  let convert_to_webp_duration = start.elapsed();

  let start = Instant::now();
  commands::convert::execute(
    downloads_path.clone(),
    output_avif_path.clone(),
    "avif".to_string(),
  )
  .await;
  let convert_to_avif_duration = start.elapsed();

  let input_count = fs::read_dir(downloads_path)
    .map(|entries| entries.count())
    .unwrap_or(0);
  let webp_count = fs::read_dir(output_webp_path)
    .map(|entries| entries.count())
    .unwrap_or(0);
  let avif_count = fs::read_dir(output_avif_path)
    .map(|entries| entries.count())
    .unwrap_or(0);

  BenchmarkResult {
    input_count,
    webp_count,
    avif_count,
    download_duration,
    convert_to_webp_duration,
    convert_to_avif_duration,
  }
}

fn clear_directory(path: &std::path::Path) {
  if path.exists() {
    fs::remove_dir_all(path).expect("Failed to remove directory");
  }
  fs::create_dir_all(path).expect("Failed to create directory");
}

#[tokio::main]
async fn main() {
  let downloads_path = std::path::PathBuf::from("./downloads");
  let output_webp_path = std::path::PathBuf::from("./output/webp");
  let output_avif_path = std::path::PathBuf::from("./output/avif");

  clear_directory(&downloads_path);
  clear_directory(&output_webp_path);
  clear_directory(&output_avif_path);

  let result = run_benchmark(&downloads_path, &output_webp_path, &output_avif_path).await;
  result.display_results();
  result.save_to_csv();
}
