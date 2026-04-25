# LumenDiff Technical Documentation

Visual regression testing tool built in Rust. Captures Storybook screenshots at configurable breakpoints and diffs them against baselines.

## Contents

- [Architecture](./architecture.md) — module structure, data flow, design decisions
- [Configuration](./configuration.md) — `.lumendiff.yaml` reference, CLI flags, env vars
- [CLI Reference](./cli.md) — commands, flags, invocation examples
- [Core Algorithms](./algorithms.md) — screenshot pipeline, image diffing, similarity scoring
- [Provider System](./providers.md) — `StoryProvider` trait, Storybook implementation, stubs
- [Output & Artifacts](./output.md) — directory layout, snapshot naming, diff images
- [CI/CD Integration](./ci-cd.md) — GitHub Action, release pipeline, multi-platform builds

## Quick Start

```bash
# Build Storybook
npm run build-storybook

# Run LumenDiff (reads .lumendiff.yaml)
lumendiff

# Update baselines
lumendiff --update

# Results in .lumendiff/
```

## Version

Current: **0.2.0** | Edition: 2024 | License: MIT
