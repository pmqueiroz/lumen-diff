use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LumenConfig {
  pub storybook_shots: StorybookShotsConfig,
  pub wait_before_screenshot: Option<u64>,
  pub concurrency: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorybookShotsConfig {
  pub storybook_url: String,
  pub breakpoints: Vec<u32>,
}

impl Default for LumenConfig {
  fn default() -> Self {
    Self {
      storybook_shots: StorybookShotsConfig {
        storybook_url: "storybook-static".to_string(),
        breakpoints: vec![1280],
      },
      wait_before_screenshot: Some(500),
      concurrency: Some(8),
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
