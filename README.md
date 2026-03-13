<div align="center">

# LumenDiff ⚡

High-performance visual regression testing tool written in Rust. 

LumenDiff drastically reduces the time spent on CI/CD visual regression pipelines by leveraging Rust's zero-cost abstractions, multi-threading, and raw memory access.

</div>


## ✨ Features

- **Extreme Speed:** Multi-threaded screenshot capture via CDP (`chromiumoxide`) and parallel CPU-bound image diffing using `rayon`.
- **Seamless Integration:** Compatible with existing Storybook static builds and familiar configuration structures.
- **Zero-overhead Diffing:** Custom raw-byte pixel comparison algorithm that skips expensive PNG decoding for unchanged components.
- **Built-in Static Server:** Instant local static file serving powered by `axum` and `tokio`.
- **Framework Agnostic:** Currently supports Storybook (v6 & v7+), with architecture ready for Ladle and Histoire.

## 🚀 Quick Start

### 1. Installation

You can install LumenDiff directly via Cargo:

```bash
cargo install lumendiff

```

### 2. Usage

Build your Storybook locally:

```bash
npm run build-storybook

```

Run LumenDiff in the root of your project:

```bash
lumendiff

```

LumenDiff will automatically read your `.lumendiff.yaml` (if available), spin up a static server, capture the snapshots, and compare them against your baseline.

## ⚙️ Configuration

LumenDiff can be configured via a `.lumendiff.yaml` file in the root of your project. All fields are optional and will fallback to sensible defaults.

```yaml
# .lumendiff.yaml
storybookShots:
  storybookUrl: ./storybook-static
  breakpoints: 
    - 360
    - 1280
waitBeforeScreenshot: 500
concurrency: 8
threshold: 0.05

```

### CLI Arguments

You can override configuration values directly from the terminal:

```bash
lumendiff --storybook-dir ./dist/docs --concurrency 12 --threshold 0.02

```

Run `lumendiff --help` for a full list of commands.

## 📁 Directory Structure

After running the tool, LumenDiff manages your visual state in the `.lumendiff` folder:

* `.lumendiff/baseline/`: The accepted source of truth for your components.
* `.lumendiff/snapshots/`: The latest captured screenshots.
* `.lumendiff/diffs/`: Highlighted diff images for components that failed the threshold.

## 🛣️ Roadmap

* [x] Storybook support
* [x] Parallel diffing & capturing
* [x] Threshold configuration
* [ ] Support for Ladle
* [ ] Support for Histoire
* [ ] CI/CD GitHub Action
