# Output & Artifacts

## Directory Layout

```
.lumendiff/
├── baseline/      # Source of truth — commit these to the repo
├── snapshots/     # Current run screenshots — ephemeral, gitignored
└── diffs/         # Diff images for failing stories — ephemeral
```

`.gitignore` excludes `.lumendiff/` entirely by default. Selectively commit `baseline/` to source control.

## Snapshot Naming

```
{story-id}__[w{breakpoint}px].png
```

Examples:
```
button--primary__[w1280px].png
button--primary__[w360px].png
card--with-image__[w768px].png
form-input--disabled__[w1280px].png
```

Story ID is derived from Storybook `title` and `name` fields converted to kebab-case:
- `title: "Button"` + `name: "Primary"` → `button--primary`
- `title: "Form/Input"` + `name: "Disabled"` → `form-input--disabled`

## Diff Images

Generated only for stories that exceed the threshold.

Pixel encoding:
- **Unchanged pixel**: RGBA from baseline with alpha = 75 (semi-transparent, ~29% opacity)
- **Changed pixel**: (255, 0, 0, 255) — solid red

Location: `.lumendiff/diffs/{snapshot-name}.png`

## Console Output

Uses `tracing` with structured log levels. Default format has no timestamps.

```
INFO  lumendiff: Starting server on :1337
INFO  lumendiff: Fetched 42 stories
INFO  lumendiff: Captured button--primary__[w1280px].png
WARN  lumendiff: Diff failed: card--default__[w360px].png (score: 0.93, threshold: 0.95)
ERROR lumendiff: Snapshot missing for header--nav__[w1280px].png
```

Control verbosity with `RUST_LOG`:
```bash
RUST_LOG=warn lumendiff    # only warnings and errors
RUST_LOG=info lumendiff    # standard output (default)
RUST_LOG=debug lumendiff   # verbose
RUST_LOG=trace lumendiff   # everything
```
