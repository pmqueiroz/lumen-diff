use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
  name = "lumen-diff",
  version,
  about = "Blazing fast visual regression testing (according to copilot)"
)]
pub struct Cli {
  #[arg(short, long, default_value = "http://localhost:6006")]
  pub url: String,

  #[arg(short, long, value_enum, default_value_t = ProviderType::Storybook)]
  pub provider: ProviderType,

  #[arg(short, long)]
  pub update: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ProviderType {
  Storybook,
  Ladle,
  Histoire,
}
