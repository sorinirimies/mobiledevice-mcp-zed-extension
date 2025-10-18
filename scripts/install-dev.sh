#!/usr/bin/env bash

set -euo pipefail

# Mobile Device MCP Zed Extension - Dev Mode Installation Script
# Automates the process of installing the extension in Zed's dev mode

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

log_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)
            echo "macos"
            ;;
        Linux*)
            echo "linux"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Get Zed extensions directory based on OS
get_zed_extensions_dir() {
    local os=$(detect_os)

    case "$os" in
        macos)
            echo "$HOME/.config/zed/extensions/installed"
            ;;
        linux)
            echo "$HOME/.config/zed/extensions/installed"
            ;;
        windows)
            echo "$APPDATA/Zed/extensions/installed"
            ;;
        *)
            log_error "Unsupported operating system"
            exit 1
            ;;
    esac
}

# Check if Rust is installed
check_rust() {
    log_step "Checking Rust installation..."

    if ! command -v cargo &> /dev/null; then
        log_error "Rust is not installed!"
        echo ""
        echo "Please install Rust from: https://rustup.rs/"
        echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    log_success "Rust is installed: $(cargo --version)"
}

# Check if wasm32-wasip1 target is installed
check_wasm_target() {
    log_step "Checking WASM target..."

    if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
        log_warning "WASM target not installed. Installing now..."
        rustup target add wasm32-wasip1
        log_success "WASM target installed"
    else
        log_success "WASM target already installed"
    fi
}

# Build the extension
build_extension() {
    log_step "Building extension..."

    # Clean previous build (optional)
    if [ -f "extension.wasm" ]; then
        log_info "Removing old extension.wasm..."
        rm -f extension.wasm
    fi

    # Build
    log_info "Compiling Rust code to WASM..."
    if cargo build --release --target wasm32-wasip1; then
        log_success "Build successful"
    else
        log_error "Build failed!"
        exit 1
    fi

    # Copy WASM file
    log_info "Copying WASM file..."
    cp target/wasm32-wasip1/release/mobile_device_mcp_server.wasm extension.wasm

    # Verify file exists and show size
    if [ -f "extension.wasm" ]; then
        local size=$(ls -lh extension.wasm | awk '{print $5}')
        log_success "extension.wasm created (${size})"
    else
        log_error "Failed to create extension.wasm"
        exit 1
    fi
}

# Create symlink to Zed extensions directory
install_to_zed() {
    log_step "Installing to Zed extensions directory..."

    local zed_ext_dir=$(get_zed_extensions_dir)
    local ext_install_path="${zed_ext_dir}/mcp-server-mobile-device"

    # Create extensions directory if it doesn't exist
    if [ ! -d "$zed_ext_dir" ]; then
        log_info "Creating Zed extensions directory..."
        mkdir -p "$zed_ext_dir"
    fi

    # Remove existing installation
    if [ -L "$ext_install_path" ] || [ -d "$ext_install_path" ]; then
        log_warning "Existing installation found. Removing..."
        rm -rf "$ext_install_path"
    fi

    # Create symlink
    log_info "Creating symlink..."
    ln -s "$SCRIPT_DIR" "$ext_install_path"

    if [ -L "$ext_install_path" ]; then
        log_success "Symlink created: $ext_install_path -> $SCRIPT_DIR"
    else
        log_error "Failed to create symlink"
        exit 1
    fi
}

# Verify installation
verify_installation() {
    log_step "Verifying installation..."

    local zed_ext_dir=$(get_zed_extensions_dir)
    local ext_install_path="${zed_ext_dir}/mcp-server-mobile-device"

    # Check symlink exists
    if [ ! -L "$ext_install_path" ]; then
        log_error "Symlink not found at $ext_install_path"
        return 1
    fi
    log_success "Symlink exists"

    # Check extension.wasm exists
    if [ ! -f "$SCRIPT_DIR/extension.wasm" ]; then
        log_error "extension.wasm not found in $SCRIPT_DIR"
        return 1
    fi
    log_success "extension.wasm exists"

    # Check extension.toml exists
    if [ ! -f "$SCRIPT_DIR/extension.toml" ]; then
        log_error "extension.toml not found"
        return 1
    fi
    log_success "extension.toml exists"

    log_success "Installation verified successfully!"
}

# Check platform tools (optional)
check_platform_tools() {
    log_step "Checking platform tools (optional)..."

    local has_android=false
    local has_ios=false

    # Check Android tools
    if command -v adb &> /dev/null; then
        log_success "Android tools (adb) found: $(adb version 2>&1 | head -n1)"
        has_android=true
    else
        log_warning "Android tools (adb) not found"
    fi

    # Check iOS tools (macOS only)
    if [ "$(detect_os)" = "macos" ]; then
        if command -v xcrun &> /dev/null && xcrun simctl help &> /dev/null; then
            log_success "iOS tools (xcrun) found"
            has_ios=true
        else
            log_warning "iOS tools (xcrun) not found"
        fi
    fi

    if [ "$has_android" = false ] && [ "$has_ios" = false ]; then
        echo ""
        log_warning "No mobile platform tools detected!"
        echo ""
        echo "The extension is installed, but you won't be able to use it without platform tools."
        echo ""
        echo "To control Android devices, install Android Platform Tools:"
        echo "  macOS:   brew install android-platform-tools"
        echo "  Linux:   sudo apt-get install android-tools-adb"
        echo "  Windows: Download from developer.android.com/studio/releases/platform-tools"
        echo ""
        if [ "$(detect_os)" = "macos" ]; then
            echo "To control iOS devices/simulators (macOS only):"
            echo "  xcode-select --install"
        fi
        echo ""
    fi
}

# Show next steps
show_next_steps() {
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  Installation Complete! 🎉${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "Next steps:"
    echo ""
    echo "1. Reload Zed extensions:"
    echo "   • Press Cmd+Shift+P (macOS) or Ctrl+Shift+P (Linux/Windows)"
    echo "   • Type: zed: reload extensions"
    echo "   • Press Enter"
    echo ""
    echo "   OR restart Zed completely"
    echo ""
    echo "2. Test the extension:"
    echo "   • Open Zed's Assistant panel (Cmd+? or Ctrl+?)"
    echo "   • Ask: 'List all my connected mobile devices'"
    echo ""
    echo "3. Configure (optional):"
    echo "   • Open Zed settings: ~/.config/zed/settings.json"
    echo "   • Add configuration as described in README.md"
    echo ""
    echo "4. View logs (for debugging):"
    local os=$(detect_os)
    if [ "$os" = "macos" ]; then
        echo "   tail -f ~/Library/Logs/Zed/Zed.log | grep -i mobile"
    elif [ "$os" = "linux" ]; then
        echo "   tail -f ~/.local/share/zed/logs/Zed.log | grep -i mobile"
    else
        echo "   Check %APPDATA%\\Zed\\logs\\Zed.log"
    fi
    echo ""
    echo "For more information, see:"
    echo "  • README.md - General usage"
    echo "  • DEV_MODE_INSTALLATION.md - Development details"
    echo "  • EXAMPLES.md - Usage examples"
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Main installation flow
main() {
    echo ""
    echo -e "${CYAN}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║   Mobile Device MCP Server - Dev Mode Installation    ║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""

    # Run installation steps
    check_rust
    check_wasm_target
    build_extension
    install_to_zed
    verify_installation
    check_platform_tools

    # Show next steps
    show_next_steps
}

# Handle script arguments
case "${1:-install}" in
    "install")
        main
        ;;
    "uninstall")
        log_step "Uninstalling extension..."
        local zed_ext_dir=$(get_zed_extensions_dir)
        local ext_install_path="${zed_ext_dir}/mcp-server-mobile-device"

        if [ -L "$ext_install_path" ] || [ -d "$ext_install_path" ]; then
            rm -rf "$ext_install_path"
            log_success "Extension uninstalled from: $ext_install_path"
            echo ""
            echo "Remember to reload Zed extensions or restart Zed."
        else
            log_warning "Extension not found at: $ext_install_path"
        fi
        ;;
    "rebuild")
        log_step "Rebuilding extension..."
        check_rust
        check_wasm_target
        build_extension
        echo ""
        log_success "Rebuild complete!"
        echo ""
        echo "Next steps:"
        echo "  1. In Zed, press Cmd+Shift+P (or Ctrl+Shift+P)"
        echo "  2. Type: zed: reload extensions"
        echo "  3. Press Enter"
        ;;
    "verify")
        verify_installation
        ;;
    "help"|"--help"|"-h")
        echo "Mobile Device MCP Server - Dev Mode Installation Script"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  install    Install extension in dev mode (default)"
        echo "  uninstall  Remove extension from Zed"
        echo "  rebuild    Rebuild the extension without reinstalling"
        echo "  verify     Verify installation is correct"
        echo "  help       Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0              # Install in dev mode"
        echo "  $0 install      # Same as above"
        echo "  $0 rebuild      # Rebuild after code changes"
        echo "  $0 uninstall    # Remove from Zed"
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac
