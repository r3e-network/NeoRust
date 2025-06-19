#!/bin/bash
set -e

# Update Version Script for NeoRust
# Usage: ./scripts/update-version.sh <new-version>

if [ $# -eq 0 ]; then
    echo "Usage: $0 <new-version>"
    echo "Example: $0 0.5.0"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (basic semver check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\.-]+)?$ ]]; then
    echo "Error: Invalid version format. Use semantic versioning (e.g., 1.0.0 or 1.0.0-beta.1)"
    exit 1
fi

echo "🔄 Updating version to $NEW_VERSION..."

# Update root Cargo.toml
echo "📦 Updating root Cargo.toml..."
sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" Cargo.toml

# Update neo-cli Cargo.toml
echo "💻 Updating neo-cli Cargo.toml..."
sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" neo-cli/Cargo.toml

# Update neo-gui Cargo.toml
echo "🖥️  Updating neo-gui Cargo.toml..."
sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" neo-gui/Cargo.toml

# Update neo-gui package.json
echo "📱 Updating neo-gui package.json..."
sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" neo-gui/package.json

# Update neo-gui tauri.conf.json
echo "🦀 Updating neo-gui tauri.conf.json..."
sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" neo-gui/tauri.conf.json

# Update all example Cargo.toml files
echo "📚 Updating example Cargo.toml files..."
find examples -name "Cargo.toml" -exec sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" {} \;

# Update future crate Cargo.toml files (when workspace is reorganized)
if [ -d "crates" ]; then
    echo "📦 Updating crate Cargo.toml files..."
    find crates -name "Cargo.toml" -exec sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" {} \;
fi

# Update version references in README.md
echo "📖 Updating README.md version references..."
sed -i.bak "s/neo3 = \"[^\"]*\"/neo3 = \"$NEW_VERSION\"/" README.md

# Update version references in documentation
echo "📚 Updating documentation version references..."
find docs -name "*.md" -exec sed -i.bak "s/neo3 = \"[^\"]*\"/neo3 = \"$NEW_VERSION\"/" {} \;

# Clean up backup files
echo "🧹 Cleaning up backup files..."
find . -name "*.bak" -delete

# Update Cargo.lock
echo "🔒 Updating Cargo.lock..."
cargo update

echo "✅ Version updated to $NEW_VERSION successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Update CHANGELOG.md with release notes"
echo "2. Review and commit changes: git add -A && git commit -m 'Bump version to $NEW_VERSION'"
echo "3. Create and push tag: git tag v$NEW_VERSION && git push origin v$NEW_VERSION"
echo "4. GitHub Actions will automatically handle the release"