#!/usr/bin/env bash

set -euo pipefail

# Mobile Device MCP Server - Manual Installation Script
# Bypasses Zed's dev extension compilation by manually copying files

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

# Check prerequisites
check_prerequisites() {
    log_step "Checking prerequisites..."

    # Check if extension.wasm exists
    if [ ! -f "extension.wasm" ]; then
        log_error "extension.wasm not found!"
        echo ""
        echo "Please build the extension first:"
        echo "  cargo build --target wasm32-wasip1 --release"
        echo "  cp target/wasm32-wasip1/release/mobile_device_mcp_server.wasm extension.wasm"
        echo ""
        echo "Or run the automated build script:"
        echo "  ./build-extension.sh"
        exit 1
    fi
    log_success "extension.wasm found"

    # Check if extension.toml exists
    if [ ! -f "extension.toml" ]; then
        log_error "extension.toml not found!"
        exit 1
    fi
    log_success "extension.toml found"

    # Check if native binary exists
    if [ ! -f "target/release/mobile-device-mcp-server" ]; then
        log_warning "Native binary not found. Building now..."
        if ! cargo build --release --features "native-binary,ios-support"; then
            log_error "Failed to build native binary!"
            exit 1
        fi
    fi
    log_success "Native binary available"

    # Verify WASM file
    local wasm_size=$(ls -lh extension.wasm | awk '{print $5}')
    log_success "WASM extension ready (${wasm_size})"
}

# Create manual installation
install_manually() {
    log_step "Installing extension manually..."

    local zed_ext_dir=$(get_zed_extensions_dir)
    local ext_install_path="${zed_ext_dir}/mcp-server-mobile-device"

    # Create extensions directory if it doesn't exist
    if [ ! -d "$zed_ext_dir" ]; then
        log_info "Creating Zed extensions directory..."
        mkdir -p "$zed_ext_dir"
    fi

    # Remove existing installation
    if [ -d "$ext_install_path" ]; then
        log_warning "Existing installation found. Removing..."
        rm -rf "$ext_install_path"
    fi

    # Create extension directory
    log_info "Creating extension directory..."
    mkdir -p "$ext_install_path"

    # Copy essential files
    log_info "Copying extension files..."

    # Copy WASM extension
    cp extension.wasm "$ext_install_path/"
    log_success "Copied extension.wasm"

    # Copy extension manifest
    cp extension.toml "$ext_install_path/"
    log_success "Copied extension.toml"

    # Copy native binary
    mkdir -p "$ext_install_path/target/release"
    cp target/release/mobile-device-mcp-server "$ext_install_path/target/release/"
    log_success "Copied native binary"

    # Copy documentation (optional)
    if [ -f "README.md" ]; then
        cp README.md "$ext_install_path/"
    fi

    # Verify installation
    if [ -f "$ext_install_path/extension.wasm" ] && \
       [ -f "$ext_install_path/extension.toml" ] && \
       [ -f "$ext_install_path/target/release/mobile-device-mcp-server" ]; then
        log_success "Manual installation completed successfully!"
        echo ""
        echo "Extension installed to: $ext_install_path"
    else
        log_error "Installation verification failed!"
        exit 1
    fi
}

# Test the installation
test_installation() {
    log_step "Testing installation..."

    local zed_ext_dir=$(get_zed_extensions_dir)
    local ext_install_path="${zed_ext_dir}/mcp-server-mobile-device"

    # Test native binary
    local binary_path="$ext_install_path/target/release/mobile-device-mcp-server"
    if [ -x "$binary_path" ]; then
        log_info "Testing native binary..."
        local test_result=$(echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | "$binary_path" 2>/dev/null || echo "test_failed")

        if [[ "$test_result" == *"mobile-device-mcp-server"* ]]; then
            log_success "Native binary is working"
        else
            log_warning "Native binary test inconclusive"
        fi
    else
        log_error "Native binary is not executable"
    fi

    # Check file permissions
    chmod +x "$binary_path" 2>/dev/null || true
    log_success "Set executable permissions"
}

# Show instructions
show_instructions() {
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  Manual Installation Complete! 🎉${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "The extension has been manually installed to avoid compilation issues."
    echo ""
    echo "Next steps:"
    echo ""
    echo "1. Restart Zed completely (important for manual installations)"
    echo ""
    echo "2. Configure the extension in your Zed settings:"
    echo "   Open: ~/.config/zed/settings.json"
    echo "   Add:"
    echo '   {'
    echo '     "context_servers": {'
    echo '       "mcp-server-mobile-device": {'
    echo '         "settings": {'
    echo '           "platform": "auto",'
    echo '           "debug": false'
    echo '         }'
    echo '       }'
    echo '     }'
    echo '   }'
    echo ""
    echo "3. Test the extension:"
    echo "   • Open Zed's Assistant panel"
    echo "   • Ask: 'List all my connected mobile devices'"
    echo ""
    echo "4. Troubleshooting:"
    echo "   • Check logs: tail -f ~/Library/Logs/Zed/Zed.log | grep -i mobile"
    echo "   • Verify devices: adb devices (for Android)"
    echo "   • Test binary directly: ./target/release/mobile-device-mcp-server"
    echo ""
    echo -e "${BLUE}Installation path:${NC} $(get_zed_extensions_dir)/mcp-server-mobile-device"
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Main installation flow
main() {
    echo ""
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║   Mobile Device MCP Server - Manual Installation        ║${NC}"
    echo -e "${CYAN}║   (Bypasses Zed's compilation process)                  ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""

    check_prerequisites
    install_manually
    test_installation
    show_instructions
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

        if [ -d "$ext_install_path" ]; then
            rm -rf "$ext_install_path"
            log_success "Extension uninstalled from: $ext_install_path"
            echo ""
            echo "Remember to restart Zed."
        else
            log_warning "Extension not found at: $ext_install_path"
        fi
        ;;
    "reinstall")
        log_step "Reinstalling extension..."
        "$0" uninstall
        echo ""
        "$0" install
        ;;
    "help"|"--help"|"-h")
        echo "Mobile Device MCP Server - Manual Installation Script"
        echo ""
        echo "This script bypasses Zed's dev extension compilation by manually"
        echo "copying the pre-built WASM extension and native binary."
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  install     Install extension manually (default)"
        echo "  uninstall   Remove extension from Zed"
        echo "  reinstall   Remove and reinstall the extension"
        echo "  help        Show this help message"
        echo ""
        echo "Prerequisites:"
        echo "  • extension.wasm must exist (run ./build-extension.sh first)"
        echo "  • Native binary will be built automatically if missing"
        echo ""
        echo "Why manual installation?"
        echo "  • Avoids Zed's compilation of native dependencies"
        echo "  • Uses pre-built WASM and native binaries"
        echo "  • More reliable for extensions with complex dependencies"
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac
