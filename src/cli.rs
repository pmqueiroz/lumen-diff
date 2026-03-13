use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
  name = "lumen-diff",
  version,
  about = "Blazing fast visual regression testing (according to copilot)"
)]
pub struct Cli {
  #[arg(long, default_value = "storybook-static")]
  pub storybook_url: String,

  #[arg(short, long, value_enum, default_value_t = ProviderType::Storybook)]
  pub provider: ProviderType,

  #[arg(long)]
  pub update: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ProviderType {
  Storybook,
  Ladle,
  Histoire,
}
