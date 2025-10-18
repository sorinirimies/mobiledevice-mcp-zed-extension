#!/bin/bash

# Script to rename all mobile_ tools to mobile_device_mcp_

set -e

echo "Renaming all mobile_* tools to mobile_device_mcp_*"
echo "=================================================="

# List of files to update
FILES=(
    "src/main.rs"
    "src/tools/definitions.rs"
    "src/tools/handlers.rs"
    "src/mcp/protocol.rs"
    "test-tools.sh"
)

# List of tool names to rename
TOOLS=(
    "mobile_list_available_devices"
    "mobile_get_screen_size"
    "mobile_get_orientation"
    "mobile_list_apps"
    "mobile_list_elements_on_screen"
    "mobile_take_screenshot"
    "mobile_save_screenshot"
    "mobile_click_on_screen_at_coordinates"
    "mobile_double_tap_on_screen"
    "mobile_long_press_on_screen_at_coordinates"
    "mobile_swipe_on_screen"
    "mobile_type_keys"
    "mobile_press_button"
    "mobile_launch_app"
    "mobile_terminate_app"
    "mobile_install_app"
    "mobile_uninstall_app"
    "mobile_open_url"
    "mobile_set_orientation"
)

# Create backup directory
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

echo "Creating backups in $BACKUP_DIR..."
for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        cp "$file" "$BACKUP_DIR/"
        echo "  ✓ Backed up $file"
    fi
done

echo ""
echo "Renaming tools..."

# Perform replacements
for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "Processing $file..."
        for tool in "${TOOLS[@]}"; do
            new_name="${tool/mobile_/mobile_device_mcp_}"
            # Use sed with in-place editing
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # macOS
                sed -i '' "s/\"${tool}\"/\"${new_name}\"/g" "$file"
            else
                # Linux
                sed -i "s/\"${tool}\"/\"${new_name}\"/g" "$file"
            fi
        done
        echo "  ✓ Updated $file"
    else
        echo "  ✗ File not found: $file"
    fi
done

echo ""
echo "=================================================="
echo "Renaming complete!"
echo ""
echo "Summary:"
echo "  - Renamed ${#TOOLS[@]} tool names"
echo "  - Updated ${#FILES[@]} files"
echo "  - Backups saved in $BACKUP_DIR"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Build: cargo build --release --features native-binary"
echo "  3. Test: ./test-tools.sh"
echo "  4. If issues, restore from $BACKUP_DIR"
