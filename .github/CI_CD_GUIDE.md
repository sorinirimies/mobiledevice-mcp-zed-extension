# CI/CD Quick Reference Guide

This guide provides a quick reference for the simplified CI/CD workflows used in the Mobile Device MCP Server project.

## 🚀 Quick Start

```bash
# Install Just command runner
cargo install just

# Setup development environment
just dev-setup

# Run local checks
just pre-commit
```

## 📊 CI/CD Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:** Push to main, Pull Requests

**Jobs:**

| Job | Description | Platform | Status |
|-----|-------------|----------|--------|
| `test` | Run test suite | Ubuntu | ✅ Required |
| `lint` | Format & clippy checks | Ubuntu | ✅ Required |

**Local Simulation:**
```bash
just test
just lint
just fmt
```

### 2. Release Workflow (`.github/workflows/release.yml`)

**Triggers:** Tags matching `v*.*.*`

**Jobs:**

| Job | Description | Outputs |
|-----|-------------|---------|
| `create-release` | Create GitHub release | Release URL |
| `build-release` | Build binaries | Platform-specific archives |

**Artifacts Created:**
- `mobile-device-mcp-linux-x86_64.tar.gz`
- `mobile-device-mcp-macos-aarch64.tar.gz`
- `mobile-device-mcp-windows-x86_64.zip`

**Create Release:**
```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags
```

## 🛠️ Just Commands Reference

### Development

```bash
just setup              # Install Rust targets and components
just dev-setup          # Complete development setup + git hooks
just check-deps         # Verify required dependencies
```

### Building

```bash
just build              # Build debug binary
just build-release      # Build release binary
just build-ios          # Build with iOS support (macOS)
just build-wasm         # Build WASM extension
just build-all          # Build all targets
```

### Testing

```bash
just test               # Run all unit tests
just test-no-default    # Test without default features
just test-native        # Test with native-binary feature
just test-coverage      # Generate coverage report
just test-integration   # Run integration tests (needs device)
just test-smoke         # Quick smoke test
```

### Code Quality

```bash
just fmt                # Format code
just fmt-check          # Check formatting (CI mode)
just lint               # Run clippy
just lint-fix           # Auto-fix clippy warnings
just doc                # Generate and open docs
just doc-check          # Check docs build (CI mode)
```

### CI/CD

```bash
just pre-commit         # Run all pre-commit checks
just ci                 # Run all CI checks locally
just audit              # Security audit
just outdated           # Check outdated dependencies
```

### Running

```bash
just run                # Run MCP server
just run-debug          # Run with debug logging
just watch              # Watch and auto-test
just watch-run          # Watch and auto-run
```

### Installation

```bash
just install            # Install binary to ~/.cargo/bin
just install-zed        # Install Zed extension
just uninstall-zed      # Uninstall Zed extension
```

### Device Management

```bash
just list-android       # List Android devices
just list-ios           # List iOS simulators (macOS)
just start-android      # Start Android emulator
just boot-ios           # Boot iOS simulator (macOS)
just kill-emulators     # Kill all emulators
```

### Release Management

```bash
just changelog          # Generate full changelog
just changelog-tag TAG  # Generate changelog for tag
just release VERSION    # Create release tag
just release-all        # Build all release targets
just package VERSION    # Package release artifacts
```

### Utilities

```bash
just clean              # Clean build artifacts
just update             # Update dependencies
just stats              # Show project statistics
just init-hooks         # Install git hooks
```

## 🔄 Common Workflows

### Pre-Commit Workflow

```bash
# Make changes
vim src/main.rs

# Format and check
just fmt
just lint

# Test
just test

# All-in-one pre-commit check
just pre-commit

# Commit
git add .
git commit -m "feat: add new feature"
```

### Pull Request Workflow

```bash
# Create branch
git checkout -b feature/my-feature

# Make changes and test
just build
just test

# Run full CI checks
just ci

# Push and create PR
git push origin feature/my-feature
```

### Release Workflow

```bash
# Update version in Cargo.toml
vim Cargo.toml

# Create release
just release 0.2.0

# Push (triggers release workflow)
git push origin main --tags

# Monitor GitHub Actions
# Artifacts will be automatically built and published
```

### Debugging CI Failures

```bash
# Formatting issues
just fmt
git add -u
git commit --amend --no-edit

# Clippy warnings
just lint-fix
# Review, test, commit

# Test failures
cargo test -- --nocapture
cargo test failing_test_name -- --nocapture

# Documentation issues
just doc-check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## 📋 CI Requirements

### For All PRs

- ✅ All tests pass
- ✅ Code formatted with rustfmt
- ✅ No clippy warnings
- ✅ Documentation builds without warnings
- ✅ Security audit passes (no high/critical vulnerabilities)
- ✅ Minimum Rust version (1.70+) supported

### For Releases

- ✅ All CI checks pass
- ✅ Version updated in Cargo.toml
- ✅ Changelog updated
- ✅ Tag follows semver (v0.2.0)
- ✅ Release notes reviewed

## 🔐 Required Secrets

Configure these in GitHub repository settings:

| Secret | Purpose | Required For |
|--------|---------|--------------|
| `GITHUB_TOKEN` | Automatic (provided by GitHub) | All workflows |
| `CARGO_REGISTRY_TOKEN` | Publish to crates.io | Release workflow |
| `CODECOV_TOKEN` | Upload coverage | CI workflow (optional) |

## 📊 Status Badges

Add to README.md:

```markdown
[![CI](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/actions/workflows/ci.yml)
[![Release](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/sorinirimies/mobiledevice-mcp-zed-extension/branch/main/graph/badge.svg)](https://codecov.io/gh/sorinirimies/mobiledevice-mcp-zed-extension)
```

## 🆘 Troubleshooting

### CI is failing but local checks pass

```bash
# Ensure you're running the same checks
just ci

# Check Rust version matches CI
rustc --version  # Should be 1.70+

# Check for platform-specific issues
# CI runs on Ubuntu, macOS, and Windows
```

### Release workflow not triggering

```bash
# Verify tag format
git tag -l "v*"

# Tag must match: v[0-9]+.[0-9]+.[0-9]+
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Coverage upload failing

```bash
# Ensure CODECOV_TOKEN is set
# Generate coverage locally to debug
just test-coverage
```

### Dependency updates failing

```bash
# Check for breaking changes
cargo update --dry-run

# Update and test
cargo update
just test
```

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Just Documentation](https://just.systems/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Rust CI Best Practices](https://docs.rust-lang.org/cargo/guide/continuous-integration.html)

---

**Last Updated:** 2024-11-01  
**Maintained By:** Mobile Device MCP Team