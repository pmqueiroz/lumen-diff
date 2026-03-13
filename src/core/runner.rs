use crate::models::Story;
use crate::config::LumenConfig;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use futures::StreamExt;
use std::path::Path;
use tokio::fs;

struct SnapshotTask {
  story: Story,
  breakpoint: u32,
}

pub async fn run_snapshots(stories: Vec<Story>, port: u16, config: &LumenConfig) -> Result<(), Box<dyn std::error::Error>> {
  println!("🚀 Starting snapshot runner with {} stories", stories.len());

  let (mut browser, mut handler) = Browser::launch(
    BrowserConfig::builder().build()?,
  )
  .await?;

  let handler_task = tokio::spawn(async move {
    while let Some(h) = handler.next().await {
      if let Err(e) = h {
        eprintln!("❌ Browser event error: {}", e);
      }
    }
  });

  let output_dir = Path::new(".lumendiff/baseline");
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

  println!("📸 Capturing snapshots...");

  futures::stream::iter(tasks)
    .map(|task| {
      let browser = &browser;
      let output_dir = &output_dir;

      async move {
        let full_url = format!("http://localhost:{}/{}", port, task.story.url);
        let safe_filename = task.story.title.replace("--", "_").replace("-", "_").replace(" ", "_").replace("/", "_");
        let filepath = output_dir.join(format!("{}_{}.png", safe_filename, task.breakpoint));

        let page = match browser.new_page(full_url).await {
          Ok(p) => p,
          Err(e) => {
            eprintln!("❌ Failed to open page for '{}': {}", task.story.title, e);
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
          eprintln!("❌ Failed to set viewport for '{}': {}", task.story.title, e);
          let _ = page.close().await;
          return;
        }

        tokio::time::sleep(std::time::Duration::from_millis(config.wait_before_screenshot.unwrap_or(500))).await;

        let screenshot_result = page.save_screenshot(
          chromiumoxide::page::ScreenshotParams::builder()
            .format(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
            .build(),
            &filepath,
        ).await;

        match screenshot_result {
          Ok(_) => println!("✅ Saved snapshot for '{}' at '{}'", task.story.title, filepath.display()),
          Err(e) => eprintln!("❌ Failed to save snapshot for '{}': {}", task.story.title, e),
        }

        let _ = page.close().await;
      }
    })
    .buffer_unordered(config.concurrency.unwrap_or(8))
    .collect::<Vec<_>>()
    .await;

  println!("✅ Snapshots saved to '{}'", output_dir.display());

  browser.close().await?;
  handler_task.await?;

  Ok(())
}
