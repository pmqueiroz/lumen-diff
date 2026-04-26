use crate::config::LumenConfig;
use image::RgbaImage;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

pub fn run_diffs(config: &LumenConfig) -> Result<(), Box<dyn std::error::Error>> {
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
    warn!("⚠️ No snapshots found in .lumendiff/snapshots");
    return Ok(());
  }

  if config.update {
    info!("🔄 Updating baselines from snapshots...");
    let updated: usize = entries
      .par_iter()
      .filter_map(|entry| {
        let snapshot_path = entry.path();
        let filename = snapshot_path.file_name().unwrap();
        let baseline_path = baseline_dir.join(filename);
        fs::copy(&snapshot_path, &baseline_path).ok().map(|_| 1)
      })
      .sum();
    info!("✅ Updated {} baselines", updated);
    return Ok(());
  }

  info!(
    "🔍 Running visual diff with threshold: {}",
    config.threshold
  );

  let results: Vec<bool> = entries
    .par_iter()
    .map(|entry| {
      let snapshot_path = entry.path();
      let filename = snapshot_path.file_name().unwrap();
      let baseline_path = baseline_dir.join(filename);
      let diff_path = diffs_dir.join(filename);

      if !baseline_path.exists() {
        warn!(
          "⚠️ No baseline found for {}, skipping diff",
          filename.to_string_lossy()
        );
        let _ = fs::copy(&snapshot_path, &baseline_path);
        return true;
      }

      let snap_bytes = match fs::read(&snapshot_path) {
        Ok(b) => b,
        Err(e) => {
          error!("❌ Failed to read snapshot {}: {}", filename.to_string_lossy(), e);
          return false;
        }
      };
      let base_bytes = match fs::read(&baseline_path) {
        Ok(b) => b,
        Err(e) => {
          error!("❌ Failed to read baseline {}: {}", filename.to_string_lossy(), e);
          return false;
        }
      };

      if snap_bytes == base_bytes {
        return true;
      }

      // Decode from already-read bytes — avoids second disk read per image
      let img_snapshot = match image::load_from_memory(&snap_bytes) {
        Ok(img) => img.into_rgba8(),
        Err(e) => {
          error!("❌ Failed to decode snapshot {}: {}", filename.to_string_lossy(), e);
          return false;
        }
      };
      let img_baseline = match image::load_from_memory(&base_bytes) {
        Ok(img) => img.into_rgba8(),
        Err(e) => {
          error!("❌ Failed to decode baseline {}: {}", filename.to_string_lossy(), e);
          return false;
        }
      };

      let (width, height) = img_baseline.dimensions();
      if (width, height) != img_snapshot.dimensions() {
        error!(
          "❌ Dimension mismatch for {}: {}x{} vs {:?}",
          filename.to_string_lossy(),
          width,
          height,
          img_snapshot.dimensions()
        );
        return false;
      }

      let base_raw = img_baseline.as_raw();
      let snap_raw = img_snapshot.as_raw();
      let total_pixels = (width * height) as usize;

      // Pass 1: count-only, no allocation — skips diff image work for passing stories
      let diff_pixels = base_raw
        .chunks_exact(4)
        .zip(snap_raw.chunks_exact(4))
        .filter(|(b, s)| b != s)
        .count();

      let similarity_score = 1.0 - (diff_pixels as f64 / total_pixels as f64);
      let min_score_accepted = 1.0 - config.threshold;

      if similarity_score >= min_score_accepted {
        return true;
      }

      error!(
        "❌ {} differs from baseline (score: {:.4}), saving diff image",
        filename.to_string_lossy(),
        similarity_score
      );

      // Pass 2: build diff image only for failing stories
      let diff_image = build_diff_image(base_raw, snap_raw, width, height);
      if let Err(e) = diff_image.save(&diff_path) {
        error!("❌ Failed to save diff image for {}: {}", filename.to_string_lossy(), e);
      }

      false
    })
    .collect();

  let failures = results.iter().filter(|&&passed| !passed).count();

  if failures > 0 {
    error!("❌ {} diffs found that exceed the threshold", failures);
    error!("📁 Check the .lumendiff/diffs directory for details");
  } else {
    info!("✅ All snapshots are within the acceptable threshold");
  }

  Ok(())
}

fn build_diff_image(base_raw: &[u8], snap_raw: &[u8], width: u32, height: u32) -> RgbaImage {
  let mut diff_raw = vec![0u8; base_raw.len()];

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
      d_chunk[0] = 255;
      d_chunk[1] = 0;
      d_chunk[2] = 0;
      d_chunk[3] = 255;
    }
  }

  RgbaImage::from_raw(width, height, diff_raw).expect("❌ Failed to create diff image")
}
