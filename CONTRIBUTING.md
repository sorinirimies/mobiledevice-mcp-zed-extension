# Contributing to Mobile Device MCP Server

Thank you for your interest in contributing to the Mobile Device MCP Server! This document provides guidelines and instructions for contributing to the project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)
- [CI/CD Pipeline](#cicd-pipeline)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members
- Accept constructive criticism gracefully

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.70 or higher** - [Install Rust](https://rustup.rs/)
- **Just** - Command runner: `cargo install just`
- **Git** - Version control
- **Android Platform Tools** - For Android support
- **Xcode Command Line Tools** - For iOS support (macOS only)

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/sorinirimies/mobile-device-mcp.git
cd mobile-device-mcp

# Run automated development setup
just dev-setup

# This will:
# - Install required Rust targets (wasm32-wasip1)
# - Add rustfmt and clippy components
# - Set up git hooks for pre-commit checks
# - Verify dependencies
```

## Development Setup

### Manual Setup

If you prefer manual setup over `just dev-setup`:

```bash
# Add WASM target
rustup target add wasm32-wasip1

# Add required components
rustup component add rustfmt clippy

# Install development tools
cargo install cargo-audit      # Security auditing
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-watch      # Auto-rebuild on changes
```

### IDE Setup

**VS Code:**
```json
{
  "rust-analyzer.cargo.features": ["native-binary"],
  "rust-analyzer.check.command": "clippy",
  "editor.formatOnSave": true
}
```

**IntelliJ IDEA / CLion:**
- Install the Rust plugin
- Enable rustfmt on save
- Configure Clippy as default checker

## Development Workflow

### 1. Create a Branch

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Or a bugfix branch
git checkout -b fix/issue-number-description
```

### 2. Make Changes

```bash
# Build and test frequently
just build
just test

# Watch mode for continuous feedback
just watch

# Format code before committing
just fmt
```

### 3. Pre-Commit Checks

Before committing, run all checks:

```bash
# Run all pre-commit checks
just pre-commit

# This runs:
# - cargo fmt (formatting)
# - cargo clippy (linting)
# - cargo test (all tests)
```

The git hook (installed by `just dev-setup`) will automatically run these checks.

### 4. Commit Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/) format:

```bash
# Format: <type>(<scope>): <subject>

git commit -m "feat(android): add screen recording support"
git commit -m "fix(ios): resolve simulator screenshot issue"
git commit -m "docs: update installation instructions"
git commit -m "test: add unit tests for device detection"
git commit -m "chore: update dependencies"
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

## Testing

### Running Tests

```bash
# Run all unit tests
just test

# Run specific test suite
cargo test --test protocol_tests

# Run tests with output
cargo test -- --nocapture

# Run integration tests (requires connected device)
just test-integration

# Generate coverage report
just test-coverage
# Opens coverage/index.html
```

### Writing Tests

Place tests in appropriate locations:

- **Unit tests:** `src/` (inline with code or in `mod tests`)
- **Integration tests:** `tests/`
- **Tool tests:** `scripts/test-all-tools.sh`

Example unit test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_detection() {
        let devices = detect_devices();
        assert!(!devices.is_empty());
    }
}
```

### Test Requirements

All PRs must:
- Include tests for new functionality
- Maintain or improve code coverage
- Pass all existing tests
- Include integration tests where applicable

## Code Style

### Formatting

```bash
# Format all code
just fmt

# Check formatting without changing files
just fmt-check
```

We use the default `rustfmt` configuration with these preferences:
- 100 character line limit (where possible)
- 4 spaces for indentation
- Use trailing commas in multi-line expressions

### Linting

```bash
# Run clippy
just lint

# Auto-fix clippy suggestions
just lint-fix
```

All code must pass Clippy with no warnings. Common patterns:

```rust
// ✅ Good: Use Result for error handling
pub fn get_device(id: &str) -> Result<Device, Error> {
    // ...
}

// ✅ Good: Use explicit types for public APIs
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
}

// ✅ Good: Document public items
/// Gets the screen size of the specified device.
///
/// # Arguments
/// * `device_id` - The unique device identifier
///
/// # Returns
/// Returns `Ok(ScreenSize)` on success, or an error if the device is not found.
pub fn get_screen_size(device_id: &str) -> Result<ScreenSize, Error> {
    // ...
}
```

### Documentation

- Document all public items (functions, structs, modules)
- Use `///` for doc comments
- Include examples in doc comments where helpful
- Keep README and docs/ up to date

```bash
# Generate and view documentation
just doc

# Check documentation builds without warnings
just doc-check
```

## Submitting Changes

### Pull Request Process

1. **Update Documentation:**
   - Update README.md if needed
   - Add documentation to docs/ for significant features
   - Update CHANGELOG.md (or let git-cliff handle it)

2. **Run CI Checks Locally:**
   ```bash
   # Run the same checks as CI
   just ci
   ```

3. **Push Your Branch:**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create Pull Request:**
   - Go to GitHub and create a PR
   - Fill out the PR template
   - Link any related issues
   - Add appropriate labels

5. **Address Review Comments:**
   - Make requested changes
   - Push updates to the same branch
   - Re-run `just ci` after changes

### Pull Request Template

Your PR should include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass (if applicable)
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings introduced
- [ ] Tests added for new functionality
```

## CI/CD Pipeline

All pull requests and commits trigger automated CI/CD workflows:

### CI Workflow (`.github/workflows/ci.yml`)

Runs on every push and PR:

✅ **Test Suite** - Runs on Ubuntu, macOS, Windows with stable and beta Rust
✅ **Code Checks** - Formatting, Clippy, documentation
✅ **Build** - Multiple targets (native binaries + WASM)
✅ **Security Audit** - Checks for known vulnerabilities
✅ **Code Coverage** - Uploads to Codecov
✅ **MSRV Check** - Ensures Rust 1.70+ compatibility

### Release Workflow (`.github/workflows/release.yml`)

Triggered on version tags (e.g., `v0.2.0`):

📦 **Build Release Binaries** - All platforms (Linux, macOS, Windows, WASM)
📦 **Create GitHub Release** - With auto-generated changelog
📦 **Publish to crates.io** - For stable releases
📦 **Build Zed Extension** - Packaged extension bundle

### Integration Tests (`.github/workflows/integration.yml`)

Runs daily and on-demand:

🧪 **Android Emulator Tests** - Multiple API levels
🧪 **iOS Simulator Tests** - Multiple device types (macOS)
🧪 **Cross-Platform Tests** - Compatibility checks
🧪 **WASM Validation** - Extension functionality

### Local CI Simulation

```bash
# Run all CI checks locally before pushing
just ci

# Individual checks
just fmt-check    # Formatting
just lint         # Clippy
just test         # All tests
just doc-check    # Documentation
just audit        # Security audit
```

### CI Troubleshooting

**Formatting failures:**
```bash
just fmt
git add -u
git commit --amend --no-edit
```

**Clippy warnings:**
```bash
just lint-fix
# Review changes, test, and commit
```

**Test failures:**
```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **Major (x.0.0):** Breaking changes
- **Minor (0.x.0):** New features, backward compatible
- **Patch (0.0.x):** Bug fixes, backward compatible

### Creating a Release

Maintainers can create releases using Just:

```bash
# Create and tag a new release
just release 0.2.0

# This will:
# 1. Generate CHANGELOG.md for the version
# 2. Commit the changelog
# 3. Create an annotated git tag
# 4. Provide push instructions

# Push the release
git push origin main --tags
```

### Automated Release Steps

Once the tag is pushed, GitHub Actions automatically:

1. Builds binaries for all platforms
2. Generates release notes from commits
3. Creates a GitHub release
4. Uploads release artifacts
5. Publishes to crates.io (for stable releases)
6. Updates documentation

### Manual Release Checklist

For manual releases:

- [ ] Update version in `Cargo.toml`
- [ ] Run `just changelog-tag v0.2.0`
- [ ] Update `CHANGELOG.md` if needed
- [ ] Commit changes: `git commit -m "chore: release v0.2.0"`
- [ ] Create tag: `git tag -a v0.2.0 -m "Release v0.2.0"`
- [ ] Push: `git push origin main --tags`
- [ ] Verify GitHub Actions workflows
- [ ] Test published artifacts

## Areas for Contribution

We welcome contributions in these areas:

### High Priority

- **iOS Physical Device Support:** Implement WebDriverAgent integration
- **Performance:** Optimize XML parsing and device communication
- **Testing:** Add more integration and unit tests
- **Documentation:** Improve guides and examples

### Feature Requests

- **Screen Recording:** Video capture for Android/iOS
- **Element Matching:** Visual element detection with ML
- **Accessibility Testing:** Automated accessibility checks
- **Multi-Device:** Control multiple devices simultaneously
- **Remote Devices:** Support for network-connected devices

### Platform Support

- **Windows:** Improve Windows compatibility
- **Linux:** Test and document Linux-specific scenarios
- **Docker:** Create containerized testing environment

## Getting Help

- **GitHub Issues:** Report bugs or request features
- **GitHub Discussions:** Ask questions and share ideas
- **Documentation:** Check docs/ for detailed guides
- **Source Code:** Comments and doc comments explain implementation

## Recognition

Contributors will be:
- Listed in release notes
- Credited in the Contributors section
- Mentioned in significant feature announcements

Thank you for contributing! 🎉

---

**Questions?** Open an issue or start a discussion on GitHub.