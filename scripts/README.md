# Version Management

This project uses Cargo workspace version inheritance to maintain consistent versions across all crates.

## Current Setup

- **Workspace version**: Defined in root `Cargo.toml`
- **All crates**: Inherit version using `version.workspace = true`
- **Tauri config**: Manually synchronized with workspace version

## Updating Versions

### Method 1: Manual Script (Recommended)

```bash
# Update to new version (e.g., 0.8.0)
./scripts/update-version.sh 0.8.0

# Review changes
git diff

# Commit and tag
git add .
git commit -m "Bump version to 0.8.0"
git tag v0.8.0
git push origin main --tags
```

### Method 2: GitHub Actions (Automated)

1. Go to **Actions** tab in GitHub
2. Select **"Create Release"** workflow
3. Click **"Run workflow"**
4. Enter the new version (e.g., `0.8.0`)
5. Choose if it's a pre-release
6. Click **"Run workflow"**

This will:
- Update all version files
- Commit the changes
- Create a git tag
- Create a GitHub release
- Trigger build workflows automatically

## Version Files Updated

- `Cargo.toml` (workspace version)
- `src-tauri/tauri.conf.json` (Tauri version)
- All crate `Cargo.toml` files inherit from workspace

## Build Triggers

Creating a GitHub release will automatically trigger:
- **Raspberry Pi TUI build** (`build.yml`)
- **x86_64 TUI + Tauri builds** (`build-x86_64.yml`)

## Versioning Strategy

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

## Troubleshooting

### Version mismatch errors
```bash
# Check current versions
cargo metadata --format-version 1 | jq -r '.packages[] | select(.name | test("^(homedisplay|hd-tui|frontend|hd-tauri)$")) | "\(.name): \(.version)"'

# Fix workspace inheritance
cargo check
```

### Tauri version out of sync
```bash
# Check tauri.conf.json version
grep '"version"' src-tauri/tauri.conf.json

# Update manually if needed
./scripts/update-version.sh $(grep '^version =' Cargo.toml | cut -d'"' -f2)
```