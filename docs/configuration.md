# Configuration

## .lumendiff.yaml

Place in project root. All fields optional.

```yaml
storybookShots:
  storybookUrl: ./storybook-static   # path to Storybook static build directory
  breakpoints: [360, 768, 1280]      # viewport widths (px) to capture

waitBeforeScreenshot: 500            # ms to wait after page load before capture
concurrency: 8                       # max concurrent screenshot tasks
threshold: 0.05                      # max acceptable diff ratio (0.0–1.0, i.e. 5%)
```

## Defaults

| Field | Default |
|-------|---------|
| `storybookUrl` | `"storybook-static"` |
| `breakpoints` | `[1280]` |
| `waitBeforeScreenshot` | `500` |
| `concurrency` | `8` |
| `threshold` | `0.05` (5%) |
| `update` | `false` |
| `provider` | `Storybook` |

## CLI Overrides

CLI flags take precedence over config file values.

| Flag | Config equivalent |
|------|------------------|
| `--storybook-url <PATH>` | `storybookShots.storybookUrl` |
| `--provider <PROVIDER>` | `provider` |
| `--update` | `update` |

Override priority: **CLI > .lumendiff.yaml > built-in defaults**

## Environment Variables

| Variable | Effect |
|----------|--------|
| `RUST_LOG` | Controls log verbosity via `tracing-subscriber` env-filter |

Examples:
```bash
RUST_LOG=info lumendiff
RUST_LOG=debug lumendiff
RUST_LOG=lumendiff=trace lumendiff
```

## threshold

Controls how much pixel difference is acceptable before a story is marked as failed.

```
threshold: 0.05   → up to 5% of pixels may differ
threshold: 0.0    → zero tolerance (exact match required)
threshold: 1.0    → always pass (disables regression detection)
```

Internally:
```
min_score_accepted = 1.0 - threshold
similarity_score   = 1.0 - (diff_pixels / total_pixels)

PASS if similarity_score >= min_score_accepted
FAIL otherwise → diff image written to .lumendiff/diffs/
```
