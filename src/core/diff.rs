use crate::config::LumenConfig;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

pub fn run_diffs(config: &LumenConfig) -> Result<(), Box<dyn std::error::Error>> {
  println!(
    "🔍 Running visual diff with threshold: {}",
    config.threshold.unwrap_or(0.05)
  );

  let snapshots_dir = Path::new(".lumendiff/snapshots");
  let baseline_dir = Path::new(".lumendiff/baseline");
  let diffs_dir = Path::new(".lumendiff/diffs");

  fs::create_dir_all(baseline_dir)?;
  fs::create_dir_all(diffs_dir)?;

  let entries = fs::read_dir(snapshots_dir)?
    .filter_map(Result::ok)
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
    .collect::<Vec<_>>();

  if entries.is_empty() {
    println!("⚠️ No snapshots found in .lumendiff/snapshots");
    return Ok(());
  }

  let results: Vec<bool> = entries
    .par_iter()
    .map(|entry| {
      let snapshot_path = entry.path();
      let filename = snapshot_path.file_name().unwrap();
      let baseline_path = baseline_dir.join(filename);
      let diff_path = diffs_dir.join(filename);

      if !baseline_path.exists() {
        println!(
          "⚠️ No baseline found for {}, skipping diff",
          filename.to_string_lossy()
        );
        let _ = fs::copy(&snapshot_path, &baseline_path);
        return true;
      }

      let snap_bytes = fs::read(&snapshot_path).unwrap_or_default();
      let base_bytes = fs::read(&baseline_path).unwrap_or_default();

      if snap_bytes == base_bytes {
        return true;
      }

      let img_snapshot = image::open(&snapshot_path)
        .expect("❌ Failed to open snapshot")
        .into_rgba8();
      let img_baseline = image::open(&baseline_path)
        .expect("❌ Failed to open baseline")
        .into_rgba8();

      match compare_images(&img_baseline, &img_snapshot) {
        Ok((score, diff_image)) => {
          let min_score_accepted = 1.0 - config.threshold.unwrap_or(0.05);

          if score >= min_score_accepted {
            true
          } else {
            println!(
              "❌ {} differs from baseline (score: {:.4}), saving diff image",
              filename.to_string_lossy(),
              score
            );
            diff_image
              .save(&diff_path)
              .expect("Failed to save diff image");
            false
          }
        }
        Err(e) => {
          eprintln!(
            "❌ Failed to compare images for {}: {}",
            filename.to_string_lossy(),
            e
          );
          false
        }
      }
    })
    .collect();

  let falhas = results.iter().filter(|&&passed| !passed).count();

  if falhas > 0 {
    println!("❌ {} diffs found that exceed the threshold", falhas);
    println!("📁 Check the .lumendiff/diffs directory for details");
  } else {
    println!("✅ All snapshots are within the acceptable threshold");
  }

  Ok(())
}

fn compare_images(baseline: &RgbaImage, snapshot: &RgbaImage) -> Result<(f64, RgbaImage), String> {
  let (width, height) = baseline.dimensions();

  if (width, height) != snapshot.dimensions() {
    return Err(format!(
      "❌ Different dimensions: {}x{} vs {:?}",
      width,
      height,
      snapshot.dimensions()
    ));
  }

  let base_raw = baseline.as_raw();
  let snap_raw = snapshot.as_raw();

  let mut diff_raw = vec![0u8; base_raw.len()];
  let mut diff_pixels = 0;

  for ((b_chunk, s_chunk), d_chunk) in base_raw
    .chunks_exact(4)
    .zip(snap_raw.chunks_exact(4))
    .zip(diff_raw.chunks_exact_mut(4))
  {
    if b_chunk == s_chunk {
      d_chunk[0] = b_chunk[0];
      d_chunk[1] = b_chunk[1];
      d_chunk[2] = b_chunk[2];
      d_chunk[3] = 75;
    } else {
      diff_pixels += 1;
      d_chunk[0] = 255;
      d_chunk[1] = 0;
      d_chunk[2] = 0;
      d_chunk[3] = 255;
    }
  }

  let total_pixels = (width * height) as usize;
  let similarity_score = 1.0 - (diff_pixels as f64 / total_pixels as f64);

  let diff_image =
    RgbaImage::from_raw(width, height, diff_raw).expect("❌ Failed to create diff image");

  Ok((similarity_score, diff_image))
}
