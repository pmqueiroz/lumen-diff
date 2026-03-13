use super::StoryProvider;
use crate::models::Story;
use async_trait::async_trait;
use heck::ToKebabCase;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tokio::fs;

#[derive(Deserialize)]
struct StorybookIndex {
  entries: HashMap<String, StorybookEntry>,
}

#[derive(Deserialize)]
struct StorybookEntry {
  name: String,
  id: String,
  title: String,
  #[serde(default)]
  #[serde(rename = "type")]
  entry_type: String,
}

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

  async fn fetch_stories(&self, source: &str) -> Result<Vec<Story>, Box<dyn Error>> {
    let path = Path::new(source).join("index.json");
    
    println!("Fetching stories from Storybook at {}", path.display());

    let file_content = fs::read_to_string(&path).await?;
    let index: StorybookIndex = serde_json::from_str(&file_content)?;

    let mut stories = Vec::new();

    for (_, entry) in index.entries {
      if entry.entry_type == "docs" {
        continue;
      }

      stories.push(Story {
        id: build_story_id(&entry),
        url: format!("iframe.html?id={}&viewMode=story", entry.id),
      });
    }
  
    Ok(stories)
  }
}

fn build_story_id(raw_story: &StorybookEntry) -> String {
  let parsed_title = raw_story.title.replace("/", "-").to_kebab_case();
  let parsed_name = raw_story.name.replace("/", "-").to_kebab_case();
  format!("{}--{}", parsed_title, parsed_name)
}
