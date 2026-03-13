use crate::cli::{Cli, ProviderType};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LumenConfig {
  pub storybook_shots: StorybookShotsConfig,
  pub wait_before_screenshot: u64,
  pub concurrency: usize,
  pub threshold: f64,
  pub update: bool,
  pub provider: ProviderType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct StorybookShotsConfig {
  pub storybook_url: String,
  pub breakpoints: Vec<u32>,
}

impl Default for StorybookShotsConfig {
  fn default() -> Self {
    Self {
      storybook_url: "storybook-static".to_string(),
      breakpoints: vec![1280],
    }
  }
}

impl Default for LumenConfig {
  fn default() -> Self {
    Self {
      storybook_shots: StorybookShotsConfig::default(),
      wait_before_screenshot: 500,
      concurrency: 8,
      threshold: 0.05,
      update: false,
      provider: ProviderType::Storybook,
    }
  }
}

impl LumenConfig {
  pub fn apply_cli_args(&mut self, args: &Cli) {
    if let Some(url) = &args.storybook_url {
      self.storybook_shots.storybook_url = url.clone();
    }
    if let Some(provider) = &args.provider {
      self.provider = provider.clone();
    }
    if let Some(update) = args.update {
      self.update = update;
    }
  }
}

pub fn load_config() -> LumenConfig {
  let config_path = Path::new(".lumendiff.yaml");

  if config_path.exists() {
    if let Ok(file_content) = fs::read_to_string(config_path) {
      match serde_yaml::from_str::<LumenConfig>(&file_content) {
        Ok(config) => {
          println!("✅ Loaded configuration from .lumendiff.yaml");
          return config;
        }
        Err(e) => eprintln!("❌ Failed to parse .lumendiff.yaml: {}", e),
      }
    }
  } else {
    println!("⚠️ No .lumendiff.yaml found, using default configuration");
  }

  LumenConfig::default()
}
