#!/bin/bash
# build-tar-gz.sh - Automated tar.gz package builder for rusty-sync
# Can be run from project root or from distribution/linux/ directory

set -e  # Exit on any error

# Parse command line arguments
VERSION_BUMP=""
COMPRESS_XZ=""
DELETE_SOURCE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --bump-version|-v)
            VERSION_BUMP="true"
            shift
            ;;
        --xz|-x)
            COMPRESS_XZ="true"
            shift
            ;;
        --delete|-d)
            DELETE_SOURCE="true"
            shift
            ;;
        --help|-h)
            echo "RustySync tar.gz Package Builder"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "OPTIONS:"
            echo "  --bump-version, -v    Prompt for version bump before building"
            echo "  --xz, -x             Use xz compression instead of gzip"
            echo "  --delete, -d         Delete source directory after compression"
            echo "  --help, -h           Show this help message"
            echo ""
            echo "EXAMPLES:"
            echo "  $0                   # Build standard tar.gz package"
            echo "  $0 --xz             # Build with xz compression (smaller)"
            echo "  $0 -v               # Prompt for version bump"
            echo "  $0 -v --xz -d       # Version bump, xz compression, delete source"
            echo ""
            echo "OUTPUT:"
            echo "  Package will be created in: distribution/linux/packages/tar/"
            echo ""
            echo "INTEGRATION:"
            echo "  Uses the 'rat' tool for compression. Make sure it's installed."
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "OPTIONS:"
            echo "  --bump-version, -v    Prompt for version bump before building"
            echo "  --xz, -x             Use xz compression instead of gzip"
            echo "  --delete, -d         Delete source directory after compression"
            echo "  --help, -h           Show this help message"
            echo ""
            echo "EXAMPLES:"
            echo "  $0                   # Build standard tar.gz package"
            echo "  $0 --xz             # Build with xz compression (smaller)"
            echo "  $0 -v               # Prompt for version bump"
            echo "  $0 -v --xz -d       # Version bump, xz compression, delete source"
            echo ""
            echo "OUTPUT:"
            echo "  Package will be created in: distribution/linux/packages/tar/"
            exit 0
            ;;
    esac
done

echo "Building rusty-sync tar.gz package..."

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

# Check if rat tool is available
if ! command -v rat &> /dev/null; then
    echo "Error: 'rat' tool not found. Please install it first."
    echo "You can find it at: https://github.com/yourusername/rat"
    exit 1
fi

# Step 0: Version determination (optional bump)
if [ "$VERSION_BUMP" = "true" ]; then
    echo "Step 0: Bumping version..."
    # Read current version from Cargo.toml
    current_version=$(grep "^version" Cargo.toml | cut -d'"' -f2)
    echo "Current version: $current_version"
    read -p "Enter new version (e.g., 0.1.3): " new_version
    if [ ! -z "$new_version" ]; then
        VERSION_TO_USE="$new_version"
        echo "Will use version: $VERSION_TO_USE"
        # Update Cargo.toml
        sed -i "s/^version = \".*\"/version = \"$VERSION_TO_USE\"/" Cargo.toml
        echo "Updated Cargo.toml version to $VERSION_TO_USE"
    else
        VERSION_TO_USE="$current_version"
    fi
else
    VERSION_TO_USE=$(grep "^version" Cargo.toml | cut -d'"' -f2)
fi

echo "Using version: $VERSION_TO_USE"

# Step 1: Build Rust binary
echo "Step 1: Building Rust binary..."
cargo build --release

if [ ! -f "target/release/rusty-sync" ]; then
    echo "Error: Binary not found after build"
    exit 1
fi

# Step 2: Prepare portable package structure
echo "Step 2: Preparing portable package structure..."
PACKAGE_NAME="rusty-sync-$VERSION_TO_USE"
PACKAGE_DIR="$PACKAGE_NAME"

# Clean up any existing package directory
rm -rf "$PACKAGE_DIR"

# Create package structure
mkdir -p "$PACKAGE_DIR/bin"
mkdir -p "$PACKAGE_DIR/lib/python"
mkdir -p "$PACKAGE_DIR/docs"
mkdir -p "$PACKAGE_DIR/scripts"

echo "Step 3: Copying files..."

# Copy main binary
cp target/release/rusty-sync "$PACKAGE_DIR/bin/"
chmod +x "$PACKAGE_DIR/bin/rusty-sync"

# Copy Python scripts for MinIO support
cp src/core/minio/main.py "$PACKAGE_DIR/lib/python/"
cp src/core/minio/test.py "$PACKAGE_DIR/lib/python/"
cp src/core/minio/src/minio_util.py "$PACKAGE_DIR/lib/python/"
cp src/core/minio/requirements.txt "$PACKAGE_DIR/lib/python/" 2>/dev/null || echo "# Python dependencies" > "$PACKAGE_DIR/lib/python/requirements.txt"

# Make Python scripts executable
chmod +x "$PACKAGE_DIR/lib/python"/*.py

# Copy documentation
cp README.md "$PACKAGE_DIR/" 2>/dev/null || echo "# RustySync" > "$PACKAGE_DIR/README.md"
cp LICENSE "$PACKAGE_DIR/" 2>/dev/null || echo "# License information" > "$PACKAGE_DIR/LICENSE"
cp docs/*.md "$PACKAGE_DIR/docs/" 2>/dev/null || true

# Create installation script
cat > "$PACKAGE_DIR/install.sh" << 'EOF'
#!/bin/bash
# Installation script for rusty-sync portable package

set -e

INSTALL_DIR="/usr/local"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing rusty-sync..."

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run with sudo:"
    echo "sudo ./install.sh"
    exit 1
fi

# Create directories
mkdir -p "$INSTALL_DIR/bin"
mkdir -p "$INSTALL_DIR/share/rusty-sync"

# Copy binary
cp "$SCRIPT_DIR/bin/rusty-sync" "$INSTALL_DIR/bin/"
chmod +x "$INSTALL_DIR/bin/rusty-sync"

# Copy Python support files
cp -r "$SCRIPT_DIR/lib/python"/* "$INSTALL_DIR/share/rusty-sync/"

# Set up Python environment
cd "$INSTALL_DIR/share/rusty-sync"
python3 -m venv .venv
.venv/bin/pip install boto3

echo "Installation complete!"
echo "You can now use 'rusty-sync' from anywhere."
echo "Run 'rusty-sync --help' to get started."
EOF

chmod +x "$PACKAGE_DIR/install.sh"

# Create uninstallation script
cat > "$PACKAGE_DIR/uninstall.sh" << 'EOF'
#!/bin/bash
# Uninstallation script for rusty-sync

set -e

INSTALL_DIR="/usr/local"

echo "Uninstalling rusty-sync..."

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run with sudo:"
    echo "sudo ./uninstall.sh"
    exit 1
fi

# Remove files
rm -f "$INSTALL_DIR/bin/rusty-sync"
rm -rf "$INSTALL_DIR/share/rusty-sync"

echo "Uninstallation complete!"
EOF

chmod +x "$PACKAGE_DIR/uninstall.sh"

# Create README for the package
cat > "$PACKAGE_DIR/INSTALL.md" << EOF
# RustySync Portable Package v$VERSION_TO_USE

## Installation

### Option 1: Automatic Installation (Recommended)
\`\`\`bash
sudo ./install.sh
\`\`\`

### Option 2: Manual Installation
1. Copy \`bin/rusty-sync\` to a directory in your PATH (e.g., \`/usr/local/bin/\`)
2. Copy \`lib/python/*\` to \`/usr/local/share/rusty-sync/\`
3. Set up Python environment:
   \`\`\`bash
   cd /usr/local/share/rusty-sync
   python3 -m venv .venv
   .venv/bin/pip install boto3
   \`\`\`

## Usage

After installation, run:
\`\`\`bash
rusty-sync --help
\`\`\`

## Configuration

First-time setup:
\`\`\`bash
rusty-sync config
\`\`\`

## Uninstallation

\`\`\`bash
sudo ./uninstall.sh
\`\`\`

## Package Contents

- \`bin/rusty-sync\` - Main executable
- \`lib/python/\` - Python MinIO support files
- \`docs/\` - Documentation
- \`install.sh\` - Installation script
- \`uninstall.sh\` - Uninstallation script

## System Requirements

- Linux (amd64)
- Python 3.6+
- python3-pip
- python3-venv

Build date: $(date)
EOF

echo "Step 4: Creating archive with rat..."

# Determine output filename
if [ "$COMPRESS_XZ" = "true" ]; then
    OUTPUT_FILE="$PACKAGE_NAME.tar.xz"
    RAT_ARGS="-x"
else
    OUTPUT_FILE="$PACKAGE_NAME.tar.gz"
    RAT_ARGS=""
fi

# Add delete flag if specified
if [ "$DELETE_SOURCE" = "true" ]; then
    RAT_ARGS="$RAT_ARGS -d"
fi

# Create archive using rat
rat $RAT_ARGS "$PACKAGE_DIR"

# Determine the actual output filename created by rat
if [ "$COMPRESS_XZ" = "true" ]; then
    ACTUAL_OUTPUT="$PACKAGE_NAME.tar.xz"
else
    ACTUAL_OUTPUT="$PACKAGE_NAME.tar.gz"
fi

# Clean up source directory if not using delete flag
if [ "$DELETE_SOURCE" != "true" ]; then
    rm -rf "$PACKAGE_DIR"
fi

# Move to distribution folder
mkdir -p "distribution/linux/packages/tar"
mv "$ACTUAL_OUTPUT" "distribution/linux/packages/tar/"

echo "✅ Package built successfully: distribution/linux/packages/tar/$ACTUAL_OUTPUT"
echo ""
echo "📦 Package info:"
echo "   Version: $VERSION_TO_USE"
echo "   Size: $(du -h "distribution/linux/packages/tar/$ACTUAL_OUTPUT" | cut -f1)"
echo "   Location: distribution/linux/packages/tar/$ACTUAL_OUTPUT"
echo ""
echo "🚀 To extract and install:"
echo "   rat -d distribution/linux/packages/tar/$ACTUAL_OUTPUT"
echo "   sudo ./$PACKAGE_NAME/install.sh"
echo ""
echo "📋 Package contents:"
echo "   - Portable binary (bin/rusty-sync)"
echo "   - Python MinIO support (lib/python/)"
echo "   - Auto-installation scripts"
echo "   - Documentation"
