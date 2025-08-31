#!/bin/bash
# build-deb.sh - Automated Debian package builder for rusty-sync

set -e  # Exit on any error

# Parse command line arguments
VERSION_BUMP=""
if [ "$1" = "--bump-version" ] || [ "$1" = "-v" ]; then
    VERSION_BUMP="true"
fi

echo "Building rusty-sync Debian package..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Run this script from the rusty-sync project root directory"
    exit 1
fi

# Step 0: Version bump (optional)
if [ "$VERSION_BUMP" = "true" ]; then
    echo "Step 0: Bumping version..."
    current_version=$(grep "Version:" rusty-sync/DEBIAN/control | cut -d' ' -f2)
    echo "Current version: $current_version"
    read -p "Enter new version (e.g., 0.1.3): " new_version
    if [ ! -z "$new_version" ]; then
        sed -i "s/Version:.*/Version: $new_version/" rusty-sync/DEBIAN/control
        echo "Updated version to $new_version"
    fi
fi

# Step 1: Build Rust binary
echo "Step 1: Building Rust binary..."
cargo build --release

if [ ! -f "target/release/rusty-sync" ]; then
    echo "Error: Binary not found after build"
    exit 1
fi

# Step 2: Prepare package structure
echo "Step 2: Preparing package structure..."
mkdir -p rusty-sync/usr/local/bin
mkdir -p rusty-sync/usr/local/share/rusty-sync/src

# Step 3: Copy files
echo "Step 3: Copying files..."

# Copy main binary
cp target/release/rusty-sync rusty-sync/usr/local/bin/
chmod +x rusty-sync/usr/local/bin/rusty-sync

# Copy Python scripts
cp src/core/minio/main.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/test.py rusty-sync/usr/local/share/rusty-sync/
cp src/core/minio/src/minio_util.py rusty-sync/usr/local/share/rusty-sync/src/

# Make Python scripts executable
chmod +x rusty-sync/usr/local/share/rusty-sync/*.py

# Verify DEBIAN directory exists
if [ ! -d "rusty-sync/DEBIAN" ]; then
    echo "Error: rusty-sync/DEBIAN directory not found"
    exit 1
fi

# Step 4: Build package
echo "Step 4: Building .deb package..."
dpkg-deb --build rusty-sync

# Check if package was created
if [ -f "rusty-sync.deb" ]; then
    echo "Package built successfully: rusty-sync.deb"
    echo ""
    echo "Package info:"
    dpkg-deb --info rusty-sync.deb
    echo ""
    echo "Install with:"
    echo "   sudo dpkg -i rusty-sync.deb"
    echo ""
    echo "Update existing installation:"
    echo "   sudo dpkg -i rusty-sync.deb"
    echo ""
    echo "Quick test installation? (y/n)"
    read -r response
    if [ "$response" = "y" ] || [ "$response" = "Y" ]; then
        echo "Testing installation..."
        sudo dpkg -i rusty-sync.deb
        echo "Installation complete!"
        echo "Testing basic functionality..."
        rusty-sync --help | head -3
        echo "Package test successful!"
    fi
else
    echo "Error: Package build failed"
    exit 1
fi
