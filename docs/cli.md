# CLI Reference

## Binary

`lumendiff`

## Synopsis

```
lumendiff [OPTIONS]
```

## Options

| Flag | Type | Description |
|------|------|-------------|
| `--storybook-url <PATH>` | `String` | Override Storybook static build path |
| `--provider <PROVIDER>` | `ProviderType` | Story source: `Storybook`, `Ladle`, `Histoire` |
| `--update` | flag | Update baseline snapshots instead of diffing |
| `--help` | flag | Print help |
| `--version` | flag | Print version |

## Examples

```bash
# Run with config file defaults
lumendiff

# Custom Storybook output path
lumendiff --storybook-url ./dist/storybook

# Update baselines (after intentional UI changes)
lumendiff --update

# Explicit provider
lumendiff --provider Storybook

# Combine flags
lumendiff --storybook-url ./out --provider Storybook --update

# Debug logging
RUST_LOG=debug lumendiff
```

## Workflow

```bash
# First run — create baselines
lumendiff --update

# Subsequent runs — compare against baselines
lumendiff

# After intentional UI changes — update baselines
lumendiff --update
```

## Installation

```bash
# Via cargo
cargo install lumendiff

# Via shell installer (from release)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/pmqueiroz/lumen-diff/releases/latest/download/lumendiff-installer.sh | sh

# Via Homebrew
brew install pmqueiroz/tap/lumendiff
```
