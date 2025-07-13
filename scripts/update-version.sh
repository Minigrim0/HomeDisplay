#!/bin/bash
set -e

# Script to update version across all project files
# Usage: ./scripts/update-version.sh <new-version>

NEW_VERSION="$1"

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: $0 <new-version>"
    echo "Example: $0 0.7.0"
    exit 1
fi

# Validate version format (basic semver check)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$'; then
    echo "Error: Version must follow semantic versioning (e.g., 1.2.3, 1.2.3-alpha, 1.2.3+build)"
    exit 1
fi

echo "Updating project version to $NEW_VERSION..."

# Update workspace Cargo.toml
echo "Updating workspace version..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update tauri.conf.json
echo "Updating Tauri configuration..."
if command -v jq >/dev/null 2>&1; then
    # Use jq if available for proper JSON handling
    jq --arg version "$NEW_VERSION" '.version = $version' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp
    mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
else
    # Fallback to sed
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" src-tauri/tauri.conf.json
fi

# Verify the changes
echo "Verifying version updates..."
echo "Workspace version: $(grep '^version =' Cargo.toml | cut -d'"' -f2)"
echo "Tauri version: $(grep '"version"' src-tauri/tauri.conf.json | cut -d'"' -f4)"

# Check cargo metadata to ensure all crates inherit the version correctly
echo "Checking crate versions..."
cargo metadata --format-version 1 | jq -r '.packages[] | select(.name | test("^(homedisplay|hd-tui|frontend|hd-tauri)$")) | "\(.name): \(.version)"'

echo "Version update completed successfully!"
echo ""
echo "Next steps:"
echo "1. Review the changes: git diff"
echo "2. Commit the changes: git add . && git commit -m 'Bump version to $NEW_VERSION'"
echo "3. Create a tag: git tag v$NEW_VERSION"
echo "4. Push with tags: git push origin main --tags"
echo "5. Create a GitHub release to trigger builds"