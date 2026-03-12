use crate::models::Story;
use async_trait::async_trait;
use std::error::Error;

pub mod storybook;

#[async_trait]
pub trait StoryProvider {
  fn name(&self) -> &'static str;

  async fn fetch_stories(&self, base_url: &str) -> Result<Vec<Story>, Box<dyn Error>>;
}
