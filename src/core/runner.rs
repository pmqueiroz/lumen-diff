use crate::config::LumenConfig;
use crate::models::Story;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use futures::StreamExt;
use std::path::Path;
use tokio::fs;
use tracing::{error, info};

struct SnapshotTask {
  story: Story,
  breakpoint: u32,
}

pub async fn run_snapshots(
  stories: Vec<Story>,
  port: u16,
  config: &LumenConfig,
) -> Result<(), Box<dyn std::error::Error>> {
  info!("🚀 Starting snapshot runner with {} stories", stories.len());

  let (mut browser, mut handler) = Browser::launch(
    BrowserConfig::builder()
      .arg("--no-sandbox")
      .arg("--disabled-setupid-sandbox")
      .arg("--disable-dev-shm-usage")
      .arg("--disable-gpu")
      .build()?,
  )
  .await?;

  let handler_task = tokio::spawn(async move {
    while let Some(h) = handler.next().await {
      if let Err(e) = h {
        error!("❌ Browser event error: {}", e);
      }
    }
  });

  let output_dir = Path::new(".lumendiff/snapshots");
  fs::create_dir_all(output_dir).await?;

  let mut tasks = Vec::new();
  for story in stories {
    for &breakpoint in &config.storybook_shots.breakpoints {
      tasks.push(SnapshotTask {
        story: story.clone(),
        breakpoint: breakpoint,
      });
    }
  }

  info!("📸 Capturing snapshots...");

  futures::stream::iter(tasks)
    .map(|task| {
      let browser = &browser;
      let output_dir = &output_dir;

      async move {
        let full_url = format!("http://localhost:{}/{}", port, task.story.url);
        let filepath = output_dir.join(format!("{}__[w{}px].png", task.story.id, task.breakpoint));

        let page = match browser.new_page(full_url).await {
          Ok(p) => p,
          Err(e) => {
            error!("❌ Failed to open page for '{}': {}", task.story.id, e);
            return;
          }
        };

        let metrics = SetDeviceMetricsOverrideParams::builder()
          .width(task.breakpoint as i64)
          .height(1080)
          .device_scale_factor(1.0)
          .mobile(task.breakpoint < 768)
          .build()
          .unwrap();

        if let Err(e) = page.execute(metrics).await {
          error!("❌ Failed to set viewport for '{}': {}", task.story.id, e);
          let _ = page.close().await;
          return;
        }

        tokio::time::sleep(std::time::Duration::from_millis(
          config.wait_before_screenshot,
        ))
        .await;

        let screenshot_result = page
          .save_screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
              .format(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
              .build(),
            &filepath,
          )
          .await;

        match screenshot_result {
          Ok(_) => info!(
            "✅ Saved snapshot for '{}' at '{}'",
            task.story.id,
            filepath.display()
          ),
          Err(e) => error!("❌ Failed to save snapshot for '{}': {}", task.story.id, e),
        }

        let _ = page.close().await;
      }
    })
    .buffer_unordered(config.concurrency)
    .collect::<Vec<_>>()
    .await;

  info!("✅ Snapshots saved to '{}'", output_dir.display());

  browser.close().await?;
  handler_task.await?;

  Ok(())
}
