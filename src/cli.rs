use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
  name = "lumen-diff",
  version,
  about = "Blazing fast visual regression testing (according to copilot)"
)]
pub struct Cli {
  #[arg(long)]
  pub storybook_url: Option<String>,

  #[arg(short, long, value_enum)]
  pub provider: Option<ProviderType>,

  #[arg(long)]
  pub update: Option<bool>,
}

#[derive(ValueEnum, Debug, Clone, serde::Deserialize)]
pub enum ProviderType {
  Storybook,
  Ladle,
  Histoire,
}
