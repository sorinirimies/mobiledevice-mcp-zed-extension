# Publishing Guide

This guide covers how to publish the Mobile Device MCP Server to crates.io and the Zed extension marketplace.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Publishing to crates.io](#publishing-to-cratesio)
3. [Publishing to Zed Extensions](#publishing-to-zed-extensions)
4. [Creating GitHub Releases](#creating-github-releases)
5. [Version Management](#version-management)
6. [Post-Release Checklist](#post-release-checklist)

---

## Prerequisites

### Required Tools

```bash
# Rust toolchain
rustup update stable
rustup target add wasm32-wasip1

# Development tools
cargo install just
cargo install git-cliff  # For changelog generation

# Verify installation
cargo --version
rustc --version
```

### Required Accounts

- **crates.io account** - For publishing the native binary
  - Sign up at https://crates.io
  - Generate API token at https://crates.io/settings/tokens
  
- **GitHub account** - For Zed extension submission
  - Fork https://github.com/zed-industries/extensions

---

## Publishing to crates.io

The native binary (`mobile-device-mcp-server`) is published to crates.io so users can install it with `cargo install`.

### 1. Pre-publish Checks

```bash
# Run all tests
just test

# Check formatting and linting
just pre-commit

# Verify the package builds correctly
cargo build --release --features native-binary

# Dry-run publish (no actual upload)
cargo publish --dry-run --features native-binary
```

### 2. Update Version

Edit `Cargo.toml`:

```toml
[package]
version = "0.1.1"  # Increment appropriately
```

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

### 3. Generate Changelog

```bash
# Using git-cliff
just changelog-tag 0.1.1

# Or manually
git-cliff --tag v0.1.1 --output CHANGELOG.md
```

### 4. Commit Version Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v0.1.1"
git push origin main
```

### 5. Login to crates.io (One-Time)

```bash
cargo login
# Paste your API token when prompted
```

### 6. Publish to crates.io

```bash
# Final check
cargo package --features native-binary --list

# Publish!
cargo publish --features native-binary

# Verify it's live
cargo search mobile-device-mcp
```

**What gets published:**
- Native binary target (`mobile-device-mcp-server`)
- All source code in `src/`
- Documentation and LICENSE
- Does NOT include WASM extension (that's separate for Zed)

### 7. Create Git Tag

```bash
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

This triggers the GitHub Actions release workflow.

---

## Publishing to Zed Extensions

The WASM extension is published through the Zed extensions repository.

### 1. Build WASM Extension

```bash
# Build release WASM
just build-wasm

# Or manually
cargo build --release --target wasm32-wasip1 --lib

# Verify output
ls -lh target/wasm32-wasip1/release/mobile_device_mcp.wasm
```

### 2. Prepare Extension Package

The extension needs these files:

```
extension/
├── extension.toml           # Extension metadata
├── mobile_device_mcp.wasm   # Compiled WASM binary
├── LICENSE                  # License file
└── README.md                # Extension documentation
```

Package them:

```bash
# Create extension directory
mkdir -p extension

# Copy files
cp target/wasm32-wasip1/release/mobile_device_mcp.wasm extension/
cp extension.toml extension/
cp LICENSE extension/
cp README.md extension/

# Create tarball
tar czf mobile-device-mcp-extension.tar.gz extension/
```

### 3. Update Extension Version

Edit `extension.toml`:

```toml
id = "mcp-server-mobile-device"
name = "Mobile Device MCP Server"
version = "0.1.1"  # Match crates.io version
# ... rest of config
```

### 4. Fork Zed Extensions Repository

```bash
# Fork on GitHub: https://github.com/zed-industries/extensions

# Clone your fork
git clone https://github.com/YOUR_USERNAME/extensions.git zed-extensions
cd zed-extensions

# Create extension directory
mkdir -p extensions/mcp-server-mobile-device
```

### 5. Add Extension Files

```bash
# Copy extension files to Zed extensions repo
cp ../mobile-device-mcp-extension/extension/* extensions/mcp-server-mobile-device/

# Verify structure
tree extensions/mcp-server-mobile-device/
```

Expected structure in Zed repo:

```
extensions/mcp-server-mobile-device/
├── extension.toml
├── mobile_device_mcp.wasm
├── LICENSE
└── README.md
```

### 6. Submit Pull Request

```bash
# Create branch
git checkout -b add-mobile-device-mcp

# Add files
git add extensions/mcp-server-mobile-device/
git commit -m "Add Mobile Device MCP Server extension"

# Push to your fork
git push origin add-mobile-device-mcp
```

Create a PR to `zed-industries/extensions` with:
- **Title:** `Add Mobile Device MCP Server extension`
- **Description:** Link to your repository, describe functionality
- **Checklist:** Follow Zed's contribution guidelines

### 7. Wait for Review

The Zed team will review your extension. They may request:
- Code changes
- Documentation improvements
- Testing on specific platforms

---

## Creating GitHub Releases

GitHub releases are **automatically created** when you push a version tag.

### Automatic Release (Recommended)

```bash
# Tag and push (triggers CI/CD)
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

The `.github/workflows/release.yml` workflow will:
1. Run tests on Linux and Windows
2. Build native binaries for Linux and Windows
3. Build WASM extension
4. Create GitHub release with all artifacts

**Artifacts created:**
- `mobile-device-mcp-linux-x86_64.tar.gz`
- `mobile-device-mcp-windows-x86_64.zip`
- `mobile-device-mcp-extension.tar.gz` (WASM)

### Manual Release (If Needed)

```bash
# Build all targets
just build-release  # Native binaries
just build-wasm     # WASM extension

# Create release on GitHub
gh release create v0.1.1 \
  --title "v0.1.1" \
  --notes "See CHANGELOG.md for details" \
  target/release/mobile-device-mcp-server \
  target/wasm32-wasip1/release/mobile_device_mcp.wasm
```

---

## Version Management

### Version Synchronization

Keep these versions synchronized:

1. **`Cargo.toml`** - `version = "0.1.1"`
2. **`extension.toml`** - `version = "0.1.1"`
3. **Git tag** - `v0.1.1`
4. **GitHub release** - `v0.1.1`

### Automated Version Bump

Using `just`:

```bash
# This creates changelog, commits, and tags
just release 0.1.1

# Then push
git push origin main --tags
```

### Versioning Strategy

- **Pre-1.0.0:** APIs may change between minor versions
- **1.0.0+:** Follow strict semantic versioning
- **Breaking changes:** Always bump MAJOR version
- **Platform support changes:** Document in CHANGELOG

---

## Post-Release Checklist

### After Publishing to crates.io

- [ ] Verify installation works: `cargo install mobile-device-mcp-server`
- [ ] Test on fresh system (VM or container)
- [ ] Check crates.io page renders correctly
- [ ] Update documentation if needed

### After Publishing to Zed Extensions

- [ ] Install extension in Zed and test
- [ ] Verify installation instructions are accurate
- [ ] Test on macOS (iOS + Android)
- [ ] Test on Linux (Android only)
- [ ] Update repository README with Zed marketplace link

### General

- [ ] Announce release on GitHub Discussions
- [ ] Update project website/docs if applicable
- [ ] Monitor GitHub issues for bug reports
- [ ] Check CI/CD passed for all platforms

---

## Platform-Specific Notes

### macOS Releases

**Note:** Automated CI does not build macOS binaries (requires runners).

Users can:
1. Install from crates.io: `cargo install mobile-device-mcp-server`
2. Build from source with iOS support:
   ```bash
   cargo install mobile-device-mcp-server --features ios-support
   ```

### Linux Releases

- Built on `ubuntu-latest` (typically Ubuntu 22.04)
- Targets `x86_64-unknown-linux-gnu`
- Requires GLIBC version present in Ubuntu 22.04+

### Windows Releases

- Built on `windows-latest`
- Targets `x86_64-pc-windows-msvc`
- Requires Visual C++ Redistributable (usually pre-installed)

---

## Troubleshooting

### crates.io Upload Fails

**Error: "crate name already taken"**
- Your crate name is unique, but if this happens, choose a different name in `Cargo.toml`

**Error: "failed to verify package"**
- Run `cargo publish --dry-run` to see what's wrong
- Check that all dependencies are properly specified
- Ensure no dev-only features break the build

**Error: "version already uploaded"**
- You can't re-upload the same version
- Bump version number and try again

### WASM Build Fails

**Error: "can't find crate for 'std'"**
- Ensure `wasm32-wasip1` target is installed: `rustup target add wasm32-wasip1`

**Error: linking failed**
- Check that you're building the lib target only: `cargo build --target wasm32-wasip1 --lib`
- Native-only dependencies should be feature-gated

### Zed Extension Rejected

Common reasons:
- Extension size too large (optimize WASM build)
- Missing or incorrect metadata in `extension.toml`
- License compatibility issues
- Security concerns with dependencies

---

## Resources

### Documentation

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Zed Extensions Documentation](https://zed.dev/docs/extensions)
- [Semantic Versioning](https://semver.org/)

### Tools

- [cargo-release](https://github.com/crate-ci/cargo-release) - Automate releasing
- [git-cliff](https://git-cliff.org/) - Changelog generation
- [cargo-audit](https://github.com/rustsec/rustsec) - Security audits

### Community

- [Zed Discord](https://discord.gg/zed) - Extension development help
- [GitHub Discussions](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/discussions) - Project discussions

---

## Quick Reference

```bash
# Complete release workflow
VERSION="0.1.1"

# 1. Pre-release checks
just pre-commit

# 2. Update version and changelog
just release $VERSION

# 3. Publish to crates.io
cargo publish --features native-binary

# 4. Push tag (triggers GitHub release)
git push origin main --tags

# 5. Build and submit Zed extension
just build-wasm
# Then create PR to zed-industries/extensions

# 6. Verify everything
cargo install mobile-device-mcp-server --force
mobile-device-mcp-server --version
```

---

**Last Updated:** December 2024  
**Maintainer:** Sorin Albu-Irimies  
**Status:** Production Ready