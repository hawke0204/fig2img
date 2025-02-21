use std::path::PathBuf;
use std::sync::Arc;

use converter::ImageConverter;
use futures::future;
use tokio::fs;
use tokio::sync::Semaphore;

use crate::core::converter;

pub async fn execute(input_dir: PathBuf, output_dir: PathBuf, format: String) {
  if format != "webp" && format != "avif" {
    eprintln!("[❌] Unsupported format: {}", format);
    return;
  }

  if let Err(e) = fs::create_dir_all(&output_dir).await {
    eprintln!("[❌] Failed to create output directory: {}", e);
    return;
  }

  match fs::read_dir(&input_dir).await {
    Ok(mut entries) => {
      let mut conversion_tasks = Vec::new();
      let semaphore = Arc::new(Semaphore::new(4));

      while let Some(entry) = entries.next_entry().await.unwrap() {
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "png") {
          let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
          let output_path = output_dir.join(format!("{}.{}", &file_stem, format));

          let input_path = path.to_str().unwrap().to_string();
          let output_path = output_path.to_str().unwrap().to_string();
          let format = format.clone();

          let semaphore = Arc::clone(&semaphore);

          conversion_tasks.push(tokio::spawn(async move {
            let _ = semaphore.acquire().await.unwrap();

            let result = match format.as_str() {
              "webp" => ImageConverter::convert_to_webp(&input_path, &output_path).await,
              "avif" => ImageConverter::convert_to_avif(&input_path, &output_path).await,
              _ => unreachable!(),
            };

            match result {
              Ok(_) => println!("[✅] Converted: {} -> {}", input_path, output_path),
              Err(e) => eprintln!("[❌] Failed conversion: {}", e),
            }
          }));
        }
      }

      if let Err(e) = future::join_all(conversion_tasks)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
      {
        eprintln!("[❌] Some conversions failed: {}", e);
      }
    }
    Err(e) => eprintln!("[❌] Failed to read input directory: {}", e),
  }
}
