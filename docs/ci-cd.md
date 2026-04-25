# CI/CD Integration

## GitHub Action

Defined in `action.yml`. Downloads pre-compiled binary from GitHub releases and runs `lumendiff`.

```yaml
# .github/workflows/visual-regression.yml
steps:
  - uses: actions/checkout@v4

  - name: Build Storybook
    run: npm run build-storybook

  - name: Run LumenDiff
    uses: pmqueiroz/lumen-diff@v0.2.0
    with:
      version: 'latest'   # optional, defaults to latest
```

Action inputs:
| Input | Default | Description |
|-------|---------|-------------|
| `version` | `"latest"` | Release version to download |

Action icon: camera | color: orange

## Release Pipeline (release.yml)

Uses **cargo-dist**. Triggered on version tags (`*[0-9]+.[0-9]+.[0-9]+*`).

Builds for:
| Target | Platform |
|--------|----------|
| `aarch64-apple-darwin` | macOS ARM (Apple Silicon) |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-unknown-linux-gnu` | Linux ARM64 |
| `x86_64-unknown-linux-gnu` | Linux x86_64 |
| `x86_64-pc-windows-msvc` | Windows x86_64 |

Installer formats:
- Shell script (`lumendiff-installer.sh`)
- Homebrew formula → published to `pmqueiroz/homebrew-tap`

## Versioning (cd.yml)

Uses **release-please**. Triggered on push to `main`.

- Parses conventional commits
- Maintains `CHANGELOG.md`
- Opens release PRs automatically
- Merging release PR triggers tag → release pipeline

Commit format:
```
feat: add new feature        → minor bump
fix: fix a bug               → patch bump
feat!: breaking change       → major bump
```

## dist-workspace.toml

```toml
[dist]
cargo-dist-version = "..."
installers = ["shell", "homebrew"]
tap = "pmqueiroz/homebrew-tap"
publish-jobs = ["homebrew"]
targets = [
  "aarch64-apple-darwin",
  "aarch64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-pc-windows-msvc"
]
```

## Typical CI Workflow

```
push to main
  └── release-please creates/updates release PR
        └── merge release PR
              └── tag created (e.g. v0.2.0)
                    └── cargo-dist builds all targets
                          └── publishes GitHub Release + Homebrew formula
```
