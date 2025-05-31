# Release Workflow Documentation

This document explains how to use the automated release workflow for NeoRust.

## Overview

The release workflow automatically builds and creates binary applications for Linux, macOS, and Windows when a new version is released. It also publishes crates to crates.io and updates documentation.

## Supported Platforms

The workflow builds binaries for the following platforms:

### Linux
- `x86_64-unknown-linux-gnu` (Intel/AMD 64-bit)
- `aarch64-unknown-linux-gnu` (ARM 64-bit)

### macOS
- `x86_64-apple-darwin` (Intel Macs)
- `aarch64-apple-darwin` (Apple Silicon Macs)

### Windows
- `x86_64-pc-windows-msvc` (64-bit Windows)
- `i686-pc-windows-msvc` (32-bit Windows)

## How to Trigger a Release

### Method 1: Push a Git Tag (Recommended)

1. **Update version numbers** in all relevant files:
   ```bash
   # Update main Cargo.toml
   sed -i 's/version = "0.2.0"/version = "0.3.0"/g' Cargo.toml
   
   # Update CLI Cargo.toml
   sed -i 's/version = "0.2.0"/version = "0.3.0"/g' neo-cli/Cargo.toml
   sed -i 's/neo3.*version = "0.2.0"/neo3 = { path = "..", version = "0.3.0" }/g' neo-cli/Cargo.toml
   
   # Update GUI Cargo.toml
   sed -i 's/version = "0.2.0"/version = "0.3.0"/g' neo-gui/Cargo.toml
   sed -i 's/neo3.*version = "0.2.0"/neo3 = { path = "..", version = "0.3.0" }/g' neo-gui/Cargo.toml
   ```

2. **Update CHANGELOG.md** with the new version:
   ```markdown
   ## [0.3.0] - 2025-06-01
   
   ### Added
   - New feature descriptions
   
   ### Changed
   - Changes made in this version
   
   ### Fixed
   - Bug fixes in this version
   ```

3. **Commit and push changes**:
   ```bash
   git add .
   git commit -m "chore: bump version to 0.3.0"
   git push origin master
   ```

4. **Create and push a git tag**:
   ```bash
   git tag -a v0.3.0 -m "Release v0.3.0"
   git push origin v0.3.0
   ```

### Method 2: Manual Workflow Dispatch

You can also trigger the release manually from the GitHub Actions tab:

1. Go to the **Actions** tab in your GitHub repository
2. Select the **Release** workflow
3. Click **Run workflow**
4. Enter the tag name (e.g., `v0.3.0`)
5. Click **Run workflow**

## What the Workflow Does

### 1. Create Release
- Extracts changelog information for the version
- Creates a GitHub release with release notes
- Sets up the release for binary uploads

### 2. Build Binaries
- Builds CLI and GUI binaries for all supported platforms
- Creates individual binary files and compressed archives
- Uploads all binaries to the GitHub release

### 3. Publish Crates
- Publishes the `neo3` crate to crates.io
- Publishes the `neo-cli` crate to crates.io
- Handles crate propagation delays

### 4. Update Documentation
- Updates README.md with new version numbers
- Updates download links to point to the latest release
- Commits changes back to the repository

### 5. Notify Completion
- Provides a summary of the release process
- Shows the release URL

## Generated Assets

For each release, the following assets are created:

### Individual Binaries
- `neo-cli-linux-x86_64` - CLI for Linux x64
- `neo-cli-linux-aarch64` - CLI for Linux ARM64
- `neo-cli-macos-x86_64` - CLI for Intel Macs
- `neo-cli-macos-aarch64` - CLI for Apple Silicon Macs
- `neo-cli-windows-x86_64.exe` - CLI for Windows x64
- `neo-cli-windows-i686.exe` - CLI for Windows x86

- `neo-gui-linux-x86_64` - GUI for Linux x64 (if available)
- `neo-gui-linux-aarch64` - GUI for Linux ARM64 (if available)
- `neo-gui-macos-x86_64` - GUI for Intel Macs (if available)
- `neo-gui-macos-aarch64` - GUI for Apple Silicon Macs (if available)
- `neo-gui-windows-x86_64.exe` - GUI for Windows x64 (if available)
- `neo-gui-windows-i686.exe` - GUI for Windows x86 (if available)

### Compressed Archives
- `neorust-linux-x86_64.tar.gz` - All Linux x64 binaries
- `neorust-linux-aarch64.tar.gz` - All Linux ARM64 binaries
- `neorust-macos-x86_64.tar.gz` - All Intel Mac binaries
- `neorust-macos-aarch64.tar.gz` - All Apple Silicon Mac binaries
- `neorust-windows-x86_64.zip` - All Windows x64 binaries
- `neorust-windows-i686.zip` - All Windows x86 binaries

## Prerequisites

### Repository Secrets

The workflow requires the following secrets to be set in your GitHub repository:

1. **`CARGO_REGISTRY_TOKEN`** - Token for publishing to crates.io
   - Go to [crates.io](https://crates.io/me) and generate an API token
   - Add it as a repository secret in GitHub

### Permissions

The workflow needs the following permissions:
- **Contents**: write (to create releases and update files)
- **Actions**: write (to run workflows)

## Troubleshooting

### Common Issues

#### 1. Crate Publishing Fails
```
Error: cargo publish failed
```
**Solution**: Check that:
- The `CARGO_REGISTRY_TOKEN` secret is set correctly
- The version number hasn't been published before
- All dependencies are available on crates.io

#### 2. Binary Build Fails
```
Error: cross compilation failed
```
**Solution**: 
- Check that the target platform is supported
- Verify that all dependencies support cross-compilation
- Check for platform-specific code issues

#### 3. GUI Build Fails
```
Error: neo-gui build failed
```
**Solution**:
- GUI builds are marked as `continue-on-error: true`
- Check the GUI dependencies and build requirements
- Ensure Tauri dependencies are properly configured

### Monitoring Releases

1. **Check the Actions tab** for workflow progress
2. **Monitor the Releases page** for published releases
3. **Verify crates.io** for published crates
4. **Test downloaded binaries** on target platforms

## Best Practices

### Version Management
- Follow [Semantic Versioning](https://semver.org/)
- Update CHANGELOG.md before each release
- Test thoroughly before creating release tags

### Release Notes
- Write clear, descriptive changelog entries
- Include breaking changes prominently
- Mention new features and bug fixes

### Testing
- Run the full test suite before releasing
- Test binaries on target platforms when possible
- Verify that the CLI and GUI work as expected

## Example Release Process

Here's a complete example of releasing version 0.3.0:

```bash
# 1. Update version numbers
sed -i 's/version = "0.2.0"/version = "0.3.0"/g' Cargo.toml
sed -i 's/version = "0.2.0"/version = "0.3.0"/g' neo-cli/Cargo.toml
sed -i 's/neo3.*version = "0.2.0"/neo3 = { path = "..", version = "0.3.0" }/g' neo-cli/Cargo.toml

# 2. Update CHANGELOG.md (manually edit the file)

# 3. Test the changes
cargo test --all-features
cd neo-cli && cargo test --all-features

# 4. Commit changes
git add .
git commit -m "chore: bump version to 0.3.0"
git push origin master

# 5. Create and push tag
git tag -a v0.3.0 -m "Release v0.3.0: Add new features and improvements"
git push origin v0.3.0

# 6. Monitor the workflow in GitHub Actions
# 7. Verify the release was created successfully
# 8. Test downloaded binaries
```

## Support

If you encounter issues with the release workflow:

1. Check the [GitHub Actions logs](https://github.com/R3E-Network/NeoRust/actions)
2. Review this documentation
3. Open an issue in the repository
4. Contact the maintainers

---

**Happy releasing! 🚀** 