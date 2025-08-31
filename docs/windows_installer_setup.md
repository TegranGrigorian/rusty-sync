# Windows Installer Setup Guide

This guide walks you through creating a Windows installer for RustySync from start to finish.

## � File Organization

All Windows-related files are organized in the `distribution/windows/` folder:
- `build-windows.ps1` - Windows build script
- `setup-windows.ps1` - Automated setup script  
- `installer.iss` - Inno Setup configuration
- `detect-python.ps1` - Python detection helper
- `INSTALL_INFO.txt` - User installation instructions

## �📋 Prerequisites

Before starting, ensure you have administrator access to a Windows machine (not WSL).

## 🖥️ Phase 1: Install Required Tools

Open **PowerShell as Administrator** and run the following commands:

```powershell
# Install Rust
winget install Rustlang.Rust

# Install Visual Studio Build Tools (required for MSVC compiler)
winget install Microsoft.VisualStudio.2022.BuildTools

# Install Python (if not already installed)
winget install Python.Python.3.11

# Install Inno Setup (for creating the installer)
winget install JRSoftware.InnoSetup

# Install Git (if not already installed)
winget install Git.Git
```

**⚠️ Important:** After installing Rust, **restart PowerShell** to reload environment variables.

## 🔧 Phase 2: Configure Development Environment

Open a **new PowerShell window** and configure Rust:

```powershell
# Add Windows target for cross-compilation
rustup target add x86_64-pc-windows-msvc

# Verify installation
rustc --version
cargo --version
py --version  # Should show Python version
```

## 📁 Phase 3: Get the Source Code

```powershell
# Navigate to your development directory
cd C:\Development  # or your preferred location

# Clone the repository
git clone https://github.com/TegranGrigorian/rusty-sync
cd rusty-sync

# Switch to windows branch (if applicable)
git checkout windows
```

## 🐍 Phase 4: Set up Python Environment

### 4.1 Set up Development Python Environment

```powershell
# Navigate to the MinIO Python directory
cd src\core\minio

# Create virtual environment using py launcher
py -m venv .venv

# Activate the virtual environment
.\.venv\Scripts\Activate.ps1

# Install Python dependencies
pip install -r requirements.txt

# Test the setup
py -c "import boto3; print('boto3 imported successfully')"

# Deactivate when done
deactivate

# Return to project root
cd ..\..\..
```

### 4.2 Verify Python Setup

```powershell
# Test Python detection
.\distribution\windows\detect-python.ps1
```

This script will show you all available Python installations and recommend the best command to use.

## 🔨 Phase 5: Build the Application

### 5.1 Option A: Use the Launcher Script (Recommended)

```powershell
# Make sure you're in the project root
pwd  # Should show rusty-sync directory

# Build in release mode
.\run-windows-build.ps1 -Release

# Or run complete setup (first time)
.\run-windows-build.ps1 -Setup
```

### 5.2 Option B: Run Scripts Directly

```powershell
# Run the Windows build script directly
.\distribution\windows\build-windows.ps1 -Release
```

Both methods will:
- Build the Rust binary for Windows
- Create the `distribution/windows/installer/` directory structure
- Copy the executable and Python scripts
- Prepare everything for the installer

### 5.3 Verify Build Output

```powershell
# Check that files were created correctly
ls distribution\windows\installer\
ls distribution\windows\installer\bin\          # Should contain rusty-sync.exe
ls distribution\windows\installer\python\       # Should contain Python scripts
```

## 🐍 Phase 6: Set up Installer Python Environment

The installer needs its own Python environment that will be distributed:

```powershell
# Navigate to installer python directory
cd distribution\windows\installer\python

# Create virtual environment for distribution
py -m venv .

# Activate it
.\Scripts\Activate.ps1

# Install dependencies
pip install -r requirements.txt

# Verify installation works
py -c "import boto3; print('Installer Python environment ready')"

# Deactivate
deactivate

# Return to project root
cd ..\..\..\..
```

## 📦 Phase 7: Create the Windows Installer

### 7.1 Build with Inno Setup

```powershell
# Make sure you're in the project root
cd C:\Development\rusty-sync  # or your project path

# Run Inno Setup compiler
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" "distribution\windows\installer.iss"
```

### 7.2 Verify Installer Creation

```powershell
# Check if installer was created
ls distribution\windows\output\
# Should show: RustySync-Setup-0.1.0.exe

# Check installer size
$installer = Get-Item "distribution\windows\output\RustySync-Setup-0.1.0.exe"
Write-Host "Installer size: $($installer.Length / 1MB) MB"
```

## 🧪 Phase 8: Test the Installer

### 8.1 Install RustySync

```powershell
# Run the installer
.\distribution\windows\output\RustySync-Setup-0.1.0.exe
```

During installation:
- Choose installation directory (default: `C:\Program Files\RustySync`)
- Optionally check "Add RustySync to PATH"
- Complete the installation

### 8.2 Test Installation

Open a **new PowerShell window** (to pick up PATH changes):

```powershell
# Test the main executable
rusty-sync --help

# Test Python integration
cd "C:\Program Files\RustySync\python"
py -c "import boto3; print('All dependencies working!')"

# Test configuration
rusty-sync config
```

## 🚀 Automated Setup Script

For convenience, you can use the automated setup script:

### Option 1: From Project Root (Recommended)
```powershell
# Run complete setup including prerequisites
.\run-windows-build.ps1 -Setup

# Or just build (if prerequisites already installed)
.\run-windows-build.ps1 -Release
```

### Option 2: Direct Script Usage
```powershell
# First Run (Install Prerequisites) - Run as Administrator
.\distribution\windows\setup-windows.ps1
# Restart PowerShell when prompted

# Second Run (Build Everything) - Run in regular PowerShell  
.\distribution\windows\setup-windows.ps1 -SkipPrerequisites
```

The automated script will handle all the steps above automatically.

## 🔧 Troubleshooting

### Python Issues

**"py is not recognized":**
```powershell
# Install Python properly
winget install Python.Python.3.11
# Restart PowerShell
```

**Multiple Python installations:**
```powershell
# List all Python versions
py -0

# Use specific version
py -3.11 -m venv .venv
```

### Build Issues

**Rust compiler errors:**
```powershell
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
.\build-windows.ps1 -Release
```

**MSVC linker errors:**
```powershell
# Ensure Visual Studio Build Tools are installed
winget install Microsoft.VisualStudio.2022.BuildTools
```

### Installer Issues

**Inno Setup not found:**
```powershell
# Check installation
ls "C:\Program Files (x86)\Inno Setup 6\"

# Reinstall if needed
winget install JRSoftware.InnoSetup
```

**Permission errors:**
```powershell
# Run PowerShell as Administrator
# Or check antivirus software settings
```

**Path issues after moving files:**
```powershell
# Use the launcher script from project root
.\run-windows-build.ps1 -Release

# Or navigate properly when using direct scripts
cd distribution\windows
.\build-windows.ps1 -Release
```

## 📊 Expected File Sizes

- **rusty-sync.exe**: ~5-15 MB (depending on features)
- **Python environment**: ~50-100 MB (with dependencies)
- **Final installer**: ~60-120 MB

## 🎯 Final Checklist

- [ ] All prerequisites installed
- [ ] Rust builds successfully
- [ ] Python environment works
- [ ] Files organized in `distribution/windows/` folder
- [ ] Installer creates without errors in `distribution/windows/output/`
- [ ] Installer runs and installs correctly
- [ ] `rusty-sync --help` works after installation
- [ ] Python dependencies are available

## 📝 Notes

- The entire process takes about 15-30 minutes
- Internet connection required for downloading dependencies
- Antivirus software may flag the installer (this is normal)
- The `py` launcher is the recommended way to use Python on Windows
- All Windows build files are now organized in `distribution/windows/`

## 🔗 Related Files

- `run-windows-build.ps1` - Project root launcher script
- `distribution/windows/build-windows.ps1` - Windows build script
- `distribution/windows/setup-windows.ps1` - Automated setup script
- `distribution/windows/installer.iss` - Inno Setup configuration
- `distribution/windows/detect-python.ps1` - Python detection helper
- `distribution/windows/INSTALL_INFO.txt` - User installation instructions