# Provider System

## StoryProvider Trait (providers/mod.rs)

```rust
#[async_trait]
pub trait StoryProvider {
    fn name(&self) -> &'static str;
    async fn fetch_stories(&self, source: &str) -> Result<Vec<Story>, Box<dyn Error>>;
}
```

`source` = path to the static build directory (value of `storybookUrl`).

## Story Model (models/mod.rs)

```rust
pub struct Story {
    pub id: String,   // kebab-case identifier used in snapshot filenames
    pub url: String,  // relative URL: iframe.html?id=…&viewMode=story
}
```

## Implementations

### StorybookProvider

Reads `{source}/index.json`. Supports Storybook v6 and v7+ (both use `entries` map).

Story ID construction:
```
title = "Button Group"  →  "button-group"
name  = "With Icon"     →  "with-icon"
id    = "button-group--with-icon"
```

Docs entries (`type: "docs"`) are filtered out.

### Ladle (stub)

Not implemented. `fetch_stories` will panic or return empty.

### Histoire (stub)

Not implemented. `fetch_stories` will panic or return empty.

## ProviderType Enum

```rust
pub enum ProviderType {
    Storybook,
    Ladle,
    Histoire,
}
```

Set via `--provider` CLI flag or omitted to use default (`Storybook`).

## Adding a New Provider

1. Create `src/providers/my_provider.rs`
2. Implement `StoryProvider` trait
3. Add variant to `ProviderType` enum in `providers/mod.rs`
4. Add match arm in `main.rs` where provider is instantiated
