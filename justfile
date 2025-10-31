# Mobile Device MCP Server - Justfile
# Common development tasks and commands

# Default recipe (show available commands)
default:
    @just --list

# Setup development environment
setup:
    @echo "Setting up development environment..."
    rustup target add wasm32-wasip1
    rustup component add rustfmt clippy
    @echo "✅ Development environment ready"

# Install dependencies (check for required tools)
check-deps:
    @echo "Checking dependencies..."
    @command -v cargo >/dev/null 2>&1 || { echo "❌ cargo not found. Install Rust from https://rustup.rs/"; exit 1; }
    @command -v adb >/dev/null 2>&1 || { echo "⚠️  adb not found. Android support will be limited."; }
    @if [ "$(uname)" = "Darwin" ]; then \
        command -v xcrun >/dev/null 2>&1 || { echo "⚠️  xcrun not found. iOS support will be limited."; }; \
    fi
    @echo "✅ Dependency check complete"

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt --all
    @echo "✅ Code formatted"

# Check formatting
fmt-check:
    @echo "Checking code formatting..."
    cargo fmt --all -- --check

# Run clippy linter
lint:
    @echo "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with auto-fix
lint-fix:
    @echo "Running clippy with fixes..."
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Run all tests
test:
    @echo "Running tests..."
    cargo test --verbose --all-features

# Run tests with coverage
test-coverage:
    @echo "Running tests with coverage..."
    cargo install cargo-tarpaulin --locked
    cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html --output-dir coverage

# Run tests without default features
test-no-default:
    @echo "Running tests (no default features)..."
    cargo test --verbose --no-default-features

# Run tests with native-binary feature
test-native:
    @echo "Running tests (native-binary)..."
    cargo test --verbose --features native-binary

# Build native binary (debug)
build:
    @echo "Building native binary (debug)..."
    cargo build --features native-binary

# Build native binary (release)
build-release:
    @echo "Building native binary (release)..."
    cargo build --release --features native-binary

# Build with iOS support (macOS only)
build-ios:
    @echo "Building with iOS support..."
    cargo build --release --features "native-binary,ios-support"

# Build WASM extension for Zed
build-wasm:
    @echo "Building WASM extension..."
    rustup target add wasm32-wasip1
    cargo build --release --target wasm32-wasip1

# Build all targets
build-all: build-release build-wasm
    @echo "✅ All targets built"

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    @echo "✅ Clean complete"

# Run the native server
run:
    @echo "Starting MCP server..."
    cargo run --features native-binary

# Run with debug logging
run-debug:
    @echo "Starting MCP server (debug mode)..."
    MOBILE_DEVICE_MCP_DEBUG=1 cargo run --features native-binary

# Generate documentation
doc:
    @echo "Generating documentation..."
    cargo doc --no-deps --all-features --open

# Check documentation
doc-check:
    @echo "Checking documentation..."
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Run security audit
audit:
    @echo "Running security audit..."
    cargo install cargo-audit
    cargo audit

# Check for outdated dependencies
outdated:
    @echo "Checking for outdated dependencies..."
    cargo install cargo-outdated
    cargo outdated

# Update dependencies
update:
    @echo "Updating dependencies..."
    cargo update
    @echo "✅ Dependencies updated"

# Install the binary locally
install:
    @echo "Installing mobile-device-mcp-server..."
    cargo install --path . --features native-binary --force
    @echo "✅ Installed to ~/.cargo/bin/"

# Install WASM extension to Zed
install-zed:
    @echo "Installing Zed extension..."
    mkdir -p ~/.config/zed/extensions/mobile-device-mcp
    cargo build --release --target wasm32-wasip1
    cp target/wasm32-wasip1/release/mobile_device_mcp.wasm ~/.config/zed/extensions/mobile-device-mcp/
    cp extension.toml ~/.config/zed/extensions/mobile-device-mcp/
    @echo "✅ Installed to ~/.config/zed/extensions/mobile-device-mcp/"

# Uninstall Zed extension
uninstall-zed:
    @echo "Uninstalling Zed extension..."
    rm -rf ~/.config/zed/extensions/mobile-device-mcp
    @echo "✅ Zed extension uninstalled"

# Run integration tests (requires connected device)
test-integration:
    @echo "Running integration tests..."
    @echo "⚠️  Ensure a device/emulator is connected!"
    ./scripts/test-all-tools.sh

# Quick smoke test
test-smoke:
    @echo "Running smoke tests..."
    ./scripts/test-tools.sh

# List connected Android devices
list-android:
    @echo "Connected Android devices:"
    @adb devices

# List available iOS simulators (macOS only)
list-ios:
    @echo "Available iOS simulators:"
    @if [ "$(uname)" = "Darwin" ]; then \
        xcrun simctl list devices | grep -E "iPhone|iPad"; \
    else \
        echo "❌ iOS simulators only available on macOS"; \
    fi

# Start Android emulator
start-android DEVICE="Pixel_6_API_34":
    @echo "Starting Android emulator: {{DEVICE}}..."
    emulator -avd {{DEVICE}} &

# Boot iOS simulator (macOS only)
boot-ios DEVICE="iPhone 15 Pro":
    @echo "Booting iOS simulator: {{DEVICE}}..."
    @if [ "$(uname)" = "Darwin" ]; then \
        xcrun simctl boot "{{DEVICE}}" || echo "Simulator already booted or not found"; \
    else \
        echo "❌ iOS simulators only available on macOS"; \
    fi

# Kill all running emulators/simulators
kill-emulators:
    @echo "Killing emulators and simulators..."
    @pkill -f emulator || true
    @if [ "$(uname)" = "Darwin" ]; then \
        killall Simulator 2>/dev/null || true; \
    fi
    @echo "✅ Emulators killed"

# Generate changelog
changelog:
    @echo "Generating changelog..."
    git-cliff --output CHANGELOG.md
    @echo "✅ Changelog generated"

# Generate changelog for specific tag
changelog-tag TAG:
    @echo "Generating changelog for {{TAG}}..."
    git-cliff --tag {{TAG}} --output CHANGELOG.md

# Create a new release tag
release VERSION:
    @echo "Creating release {{VERSION}}..."
    @if git rev-parse "v{{VERSION}}" >/dev/null 2>&1; then \
        echo "❌ Tag v{{VERSION}} already exists"; \
        exit 1; \
    fi
    git-cliff --tag v{{VERSION}} --output CHANGELOG.md
    git add CHANGELOG.md
    git commit -m "chore: release v{{VERSION}}" || true
    git tag -a "v{{VERSION}}" -m "Release v{{VERSION}}"
    @echo "✅ Release v{{VERSION}} created"
    @echo "Push with: git push origin main --tags"

# Pre-commit checks (run before committing)
pre-commit: fmt lint test
    @echo "✅ Pre-commit checks passed"

# CI checks (mimics GitHub Actions)
ci: fmt-check lint test-no-default test-native doc-check audit
    @echo "✅ CI checks passed"

# Benchmark performance (if benchmarks exist)
bench:
    @echo "Running benchmarks..."
    cargo bench

# Show project statistics
stats:
    @echo "Project Statistics:"
    @echo "==================="
    @echo "Lines of Rust code:"
    @find src -name "*.rs" -type f -exec wc -l {} + | tail -n 1
    @echo ""
    @echo "Lines of test code:"
    @find tests -name "*.rs" -type f -exec wc -l {} + 2>/dev/null | tail -n 1 || echo "0"
    @echo ""
    @echo "Lines of documentation:"
    @find docs -name "*.md" -type f -exec wc -l {} + 2>/dev/null | tail -n 1 || echo "0"
    @echo ""
    @echo "Dependencies:"
    @cargo tree --depth 1 | wc -l

# Watch for changes and run tests
watch:
    @echo "Watching for changes..."
    cargo install cargo-watch
    cargo watch -x "test --all-features"

# Watch and run the server
watch-run:
    @echo "Watching and running server..."
    cargo install cargo-watch
    cargo watch -x "run --features native-binary"

# Create release build for all platforms
release-all:
    @echo "Building release for all platforms..."
    @echo "Building Linux x86_64..."
    cargo build --release --target x86_64-unknown-linux-gnu --features native-binary
    @echo "Building macOS x86_64..."
    cargo build --release --target x86_64-apple-darwin --features "native-binary,ios-support"
    @echo "Building macOS ARM64..."
    cargo build --release --target aarch64-apple-darwin --features "native-binary,ios-support"
    @echo "Building Windows x86_64..."
    cargo build --release --target x86_64-pc-windows-msvc --features native-binary
    @echo "Building WASM..."
    cargo build --release --target wasm32-wasip1
    @echo "✅ All release builds complete"

# Package release artifacts
package VERSION:
    @echo "Packaging release v{{VERSION}}..."
    mkdir -p dist/v{{VERSION}}
    @# Linux
    tar czf dist/v{{VERSION}}/mobile-device-mcp-server-linux-x86_64.tar.gz \
        -C target/x86_64-unknown-linux-gnu/release mobile-device-mcp-server
    @# macOS Intel
    tar czf dist/v{{VERSION}}/mobile-device-mcp-server-macos-x86_64.tar.gz \
        -C target/x86_64-apple-darwin/release mobile-device-mcp-server
    @# macOS ARM
    tar czf dist/v{{VERSION}}/mobile-device-mcp-server-macos-aarch64.tar.gz \
        -C target/aarch64-apple-darwin/release mobile-device-mcp-server
    @# Windows
    cd target/x86_64-pc-windows-msvc/release && \
        zip -r ../../../dist/v{{VERSION}}/mobile-device-mcp-server-windows-x86_64.zip mobile-device-mcp-server.exe
    @# WASM Extension
    tar czf dist/v{{VERSION}}/mobile-device-mcp-zed-extension.tar.gz \
        -C target/wasm32-wasip1/release mobile_device_mcp.wasm \
        extension.toml README.md LICENSE
    @echo "✅ Release packages created in dist/v{{VERSION}}/"

# Check minimum supported Rust version
check-msrv:
    @echo "Checking MSRV (1.70)..."
    cargo +1.70 check --all-features

# Initialize git hooks
init-hooks:
    @echo "Installing git hooks..."
    echo "#!/bin/sh\njust pre-commit" > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    @echo "✅ Git hooks installed"

# Full development setup
dev-setup: setup check-deps init-hooks
    @echo "✅ Development setup complete"
    @echo ""
    @echo "Quick commands:"
    @echo "  just build       - Build debug binary"
    @echo "  just test        - Run tests"
    @echo "  just run         - Run the server"
    @echo "  just install-zed - Install Zed extension"
    @echo ""
    @echo "Run 'just' to see all available commands"
