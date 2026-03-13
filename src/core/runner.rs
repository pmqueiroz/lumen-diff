use crate::models::Story;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::path::Path;
use tokio::fs;

pub async fn run_snapshots(stories: Vec<Story>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
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

  println!("📸 Capturing snapshots...");

  // @TODO config through config file
  let concurrency_limit = 8;

  futures::stream::iter(stories)
    .map(|story| {
      let browser = &browser;
      let output_dir = &output_dir;

      async move {
        let full_url = format!("http://localhost:{}/{}", port, story.url);
        let safe_filename = story.title.replace("--", "_").replace("-", "_").replace(" ", "_").replace("/", "_");
        let filepath = output_dir.join(format!("{}.png", safe_filename));

        let page = match browser.new_page(full_url).await {
          Ok(p) => p,
          Err(e) => {
            eprintln!("❌ Failed to open page for '{}': {}", story.title, e);
            return;
          }
        };

        // @TODO config through config file
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let screenshot_result = page.save_screenshot(
          chromiumoxide::page::ScreenshotParams::builder()
            .format(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
            .build(),
            &filepath,
        ).await;

        match screenshot_result {
          Ok(_) => println!("✅ Saved snapshot for '{}' at '{}'", story.title, filepath.display()),
          Err(e) => eprintln!("❌ Failed to save snapshot for '{}': {}", story.title, e),
        }

        let _ = page.close().await;
      }
    })
    .buffer_unordered(concurrency_limit)
    .collect::<Vec<_>>()
    .await;

  println!("✅ Snapshots saved to '{}'", output_dir.display());

  browser.close().await?;
  handler_task.await?;

  Ok(())
}
