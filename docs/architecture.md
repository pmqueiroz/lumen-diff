# Architecture

## Module Structure

```
src/
├── main.rs              # Entry point — orchestrates full workflow
├── cli.rs               # CLI argument parsing (clap)
├── config.rs            # Config loading and merging
├── logger.rs            # tracing initialization
├── server.rs            # Static HTTP server (axum)
├── models/
│   └── mod.rs           # Story struct
├── providers/
│   ├── mod.rs           # StoryProvider trait
│   └── storybook.rs     # Storybook implementation
└── core/
    ├── mod.rs           # Re-exports
    ├── runner.rs        # Screenshot capture
    └── diff.rs          # Image comparison
```

## Data Flow

```
main.rs
  │
  ├── load config (.lumendiff.yaml + CLI overrides)
  ├── start axum static server on :1337
  ├── provider.fetch_stories(storybook_url)
  │     └── parse index.json → Vec<Story>
  ├── runner::run_snapshots(stories, config)
  │     ├── launch Chromium (CDP via chromiumoxide)
  │     ├── for each story × breakpoint (buffered concurrency):
  │     │     ├── open new page
  │     │     ├── set viewport (width, height=1080)
  │     │     ├── navigate to iframe.html?id=…
  │     │     ├── wait N ms
  │     │     └── capture PNG → .lumendiff/snapshots/
  │     └── close browser
  └── diff::run_diffs(config)
        ├── for each baseline in .lumendiff/baseline/ (rayon parallel):
        │     ├── load baseline image
        │     ├── load matching snapshot
        │     ├── compare raw bytes (early exit if identical)
        │     ├── pixel-by-pixel RGBA diff
        │     ├── compute similarity score
        │     └── if score < threshold: write diff image → .lumendiff/diffs/
        └── report pass/fail
```

## Key Design Decisions

**Trait-based providers** — `StoryProvider` allows plugging in Ladle, Histoire, or custom sources without touching the core pipeline. Currently only `StorybookProvider` is implemented.

**Raw byte comparison before decode** — if baseline and snapshot byte arrays are identical, the diff loop is skipped entirely. Avoids PNG decoding cost on unchanged stories.

**Async screenshot capture, parallel diff** — screenshot capture uses `futures::stream::buffer_unordered` (I/O bound, chromium pages). Diff computation uses `rayon` (CPU bound, image processing).

**Separate baseline / snapshot directories** — baselines are committed to source control; snapshots are ephemeral. Diff images are only written on failure.

**Chromium via CDP** — `chromiumoxide` drives Chrome/Chromium directly over the Chrome DevTools Protocol. No Puppeteer or Playwright overhead.

## Dependencies

| Crate | Version | Role |
|-------|---------|------|
| `tokio` | 1.50.0 | Async runtime (full features) |
| `axum` | 0.8.8 | Static file server |
| `tower-http` | 0.6.8 | `ServeDir` middleware |
| `reqwest` | 0.13.2 | HTTP client (fetch index.json) |
| `chromiumoxide` | 0.9.1 | Chrome DevTools Protocol client |
| `image` | 0.25.10 | PNG load/save |
| `rayon` | 1.11.0 | Parallel diff processing |
| `serde` / `serde_json` / `serde_yaml` | latest | Config + index.json parsing |
| `clap` | 4.6.0 | CLI argument parsing |
| `heck` | 0.5.0 | kebab-case story ID generation |
| `tracing` + `tracing-subscriber` | 0.1.44 / 0.3.23 | Structured logging |
| `async-trait` | 0.1.89 | Async fn in traits |
| `futures` | 0.3.32 | Stream utilities |

## Build Profiles

```toml
[profile.dist]
inherits = "release"
lto = "thin"   # Thin LTO for distribution builds
```
