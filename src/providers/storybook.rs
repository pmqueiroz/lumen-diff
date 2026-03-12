use super::StoryProvider;
use crate::models::Story;
use async_trait::async_trait;
use std::error::Error;

pub struct StorybookProvider;

impl StorybookProvider {
  pub fn new() -> Self {
    Self
  }
}

#[async_trait]
impl StoryProvider for StorybookProvider {
  fn name(&self) -> &'static str {
    "Storybook"
  }

  async fn fetch_stories(&self, base_url: &str) -> Result<Vec<Story>, Box<dyn Error>> {
    let url = format!("{}/stories.json", base_url);
    let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    let stories = response["stories"]
      .as_object()
      .ok_or("Invalid response format")?
      .iter()
      .map(|(id, story)| Story {
        id: id.clone(),
        title: story["name"].as_str().unwrap_or("").to_string(),
        url: format!("{}/iframe.html?id={}", base_url, id),
      })
      .collect();

    Ok(stories)
  }
}
