# Windows Support Guide

Complete guide for using Mobile Device MCP Server on Windows platforms.

## Overview

The Mobile Device MCP Server fully supports Windows 10/11 for Android device automation. iOS support requires macOS and is not available on Windows.

## Support Matrix

| Feature | Windows 10 | Windows 11 | Notes |
|---------|------------|------------|-------|
| Android Device Support | ✅ | ✅ | Full support via ADB |
| Android Emulator Support | ✅ | ✅ | Android Studio emulator |
| iOS Device Support | ❌ | ❌ | Requires macOS |
| iOS Simulator Support | ❌ | ❌ | Requires macOS + Xcode |
| Native Binary | ✅ | ✅ | Windows x86_64 |
| Zed Extension | ✅ | ✅ | Via WASM |

## Prerequisites

### 1. Install Rust

**Using rustup (Recommended):**

```powershell
# Download and run rustup-init.exe from:
# https://rustup.rs/

# Or using winget:
winget install Rustlang.Rustup

# Verify installation
rustc --version
cargo --version
```

### 2. Install Android Platform Tools

**Method 1: Using Chocolatey**

```powershell
# Install Chocolatey first (if not already installed)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install ADB
choco install adb

# Verify installation
adb version
```

**Method 2: Using Scoop**

```powershell
# Install Scoop first (if not already installed)
irm get.scoop.sh | iex

# Install ADB
scoop install adb

# Verify installation
adb version
```

**Method 3: Manual Installation**

1. Download Android Platform Tools:
   - Visit: https://developer.android.com/tools/releases/platform-tools
   - Download ZIP for Windows

2. Extract to a permanent location:
   ```powershell
   Expand-Archive -Path platform-tools-latest-windows.zip -DestinationPath C:\platform-tools
   ```

3. Add to PATH:
   ```powershell
   # Via PowerShell (Current User)
   $env:Path += ";C:\platform-tools"
   [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
   
   # Or via System Settings:
   # Settings → System → About → Advanced system settings
   # → Environment Variables → Path → Edit → New
   # Add: C:\platform-tools
   ```

4. Verify:
   ```powershell
   adb version
   ```

### 3. Install Zed Editor (Optional)

```powershell
# Download from: https://zed.dev
# Or using winget:
winget install Zed.Zed
```

## Building from Source

### Build Native Binary

```powershell
# Clone repository
git clone https://github.com/sorinirimies/mobiledevice-mcp-zed-extension
cd mobiledevice-mcp-zed-extension

# Build release binary
cargo build --release --features native-binary

# Binary location
.\target\release\mobile-device-mcp-server.exe
```

### Build Zed Extension (WASM)

```powershell
# Add WASM target
rustup target add wasm32-wasip1

# Build extension
cargo build --release --target wasm32-wasip1

# Extension location
.\target\wasm32-wasip1\release\mobile_device_mcp.wasm
```

## Installation

### Option 1: Build and Install Locally

```powershell
# Build and install to ~/.cargo/bin
cargo install --path . --features native-binary

# Verify installation
mobile-device-mcp-server.exe --version
```

### Option 2: Manual Installation

```powershell
# Build binary
cargo build --release --features native-binary

# Copy to a location in PATH
Copy-Item .\target\release\mobile-device-mcp-server.exe -Destination C:\Users\$env:USERNAME\.cargo\bin\

# Or create a dedicated directory
New-Item -ItemType Directory -Force -Path C:\mcp-servers
Copy-Item .\target\release\mobile-device-mcp-server.exe -Destination C:\mcp-servers\
$env:Path += ";C:\mcp-servers"
```

## Android Device Setup

### Physical Devices

1. **Enable Developer Options:**
   - Go to Settings → About Phone
   - Tap "Build Number" 7 times
   - Developer Options will appear

2. **Enable USB Debugging:**
   - Settings → Developer Options → USB Debugging
   - Toggle ON

3. **Connect Device:**
   - Connect via USB
   - Accept "Allow USB Debugging" prompt
   - Check "Always allow from this computer"

4. **Verify Connection:**
   ```powershell
   adb devices
   
   # Expected output:
   # List of devices attached
   # ABC123DEF456    device
   ```

### Android Emulator (Android Studio)

1. **Install Android Studio:**
   ```powershell
   winget install Google.AndroidStudio
   ```

2. **Create Virtual Device:**
   - Open Android Studio
   - Tools → Device Manager
   - Create Device → Select hardware → Download system image
   - Finish

3. **Start Emulator:**
   ```powershell
   # Via Android Studio: Device Manager → Play button
   
   # Or via command line:
   emulator -list-avds
   emulator -avd Pixel_6_API_34
   ```

4. **Verify Connection:**
   ```powershell
   adb devices
   # Output: emulator-5554    device
   ```

## Usage

### Running the MCP Server

```powershell
# Start server
mobile-device-mcp-server.exe

# With debug logging
$env:MOBILE_DEVICE_MCP_DEBUG = "1"
mobile-device-mcp-server.exe

# Specify platform
$env:MOBILE_PLATFORM = "android"
mobile-device-mcp-server.exe
```

### Example Request

```powershell
# List devices
@'
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_list_available_devices",
    "arguments": {}
  }
}
'@ | mobile-device-mcp-server.exe

# Take screenshot
@'
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_take_screenshot",
    "arguments": {
      "device_id": "emulator-5554",
      "platform": "android"
    }
  }
}
'@ | mobile-device-mcp-server.exe
```

### Integration with Zed

1. **Install Zed Extension:**
   - Open Zed
   - Extensions → Search "Mobile Device MCP"
   - Install

2. **Configure Settings:**

   Edit `%APPDATA%\Zed\settings.json`:

   ```json
   {
     "context_servers": {
       "mcp-server-mobile-device": {
         "settings": {
           "debug": false,
           "platform": "android"
         }
       }
     }
   }
   ```

3. **Restart Zed**

4. **Test in Assistant:**
   - Open Zed Assistant
   - Type: "List my mobile devices"

## Windows-Specific Considerations

### Line Endings

**Issue:** ADB output may contain Windows line endings (`\r\n`) that need handling.

**Solution:** The server automatically handles line ending conversions:

```rust
// Handled internally
let output = String::from_utf8_lossy(&output.stdout)
    .replace("\r\n", "\n");
```

### Path Separators

**Issue:** Windows uses backslashes (`\`) for paths.

**Solution:** Use forward slashes or double backslashes:

```powershell
# Good
mobile-device-mcp-server.exe --config C:/config/settings.json

# Also good
mobile-device-mcp-server.exe --config C:\\config\\settings.json
```

### Firewall

**Issue:** Windows Firewall may block ADB server.

**Solution:** Allow ADB server through firewall:

```powershell
# Add firewall rule
New-NetFirewallRule -DisplayName "ADB Server" -Direction Inbound -Program "C:\platform-tools\adb.exe" -Action Allow

# Or via GUI:
# Windows Security → Firewall & network protection
# → Allow an app through firewall → Add adb.exe
```

### Antivirus

**Issue:** Some antivirus software may flag ADB or the MCP server.

**Solution:** Add exceptions:

- Windows Defender: Settings → Virus & threat protection → Exclusions
- Add `C:\platform-tools`
- Add `%USERPROFILE%\.cargo\bin`

### PowerShell Execution Policy

**Issue:** Scripts may be blocked by execution policy.

**Solution:**

```powershell
# Check current policy
Get-ExecutionPolicy

# Allow scripts for current user
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Or run scripts with bypass
powershell -ExecutionPolicy Bypass -File script.ps1
```

## Troubleshooting

### ADB Device Not Found

**Problem:** `adb devices` shows "unauthorized" or device not listed.

**Solutions:**

1. **Revoke USB debugging authorizations:**
   - Device Settings → Developer Options
   - Revoke USB debugging authorizations
   - Disconnect and reconnect device
   - Accept prompt again

2. **Restart ADB server:**
   ```powershell
   adb kill-server
   adb start-server
   ```

3. **Check USB driver:**
   ```powershell
   # Open Device Manager
   devmgmt.msc
   
   # Look for device under "Android Device" or "Other devices"
   # If driver issue, download manufacturer's USB driver
   ```

4. **Try different USB cable/port:**
   - Use data-capable USB cable (not charge-only)
   - Try different USB ports
   - Avoid USB hubs if possible

### Emulator Won't Start

**Problem:** Android emulator fails to start or is extremely slow.

**Solutions:**

1. **Enable Hyper-V (Windows 11 / Windows 10 Pro):**
   ```powershell
   # Check if Hyper-V is enabled
   Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V
   
   # Enable Hyper-V
   Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
   
   # Restart required
   Restart-Computer
   ```

2. **Enable Windows Hypervisor Platform:**
   ```powershell
   Enable-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform
   ```

3. **Allocate more resources:**
   - Android Studio → Device Manager
   - Edit AVD → Show Advanced Settings
   - Increase RAM (4GB+)
   - Increase Internal Storage (2GB+)

4. **Use x86_64 image:**
   - Faster than ARM images on x86 Windows
   - Download Intel x86 system image in AVD Manager

### Build Errors

**Problem:** Compilation fails on Windows.

**Solutions:**

1. **Visual Studio Build Tools missing:**
   ```powershell
   # Install VS Build Tools
   winget install Microsoft.VisualStudio.2022.BuildTools
   
   # Or download from:
   # https://visualstudio.microsoft.com/downloads/
   # Select: Desktop development with C++
   ```

2. **OpenSSL dependency issues:**
   ```powershell
   # Install via vcpkg
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   .\vcpkg install openssl:x64-windows
   
   # Or use pre-built binaries
   # Download from: https://slproweb.com/products/Win32OpenSSL.html
   ```

3. **Link errors:**
   ```powershell
   # Ensure Rust is up to date
   rustup update
   
   # Clean and rebuild
   cargo clean
   cargo build --release --features native-binary
   ```

### Permission Errors

**Problem:** Access denied when installing or running.

**Solutions:**

```powershell
# Run PowerShell as Administrator
Start-Process powershell -Verb RunAs

# Or adjust file permissions
icacls "C:\path\to\file.exe" /grant:r "$env:USERNAME:(RX)"
```

### Environment Variable Not Persisting

**Problem:** PATH changes don't persist after closing PowerShell.

**Solution:**

```powershell
# Set permanently for user
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\platform-tools",
    "User"
)

# Verify (restart PowerShell)
$env:Path
```

## Performance Optimization

### ADB Connection Speed

```powershell
# Use USB 3.0 port if available
# USB 2.0: ~30 MB/s
# USB 3.0: ~150 MB/s

# Check USB version in Device Manager
devmgmt.msc
```

### Emulator Performance

```powershell
# Enable hardware acceleration (HAXM or WHPX)
# Allocate more RAM to emulator (4-8GB)
# Use x86 system images
# Enable GPU acceleration in AVD settings
```

### Parallel Device Testing

```powershell
# Run multiple tests in parallel using Start-Job
$devices = @("emulator-5554", "emulator-5556")

$jobs = $devices | ForEach-Object {
    Start-Job -ScriptBlock {
        param($device)
        echo "Testing $device"
        # Run tests here
    } -ArgumentList $_
}

$jobs | Wait-Job | Receive-Job
$jobs | Remove-Job
```

## Best Practices for Windows

### 1. Use PowerShell Core (PowerShell 7+)

```powershell
# Install PowerShell 7
winget install Microsoft.PowerShell

# Better performance and cross-platform compatibility
```

### 2. Path Handling

```rust
// Use PathBuf for cross-platform paths
use std::path::PathBuf;

let path = PathBuf::from("C:/platform-tools");
let adb = path.join("adb.exe");
```

### 3. Long Path Support

```powershell
# Enable long paths (Windows 10 1607+)
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
  -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

### 4. Automated Testing

```powershell
# Create test script: test-android.ps1
param(
    [string]$DeviceId = "emulator-5554"
)

$ErrorActionPreference = "Stop"

Write-Host "Testing MCP Server on $DeviceId" -ForegroundColor Green

# Test 1: List devices
Write-Host "`nTest 1: List devices" -ForegroundColor Yellow
@'
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_list_available_devices","arguments":{}}}
'@ | mobile-device-mcp-server.exe | ConvertFrom-Json | Select-Object -ExpandProperty result

# Test 2: Take screenshot
Write-Host "`nTest 2: Take screenshot" -ForegroundColor Yellow
$result = @'
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mobile_device_mcp_take_screenshot","arguments":{"device_id":"emulator-5554","platform":"android"}}}
'@ | mobile-device-mcp-server.exe | ConvertFrom-Json

if ($result.result.content[0].data) {
    Write-Host "✓ Screenshot captured" -ForegroundColor Green
} else {
    Write-Host "✗ Screenshot failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n✓ All tests passed!" -ForegroundColor Green
```

## CI/CD Integration

### GitHub Actions (Windows)

```yaml
name: Windows Build

on: [push, pull_request]

jobs:
  build-windows:
    runs-on: windows-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: x86_64-pc-windows-msvc
        override: true
    
    - name: Install ADB
      run: choco install adb
    
    - name: Build
      run: cargo build --release --features native-binary
    
    - name: Run tests
      run: cargo test --release --features native-binary
    
    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: mobile-device-mcp-server-windows
        path: target/release/mobile-device-mcp-server.exe
```

## Additional Resources

- [Windows Terminal](https://aka.ms/terminal) - Modern terminal for Windows
- [PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/)
- [Android Developer Tools](https://developer.android.com/tools)
- [Windows Subsystem for Android](https://learn.microsoft.com/en-us/windows/android/wsa/)
- [Rust on Windows](https://www.rust-lang.org/tools/install)

## Known Limitations

1. **iOS Support:** Not available on Windows (requires macOS)
2. **Simulator Support:** iOS simulators require Xcode (macOS only)
3. **Path Length:** Windows has 260 character path limit (enable long paths)
4. **Case Sensitivity:** File system is case-insensitive (unlike Linux/macOS)
5. **WSL:** Windows Subsystem for Linux not currently supported

## Future Enhancements

- Windows Subsystem for Android (WSA) integration
- Native Windows GUI for device management
- Windows performance counters and telemetry
- MSI installer package
- Windows Terminal integration

---

**Last Updated:** 2024-11-01  
**Windows Version:** 10/11 (64-bit)  
**Status:** Production Ready (Android Only)  
**Architecture:** x86_64