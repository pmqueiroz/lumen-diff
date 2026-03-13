mod cli;
mod config;
mod core;
mod logger;
mod models;
mod providers;
mod server;

use clap::Parser;
use cli::{Cli, ProviderType};
use providers::{storybook::StorybookProvider, StoryProvider};
use std::time::Duration;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  logger::init();
  let mut config = config::load_config();
  let args = Cli::parse();
  config.apply_cli_args(&args);

  info!("🚀 Starting lumen diff");

  let server_dir = config.storybook_shots.storybook_url.clone();
  let port = 1337;
  tokio::spawn(async move {
    if let Err(e) = server::start(server_dir, port).await {
      error!("❌ Server error: {}", e);
    }
  });
  tokio::time::sleep(Duration::from_millis(100)).await;

  let provider: Box<dyn StoryProvider> = match config.provider {
    ProviderType::Storybook => Box::new(StorybookProvider::new()),
    ProviderType::Ladle => unimplemented!("Ladle provider is not implemented yet"),
    ProviderType::Histoire => unimplemented!("Histoire provider is not implemented yet"),
  };
  info!("📦 Using provider: {}", provider.name());

  let stories = provider.fetch_stories(&config.storybook_shots.storybook_url).await?;
  info!("✅ Fetched {} stories", stories.len());

  core::runner::run_snapshots(stories, port, &config).await?;

  if let Err(e) = core::diff::run_diffs(&config) {
    error!("❌ Diff error: {}", e);
  }

  info!("🎉 Lumen diff completed successfully!");

  Ok(())
}
