# CI/CD & Release Process

## Overview

Two workflows drive the release pipeline:

```
push to main
  └── cd.yml (release-please)
        └── creates/updates Release PR
              └── merge Release PR
                    └── tag created (e.g. v0.2.0)
                          └── release.yml (cargo-dist)
                                ├── plan
                                ├── build-local-artifacts (5 platforms, parallel)
                                ├── build-global-artifacts (checksums, shell installer)
                                ├── host → create GitHub Release
                                ├── publish-homebrew-formula → pmqueiroz/homebrew-tap
                                └── announce
```

---

## cd.yml — Versioning (release-please)

**Trigger:** push to `main`

**Tool:** `googleapis/release-please-action@v4`, `release-type: rust`

**What it does:**
- Parses conventional commits since last release
- Bumps version in `Cargo.toml`
- Updates `CHANGELOG.md`
- Opens (or updates) a Release PR
- When Release PR is merged: creates the version tag (e.g. `v0.2.0`)

**Key flag:** `skip-github-release: true` — release-please only creates the tag; the actual GitHub Release is created by cargo-dist in `release.yml`.

**Required secret:** `RELEASE_PLEASE_TOKEN`

**Required permissions:** `contents: write`, `pull-requests: write`

### Commit convention

| Commit prefix | Version bump |
|---------------|-------------|
| `fix:` | patch (0.0.X) |
| `feat:` | minor (0.X.0) |
| `feat!:` or `BREAKING CHANGE:` | major (X.0.0) |
| `chore:`, `docs:`, `refactor:` | no bump |

---

## release.yml — Distribution (cargo-dist)

**Trigger:**
- `push` on tags matching `**[0-9]+.[0-9]+.[0-9]+*`
- `pull_request` (dry-run only — runs `dist plan`, no publish)

**Tool:** cargo-dist v0.31.0

**Required secrets:** `GITHUB_TOKEN` (auto), `HOMEBREW_TAP_TOKEN`

### Jobs

#### 1. plan

Runs on `ubuntu-22.04`. Installs cargo-dist, runs `dist plan` (on PRs) or `dist host --steps=create` (on tags). Outputs the build matrix used by all downstream jobs.

#### 2. build-local-artifacts

Runs in parallel across all target platforms:

| Target | Runner |
|--------|--------|
| `aarch64-apple-darwin` | macos (ARM) |
| `x86_64-apple-darwin` | macos (Intel) |
| `aarch64-unknown-linux-gnu` | ubuntu-22.04 (cross) |
| `x86_64-unknown-linux-gnu` | ubuntu-22.04 |
| `x86_64-pc-windows-msvc` | windows |

Each runner: checks out repo, installs dist, compiles binary with `dist build`, uploads artifact zip + per-platform manifest.

#### 3. build-global-artifacts

Runs on `ubuntu-22.04` after all local builds complete. Produces:
- Shell installer script (`lumendiff-installer.sh`)
- SHA256 checksums for all artifacts

#### 4. host

Runs only when `publishing == true` (tag push, not PR). Downloads all artifacts, runs `dist host --steps=upload --steps=release`, creates the GitHub Release with generated title/body from CHANGELOG, attaches all artifacts.

Pre-release versions (e.g. `v0.2.0-beta.1`) are automatically marked as pre-release on GitHub.

#### 5. publish-homebrew-formula

Skipped for pre-releases (unless `publish_prereleases` is set).

Checks out `pmqueiroz/homebrew-tap` using `HOMEBREW_TAP_TOKEN`, copies the generated `.rb` formula file into `Formula/`, runs `brew style --fix`, commits and pushes.

#### 6. announce

Final job. Runs after `host` and `publish-homebrew-formula`. Currently a no-op checkpoint — exists to add announcement integrations (Slack, Discord, etc.) if needed.

---

## dist-workspace.toml

```toml
[dist]
cargo-dist-version = "0.31.0"
ci = "github"
installers = ["shell", "homebrew"]
tap = "pmqueiroz/homebrew-tap"
targets = [
  "aarch64-apple-darwin",
  "aarch64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-pc-windows-msvc",
]
install-path = "CARGO_HOME"
publish-jobs = ["homebrew"]
install-updater = false
```

`install-path = "CARGO_HOME"` installs binary to `~/.cargo/bin/` (same as `cargo install`).

`install-updater = false` disables the self-update binary that cargo-dist can optionally ship.

---

## Required Secrets

| Secret | Used by | Purpose |
|--------|---------|---------|
| `GITHUB_TOKEN` | release.yml | Create releases, upload artifacts |
| `RELEASE_PLEASE_TOKEN` | cd.yml | Open/update Release PRs, create tags |
| `HOMEBREW_TAP_TOKEN` | release.yml | Push formula to `pmqueiroz/homebrew-tap` |

---

## GitHub Action (action.yml)

Lets other repos consume lumendiff in CI without installing Rust.

```yaml
- uses: pmqueiroz/lumen-diff@v0.2.0
  with:
    version: 'latest'   # optional
```

Downloads pre-compiled binary from GitHub Releases, adds to `$HOME/.cargo/bin`, runs `lumendiff`.

| Input | Default | Description |
|-------|---------|-------------|
| `version` | `latest` | Release tag to download |

---

## Making a Release (step by step)

1. Merge PRs to `main` using conventional commit messages (`feat:`, `fix:`, etc.)
2. `cd.yml` opens a Release PR automatically — review the version bump and CHANGELOG
3. Merge the Release PR
4. Tag is created → `release.yml` triggers
5. Monitor Actions tab: `plan → build-local-artifacts → build-global-artifacts → host → publish-homebrew-formula → announce`
6. GitHub Release appears with binaries for all 5 platforms + shell installer + checksums
7. Homebrew tap updated: `brew upgrade pmqueiroz/tap/lumendiff` works immediately
