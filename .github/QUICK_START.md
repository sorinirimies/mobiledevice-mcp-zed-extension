# CI/CD Quick Start Guide

Get up and running with the Mobile Device MCP Server CI/CD infrastructure in minutes.

## 🚀 For New Contributors

### 1. Initial Setup (5 minutes)

```bash
# Clone and enter repository
git clone https://github.com/sorinirimies/mobiledevice-mcp-zed-extension.git
cd mobile-device-mcp

# Install Just command runner
cargo install just

# Run automated setup
just dev-setup
```

This installs:
- ✅ Rust targets (wasm32-wasip1)
- ✅ Development tools (rustfmt, clippy)
- ✅ Git pre-commit hooks
- ✅ Dependency verification

### 2. Verify Setup

```bash
# Check everything works
just build
just test

# Should see: "✅ All tests passed"
```

### 3. Make Your First Change

```bash
# Create a feature branch
git checkout -b feature/my-contribution

# Make changes
vim src/main.rs

# Test your changes
just test
```

### 4. Pre-Commit Checks

```bash
# Run all quality checks (same as CI)
just ci

# This runs:
# - Format check
# - Clippy linting
# - All tests
# - Documentation check
# - Security audit
```

### 5. Submit Pull Request

```bash
# Commit with conventional commits format
git commit -m "feat: add awesome feature"

# Push to your fork
git push origin feature/my-contribution

# Create PR on GitHub
# ✅ CI will automatically run all checks
```

## 🔄 Daily Development Workflow

### Quick Commands

```bash
# Build and test cycle
just build      # Build debug binary
just test       # Run tests
just run        # Run the server

# Code quality
just fmt        # Format code
just lint       # Check with clippy

# Watch mode (auto-rebuild)
just watch      # Auto-run tests on changes
just watch-run  # Auto-restart server on changes
```

### Before Every Commit

```bash
# One command to rule them all
just pre-commit

# Automatically runs:
# 1. cargo fmt
# 2. cargo clippy
# 3. cargo test
```

## 📦 For Maintainers

### Creating a Release

```bash
# 1. Update version in Cargo.toml
vim Cargo.toml

# 2. Create release (generates changelog + tag)
just release 0.2.0

# 3. Push to trigger automated release
git push origin main --tags

# ✅ GitHub Actions automatically:
# - Builds binaries for all platforms
# - Creates GitHub release
# - Publishes to crates.io
# - Uploads artifacts
```

### Manual Testing Before Release

```bash
# Build all targets
just build-all

# Run integration tests
just test-integration

# Check for outdated dependencies
just outdated

# Security audit
just audit
```

## 🧪 Testing with Devices

### Android Testing

```bash
# List connected devices
just list-android

# Start emulator
just start-android Pixel_6_API_34

# Run integration tests
just test-integration
```

### iOS Testing (macOS only)

```bash
# List simulators
just list-ios

# Boot simulator
just boot-ios "iPhone 15 Pro"

# Run integration tests
just test-integration
```

## 🎯 Common Scenarios

### Scenario 1: PR Check Failed - Formatting

```bash
# Fix formatting
just fmt

# Commit fix
git add -u
git commit --amend --no-edit
git push --force
```

### Scenario 2: PR Check Failed - Clippy Warnings

```bash
# Auto-fix warnings
just lint-fix

# Review changes
git diff

# Commit
git add -u
git commit -m "fix: address clippy warnings"
```

### Scenario 3: PR Check Failed - Tests

```bash
# Run tests with output
cargo test -- --nocapture

# Run specific failing test
cargo test test_name -- --nocapture

# Fix and verify
just test
```

### Scenario 4: Adding a New Feature

```bash
# 1. Create branch
git checkout -b feature/new-tool

# 2. Implement feature
vim src/tools/handlers.rs

# 3. Add tests
vim tests/new_tool_tests.rs

# 4. Update docs
vim README.md
vim docs/FEATURE_GUIDE.md

# 5. Verify everything
just ci

# 6. Commit and push
git commit -m "feat(tools): add new automation tool"
git push origin feature/new-tool
```

### Scenario 5: Fixing a Bug

```bash
# 1. Create branch from issue number
git checkout -b fix/123-device-detection

# 2. Add failing test first
vim tests/device_tests.rs
just test  # Should fail

# 3. Fix the bug
vim src/devices/android.rs

# 4. Verify fix
just test  # Should pass

# 5. Run all checks
just ci

# 6. Commit with issue reference
git commit -m "fix(android): resolve device detection issue

Fixes #123"
```

## 📊 Understanding CI Checks

### What Runs on Every PR

| Check | Duration | Purpose |
|-------|----------|---------|
| **test (Ubuntu)** | ~3 min | Tests on Linux |
| **test (macOS)** | ~3 min | Tests on macOS (iOS) |
| **test (Windows)** | ~3 min | Tests on Windows |
| **check** | ~2 min | Format, lint, docs |
| **build** | ~5 min | Multi-platform builds |
| **security** | ~1 min | Vulnerability scan |
| **coverage** | ~3 min | Code coverage |

**Total:** ~10-15 minutes for full CI run

### What Runs on Release

| Step | Duration | Output |
|------|----------|--------|
| **Create Release** | ~1 min | GitHub release |
| **Build Binaries** | ~15 min | 7 platform targets |
| **Publish Crate** | ~2 min | crates.io package |
| **Build Extension** | ~3 min | Zed WASM bundle |

**Total:** ~20 minutes for complete release

## 🔍 Troubleshooting

### "Just command not found"

```bash
cargo install just
# Add ~/.cargo/bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### "WASM target not found"

```bash
rustup target add wasm32-wasip1
```

### "Git hook fails"

```bash
# Re-install hooks
just init-hooks

# Or skip hook temporarily
git commit --no-verify
```

### "CI passes locally but fails on GitHub"

```bash
# Ensure you're on latest Rust
rustup update

# Clean build
just clean
just build

# Check specific platform
# (CI runs on Ubuntu, macOS, Windows)
```

### "Integration tests fail"

```bash
# Ensure device is connected
adb devices  # Android
xcrun simctl list devices | grep Booted  # iOS

# Check device is booted
just list-android
just list-ios

# Restart device
just kill-emulators
just start-android
```

## 📚 Learn More

- **Full CI/CD Guide:** `.github/CI_CD_GUIDE.md`
- **Contributing Guide:** `CONTRIBUTING.md`
- **CI/CD Setup:** `CI_CD_SETUP.md`
- **All Commands:** Run `just` to see all available commands

## 💡 Pro Tips

1. **Use watch mode** during development: `just watch`
2. **Run `just ci` before pushing** to catch issues early
3. **Use conventional commits** for automatic changelog generation
4. **Check `just stats`** to see project metrics
5. **Install git hooks** with `just init-hooks` for automatic checks

## 🆘 Getting Help

- **Command help:** `just --list` or `just`
- **CI/CD questions:** Check `.github/CI_CD_GUIDE.md`
- **Development issues:** See `CONTRIBUTING.md`
- **GitHub Issues:** Report problems or ask questions
- **GitHub Discussions:** Community Q&A

## ✅ Quick Reference

```bash
# Essential commands
just dev-setup          # First-time setup
just build             # Build project
just test              # Run tests
just pre-commit        # Pre-commit checks
just ci                # Full CI simulation
just run               # Run server

# Quality checks
just fmt               # Format code
just lint              # Run clippy
just audit             # Security audit

# Testing
just test-integration  # Integration tests
just test-coverage    # Coverage report

# Release (maintainers)
just release 0.2.0    # Create release
```

---

**Ready to contribute?** Start with `just dev-setup` and you're good to go! 🚀

**Questions?** Open an issue or check the comprehensive guides in `.github/` and root directory.