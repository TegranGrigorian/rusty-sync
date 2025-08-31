#!/bin/bash
# build-deb.sh - Automated Debian package builder for rusty-sync
# Can be run from project root or from distribution/linux/ directory

set -e  # Exit on any error

# Parse command line arguments
VERSION_BUMP=""
if [ "$1" = "--bump-version" ] || [ "$1" = "-v" ]; then
    VERSION_BUMP="true"
fi

echo "Building rusty-sync Debian package..."

# Navigate to project root if we're in the distribution/linux folder
CURRENT_DIR=$(pwd)
if [[ "$CURRENT_DIR" == *"distribution/linux"* ]] || [[ "$CURRENT_DIR" == *"distribution\\linux"* ]]; then
    echo "Detected running from distribution/linux folder, navigating to project root..."
    cd "../.."
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cannot find Cargo.toml. Please run this script from the rusty-sync project root directory"
    echo "Current directory: $(pwd)"
    exit 1
fi

PROJECT_ROOT=$(pwd)
echo "Project root: $PROJECT_ROOT"

# Step 0: Version bump (optional)
if [ "$VERSION_BUMP" = "true" ]; then
    echo "Step 0: Bumping version..."
    # Create temp DEBIAN structure to read current version
    mkdir -p rusty-sync/DEBIAN
    if [ ! -f "rusty-sync/DEBIAN/control" ]; then
        current_version="0.1.0"
        echo "No existing control file found, current version: $current_version"
    else
        current_version=$(grep "Version:" rusty-sync/DEBIAN/control | cut -d' ' -f2)
        echo "Current version: $current_version"
    fi
    read -p "Enter new version (e.g., 0.1.3): " new_version
    if [ ! -z "$new_version" ]; then
        VERSION_TO_USE="$new_version"
        echo "Will use version: $VERSION_TO_USE"
    else
        VERSION_TO_USE="$current_version"
    fi
else
    VERSION_TO_USE="0.1.0"
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
mkdir -p rusty-sync/DEBIAN

# Create DEBIAN control file if it doesn't exist
if [ ! -f "rusty-sync/DEBIAN/control" ]; then
    echo "Creating DEBIAN control file with version $VERSION_TO_USE..."
    cat > rusty-sync/DEBIAN/control << EOF
Package: rusty-sync
Version: $VERSION_TO_USE
Section: utils
Priority: optional
Architecture: amd64
Depends: python3, python3-pip, python3-venv
Maintainer: TegranGrigorian <your-email@domain.com>
Description: Cross-platform file synchronization tool
 RustySync is a cross-platform file synchronization tool built in Rust
 with MinIO backend support. It provides intelligent file change detection,
 Git repository handling, and a CLI interface for easy automation.
EOF
else
    # Update existing control file with new version if specified
    if [ "$VERSION_BUMP" = "true" ] && [ ! -z "$new_version" ]; then
        sed -i "s/Version:.*/Version: $VERSION_TO_USE/" rusty-sync/DEBIAN/control
        echo "Updated version to $VERSION_TO_USE"
    fi
fi

# Create postinst script if it doesn't exist
if [ ! -f "rusty-sync/DEBIAN/postinst" ]; then
    echo "Creating DEBIAN postinst script..."
    cat > rusty-sync/DEBIAN/postinst << 'EOF'
#!/bin/bash
# Post-installation script for rusty-sync

echo "Setting up Python environment for rusty-sync..."

# Navigate to the app directory
cd /usr/local/share/rusty-sync

# Create virtual environment
python3 -m venv .venv

# Activate virtual environment and install dependencies
.venv/bin/pip install boto3

echo "Python environment setup complete."
echo ""
echo "RustySync has been installed successfully!"
echo "Run 'rusty-sync --help' to get started."
EOF
    chmod +x rusty-sync/DEBIAN/postinst
fi

# Create prerm script if it doesn't exist
if [ ! -f "rusty-sync/DEBIAN/prerm" ]; then
    echo "Creating DEBIAN prerm script..."
    cat > rusty-sync/DEBIAN/prerm << 'EOF'
#!/bin/bash
# Pre-removal script for rusty-sync

echo "Cleaning up rusty-sync..."

# Remove virtual environment if it exists
if [ -d "/usr/local/share/rusty-sync/.venv" ]; then
    rm -rf /usr/local/share/rusty-sync/.venv
fi

exit 0
EOF
    chmod +x rusty-sync/DEBIAN/prerm
fi

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
