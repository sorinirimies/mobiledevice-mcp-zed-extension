#!/usr/bin/env bash

set -euo pipefail

# Mobile Device MCP Zed Extension Development Script
# This script helps with building, testing, and packaging the extension

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    log_info "Checking dependencies..."

    # Check Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi

    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js not found. Please install Node.js 18+: https://nodejs.org/"
        exit 1
    fi

    # Check npm
    if ! command -v npm &> /dev/null; then
        log_error "npm not found. Please install npm."
        exit 1
    fi

    log_success "All dependencies found"
}

# Build the extension
build() {
    log_info "Building the extension..."

    # Clean previous builds
    if [ -d "target" ]; then
        log_info "Cleaning previous build artifacts..."
        cargo clean
    fi

    # Build the extension
    log_info "Compiling Rust code..."
    cargo build --release

    if [ $? -eq 0 ]; then
        log_success "Extension built successfully"
    else
        log_error "Build failed"
        exit 1
    fi
}

# Run tests (if we had any)
test_extension() {
    log_info "Running tests..."
    cargo test

    if [ $? -eq 0 ]; then
        log_success "Tests passed"
    else
        log_error "Tests failed"
        exit 1
    fi
}

# Validate the extension configuration
validate() {
    log_info "Validating extension configuration..."

    # Check if required files exist
    required_files=(
        "Cargo.toml"
        "extension.toml"
        "src/lib.rs"
        "configuration/installation_instructions.md"
        "configuration/default_settings.jsonc"
    )

    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            log_error "Required file missing: $file"
            exit 1
        fi
    done

    # Validate Cargo.toml
    if ! grep -q "crate-type.*cdylib" Cargo.toml; then
        log_error "Cargo.toml must specify crate-type = ['cdylib']"
        exit 1
    fi

    # Validate extension.toml
    if ! grep -q "context_servers" extension.toml; then
        log_error "extension.toml must define context_servers section"
        exit 1
    fi

    log_success "Extension configuration is valid"
}

# Check code formatting
check_format() {
    log_info "Checking code formatting..."

    cargo fmt -- --check
    if [ $? -ne 0 ]; then
        log_warning "Code is not formatted. Run 'cargo fmt' to fix."
        return 1
    fi

    log_success "Code is properly formatted"
}

# Format code
format_code() {
    log_info "Formatting code..."
    cargo fmt
    log_success "Code formatted"
}

# Run clippy for linting
lint() {
    log_info "Running clippy lints..."
    cargo clippy -- -D warnings

    if [ $? -eq 0 ]; then
        log_success "No lint errors found"
    else
        log_error "Lint errors found"
        exit 1
    fi
}

# Package the extension for distribution
package() {
    log_info "Packaging extension..."

    # Create package directory
    PACKAGE_DIR="mobile-mcp-zed-extension-package"
    rm -rf "$PACKAGE_DIR"
    mkdir -p "$PACKAGE_DIR"

    # Copy necessary files
    cp Cargo.toml "$PACKAGE_DIR/"
    cp extension.toml "$PACKAGE_DIR/"
    cp README.md "$PACKAGE_DIR/"
    cp LICENSE "$PACKAGE_DIR/"
    cp EXAMPLES.md "$PACKAGE_DIR/"
    cp -r src "$PACKAGE_DIR/"
    cp -r configuration "$PACKAGE_DIR/"

    # Build the extension
    cd "$PACKAGE_DIR"
    cargo build --release
    cd ..

    # Create tarball
    tar -czf "mobile-mcp-zed-extension.tar.gz" "$PACKAGE_DIR"

    log_success "Package created: mobile-mcp-zed-extension.tar.gz"
}

# Install mobile-mcp for testing
install_mobile_mcp() {
    log_info "Installing mobile-mcp package for testing..."

    # Create a temporary package.json for testing
    cat > package.json << 'EOF'
{
  "name": "mobile-mcp-zed-extension-test",
  "version": "1.0.0",
  "dependencies": {
    "@mobilenext/mobile-mcp": "^0.0.32"
  }
}
EOF

    npm install

    if [ $? -eq 0 ]; then
        log_success "mobile-mcp package installed for testing"
    else
        log_error "Failed to install mobile-mcp package"
        exit 1
    fi
}

# Test mobile-mcp installation
test_mobile_mcp() {
    log_info "Testing mobile-mcp installation..."

    if [ -f "node_modules/@mobilenext/mobile-mcp/lib/index.js" ]; then
        log_success "mobile-mcp binary found"

        # Test if it runs
        node node_modules/@mobilenext/mobile-mcp/lib/index.js --help &> /dev/null
        if [ $? -eq 0 ]; then
            log_success "mobile-mcp binary is executable"
        else
            log_warning "mobile-mcp binary found but may not be working correctly"
        fi
    else
        log_error "mobile-mcp binary not found after installation"
        exit 1
    fi
}

# Clean up build artifacts
clean() {
    log_info "Cleaning up..."

    cargo clean
    rm -rf node_modules
    rm -f package.json
    rm -f package-lock.json
    rm -rf mobile-mcp-zed-extension-package
    rm -f mobile-mcp-zed-extension.tar.gz

    log_success "Clean up completed"
}

# Show help
show_help() {
    echo "Mobile Device MCP Zed Extension Development Script"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  build              Build the extension"
    echo "  test               Run tests"
    echo "  validate           Validate extension configuration"
    echo "  format             Format the code"
    echo "  check-format       Check if code is formatted"
    echo "  lint               Run clippy lints"
    echo "  package            Package extension for distribution"
    echo "  install-mobile-mcp Install mobile-mcp package for testing"
    echo "  test-mobile-mcp    Test mobile-mcp installation"
    echo "  clean              Clean up build artifacts"
    echo "  all                Run validate, format, lint, build, and test"
    echo "  help               Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 build           # Build the extension"
    echo "  $0 all             # Run full development cycle"
    echo "  $0 package         # Create distribution package"
}

# Run all development steps
run_all() {
    log_info "Running full development cycle..."

    check_dependencies
    validate
    format_code
    lint
    build
    test_extension

    log_success "All development steps completed successfully!"
}

# Main script logic
case "${1:-help}" in
    "build")
        check_dependencies
        build
        ;;
    "test")
        test_extension
        ;;
    "validate")
        validate
        ;;
    "format")
        format_code
        ;;
    "check-format")
        check_format
        ;;
    "lint")
        lint
        ;;
    "package")
        check_dependencies
        validate
        build
        package
        ;;
    "install-mobile-mcp")
        install_mobile_mcp
        ;;
    "test-mobile-mcp")
        test_mobile_mcp
        ;;
    "clean")
        clean
        ;;
    "all")
        run_all
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        log_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
