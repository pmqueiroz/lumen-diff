use crate::config::LumenConfig;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

pub fn run_diff(config: &LumenConfig) -> Result<(), Box<dyn std::error::Error>> {
  println!(
    "🔍 Running visual diff with threshold: {}",
    config.threshold.unwrap_or(0.05)
  );

  let snapshots_dir = Path::new(".lumendiff/snapshots");
  let baseline_dir = Path::new(".lumendiff/baseline");
  let diffs_dir = Path::new(".lumendiff/diffs");

  fs::create_dir_all(baseline_dir)?;
  fs::create_dir_all(diffs_dir)?;

  let entries: Vec<_> = fs::read_dir(snapshots_dir)?
    .filter_map(Result::ok)
    .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "png"))
    .collect();

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

      let img_snapshot = image::open(&snapshot_path)
        .expect("Failed to open snapshot image")
        .to_rgba8();
      let img_baseline = image::open(&baseline_path)
        .expect("Failed to open baseline image")
        .to_rgba8();

      match compare_images(&img_snapshot, &img_baseline) {
        Ok((score, diff_image)) => {
          let min_score_accepted = 1.0 - config.threshold.unwrap_or(0.05);

          if score >= min_score_accepted {
            println!(
              "✅ {} is similar to baseline (score: {:.4})",
              filename.to_string_lossy(),
              score
            );
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
  let (snap_width, snap_height) = snapshot.dimensions();

  if width != snap_width || height != snap_height {
    return Err(format!(
      "Image dimensions do not match: baseline is {}x{}, snapshot is {}x{}",
      width, height, snap_width, snap_height
    ));
  }

  let mut diff_image = RgbaImage::new(width, height);
  let mut diff_pixels = 0;

  for y in 0..height {
    for x in 0..width {
      let p1 = baseline.get_pixel(x, y);
      let p2 = snapshot.get_pixel(x, y);
      let is_diff = p1[0] != p2[0] || p1[1] != p2[1] || p1[2] != p2[2];

      if is_diff {
        diff_pixels += 1;
        diff_image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
      } else {
        diff_image.put_pixel(x, y, Rgba([p1[0], p1[1], p1[2], 75]));
      }
    }
  }

  let total_pixels = width * height;
  let similarity_score = 1.0 - (diff_pixels as f64 / total_pixels as f64);

  Ok((similarity_score, diff_image))
}
