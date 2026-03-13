use crate::models::Story;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::path::Path;
use tokio::fs;

pub async fn run_snapshots(stories: Vec<Story>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
  println!("🚀 Starting snapshot runner with {} stories", stories.len());

  let (mut browser, mut handler) = Browser::launch(
    BrowserConfig::builder().with_head().build()?,
  )
  .await?;

  let handler_task = tokio::spawn(async move {
    while let Some(h) = handler.next().await {
      if let Err(e) = h {
        eprintln!("❌ Browser event error: {}", e);
        // break;
      }
    }
  });

  let output_dir = Path::new(".lumendiff/baseline");
  fs::create_dir_all(output_dir).await?;

  println!("📸 Capturing snapshots...");
  let page = browser.new_page("about:blank").await?;

  for story in stories {
    let full_url = format!("http://localhost:{}/{}", port, story.url);
    println!("🔍 Processing story at: {}", full_url);

    let _ = page.goto("about:blank").await;

    page.goto(&full_url).await?;
    
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    let safe_filename = story.title.replace("--", "_").replace("-", "_").replace(" ", "_").replace("/", "_");
    let filepath = output_dir.join(format!("{}.png", safe_filename));

    page.save_screenshot(
      chromiumoxide::page::ScreenshotParams::builder()
        .format(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
        .build(),
        &filepath,
    ).await?;
  }

  println!("✅ Snapshots saved to '{}'", output_dir.display());

  browser.close().await?;
  handler_task.await?;

  Ok(())
}
