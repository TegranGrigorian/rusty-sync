# Debian Package Setup Guide

This guide explains how to build and install the Debian package for rusty-sync.

## Prerequisites

Make sure you have the required tools installed:

```bash
sudo apt update
sudo apt install build-essential dpkg-dev
```

## Package Structure

The Debian package structure is:
```
rusty-sync/
├── DEBIAN/
│   ├── control          # Package metadata
│   ├── postinst         # Post-installation script
│   └── prerm           # Pre-removal script
└── usr/
    └── local/
        ├── bin/
        │   └── rusty-sync     # Main binary
        └── share/
            └── rusty-sync/
                ├── main.py           # Python MinIO interface
                ├── src/
                │   └── minio_util.py # MinIO utilities
                └── test.py          # Test script
```

## Quick Build (Recommended)

Use the automated build script:

```bash
# Simple build (from project root)
./distribution/linux/build-deb.sh

# Build with version bump
./distribution/linux/build-deb.sh --bump-version

# Or run from the distribution/linux directory
cd distribution/linux
./build-deb.sh
```

The script will:
1. Build the Rust binary
2. Prepare the package structure
3. Copy all necessary files
4. Build the .deb package
5. Offer to test install it

## Manual Build Process

### Step 1: Build the Rust Binary

First, build the optimized release binary:

```bash
cd /path/to/rusty-sync
cargo build --release
```

### Step 2: Prepare Package Directory

Copy the binary to the package structure:

```bash
# Create the directory structure if it doesn't exist
mkdir -p rusty-sync/usr/local/bin
mkdir -p rusty-sync/usr/local/share/rusty-sync/src

# Copy the binary
cp target/release/rusty-sync rusty-sync/usr/local/bin/

# Make it executable
chmod +x rusty-sync/usr/local/bin/rusty-sync

# Copy Python scripts
cp src/core/minio/main.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/test.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/src/minio_util.py rusty-sync/usr/local/share/rusty-sync/src/

# Make scripts executable
chmod +x rusty-sync/usr/local/share/rusty-sync/*.py
```

### Step 3: Update Package Version (Optional)

Edit `rusty-sync/DEBIAN/control` to update version:

```bash
nano rusty-sync/DEBIAN/control
```

Update the Version field:
```
Version: 0.1.3
```

### Step 4: Build the Package

Build the .deb package:

```bash
dpkg-deb --build rusty-sync
```

This creates `rusty-sync.deb` in the current directory.

### Step 5: Install the Package

Install the package:

```bash
sudo dpkg -i rusty-sync.deb
```

If you get dependency errors, fix them with:

```bash
sudo apt-get install -f
```

## Automated Build Script

Create a build script for convenience:

```bash
#!/bin/bash
# build-deb.sh

echo "Building rusty-sync Debian package..."

# Build Rust binary
echo "Step 1: Building Rust binary..."
cargo build --release

# Prepare package structure
echo "Step 2: Preparing package structure..."
mkdir -p rusty-sync/usr/local/bin
mkdir -p rusty-sync/usr/local/share/rusty-sync/src

# Copy files
echo "Step 3: Copying files..."
cp target/release/rusty-sync rusty-sync/usr/local/bin/
chmod +x rusty-sync/usr/local/bin/rusty-sync

cp src/core/minio/main.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/test.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/src/minio_util.py rusty-sync/usr/local/share/rusty-sync/src/
chmod +x rusty-sync/usr/local/share/rusty-sync/*.py

# Build package
echo "Step 4: Building .deb package..."
dpkg-deb --build rusty-sync

echo "Package built: rusty-sync.deb"
echo "Install with: sudo dpkg -i rusty-sync.deb"
```

Make it executable:
```bash
chmod +x build-deb.sh
```

## Updating an Existing Installation

To update an existing installation:

```bash
# Remove old version
sudo dpkg -r rusty-sync

# Install new version
sudo dpkg -i rusty-sync.deb
```

Or simply install over the old version:
```bash
sudo dpkg -i rusty-sync.deb
```

## Verification

After installation, verify it works:

```bash
# Check installation
which rusty-sync
# Should output: /usr/local/bin/rusty-sync

# Test basic functionality
rusty-sync --help

# Test configuration
rusty-sync config
```

## Troubleshooting

### Common Issues

1. **Permission denied**: Make sure scripts are executable
   ```bash
   chmod +x rusty-sync/usr/local/bin/rusty-sync
   chmod +x rusty-sync/usr/local/share/rusty-sync/*.py
   ```

2. **Python dependencies missing**: The postinst script handles this, but manually:
   ```bash
   cd /usr/local/share/rusty-sync
   python3 -m venv .venv
   .venv/bin/pip install boto3
   ```

3. **Package build fails**: Check directory structure and permissions

4. **Installation fails**: Check for dependency issues
   ```bash
   sudo apt-get install -f
   ```

## Files Created by Package

After installation, these files will exist:
- `/usr/local/bin/rusty-sync` - Main executable
- `/usr/local/share/rusty-sync/` - Python support files
- `/usr/local/share/rusty-sync/.venv/` - Python virtual environment (created during install)

User configuration files:
- `~/.rusty-sync/config.json` - User configuration (created on first run)
