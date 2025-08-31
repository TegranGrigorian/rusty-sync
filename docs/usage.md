# Rusty Sync - Usage Guide

**OneDrive-like File Synchronization with MinIO Storage**

Rusty Sync provides git-like commands for seamless file synchronization between your local machine and MinIO cloud storage. Think of it as "Git for your files" with OneDrive-like functionality.

## 🚀 Quick Start

### Prerequisites
1. **MinIO Server** running and accessible
2. **Environment Setup**: Create a `.env` file in the project root:
   ```env
   MINIO_ENDPOINT_URL=http://your-minio-server:9000
   MINIO_ACCESS_KEY=your-access-key
   MINIO_SECRET_KEY=your-secret-key
   ```

### Build the Project
```bash
cargo build --release
# Binary will be available at ./target/release/rusty-sync
```

## 📖 Command Overview

Rusty Sync supports two command styles:

### Git-like Commands (Recommended)
- `remote` - List available buckets
- `clone` - Download entire bucket to local folder
- `status` - Show sync status of local folder
- `push` - Upload local changes to bucket
- `pull` - Download changes from bucket

### Classic Commands
- `-i, --init` - Initialize folder for sync
- `-s, --sync` - Sync folder to bucket
- `-t, --test` - Run comprehensive tests
- `-h, --help` - Show help

## 🎯 Git-like Workflow (Recommended)

### 1. Discover Available Buckets
```bash
rusty-sync remote
```
**Output:**
```
🌐 Discovering available buckets on MinIO server...
📦 Available buckets:
  1. my-documents
  2. photos-2024
  3. project-files
```

### 2. Clone a Bucket
Download an entire bucket to your local machine:

```bash
# Clone bucket to a new folder
rusty-sync clone my-documents ./my-local-docs

# Clone to current directory
rusty-sync clone photos-2024 .
```

**What happens:**
- ✅ Creates local folder if it doesn't exist
- ✅ Downloads all files from the bucket
- ✅ Sets up sync metadata automatically
- ✅ Ready for immediate use

### 3. Check Sync Status
See what files need to be synchronized:

```bash
# Check status of current folder
rusty-sync status

# Check status of specific folder
rusty-sync status ./my-local-docs
```

**Output:**
```
📊 Sync status for folder: ./my-local-docs
📁 Total files: 15
✅ All files are synchronized

# OR if changes are detected:
📤 Files that need syncing: 3
  📄 document.pdf
  📄 presentation.pptx
  📄 notes.txt
```

### 4. Make Changes and Push
After editing files locally:

```bash
# Check what changed
rusty-sync status

# Upload changes to bucket
rusty-sync push my-documents

# Or push from any directory
rusty-sync push my-documents ./path/to/folder
```

### 5. Pull Remote Changes
Download changes made by other users or devices:

```bash
# Pull changes to current folder
rusty-sync pull my-documents

# Pull changes to specific folder
rusty-sync pull my-documents ./my-local-docs
```

## 📂 Complete Workflow Example

```bash
# 1. See what buckets are available
rusty-sync remote

# 2. Clone the project-files bucket
rusty-sync clone project-files ./my-project
cd my-project

# 3. Make some changes
echo "New feature ideas" >> ideas.txt
mkdir new-feature
echo "print('Hello World')" > new-feature/main.py

# 4. Check what needs syncing
rusty-sync status
# Output: 📤 Files that need syncing: 2
#   📄 ideas.txt
#   📄 new-feature/main.py

# 5. Push your changes
rusty-sync push project-files

# 6. Later, pull any remote changes
rusty-sync pull project-files

# 7. Check everything is synced
rusty-sync status
# Output: ✅ All files are synchronized
```

## 🛠️ Classic Command Workflow

For scripting or when you prefer explicit commands:

### Initialize a Folder
```bash
# Initialize current folder
rusty-sync -i .

# Initialize specific folder
rusty-sync -i /path/to/my/folder
rusty-sync --init ./documents
```

### Sync to Bucket
```bash
# Sync folder to bucket
rusty-sync -s ./documents my-bucket
rusty-sync --sync /home/user/photos photos-bucket
```

### Run Tests
```bash
# Test sync functionality
rusty-sync -t ./test-folder test-bucket
```

## ⚙️ Advanced Usage

### Working from Different Directories

Rusty Sync automatically finds the project root, so you can run commands from anywhere:

```bash
# These all work the same way:
cd /any/directory
rusty-sync push my-bucket ./path/to/sync/folder

cd ./path/to/sync/folder
rusty-sync push my-bucket

rusty-sync push my-bucket .
```

### Cross-Platform Compatibility

Rusty Sync uses forward slashes for all paths internally, making it compatible across Windows, Linux, and macOS:

- ✅ Automatic path conversion
- ✅ Relative path storage in metadata
- ✅ Machine ID tracking for conflict resolution

### File Change Detection

The system intelligently detects changes using:
- **File modification timestamps**
- **File size changes**
- **Last sync timestamps**
- **Machine identification**

Only files that have actually changed are uploaded, making syncs fast and efficient.

## 🔧 Troubleshooting

### Connection Issues
```bash
# Check if MinIO server is accessible
rusty-sync remote
```
If this fails, verify your `.env` file and MinIO server status.

### Permission Errors
Ensure the MinIO access key has read/write permissions for the buckets you're trying to access.

### Missing Files After Clone
If files seem missing after clone:
1. Check the bucket contents: `rusty-sync remote` then list files manually
2. Verify the download directory permissions
3. Run `rusty-sync status` to see the current state

### Working Directory Issues
If commands fail with "file not found" errors, ensure you're running from a location where Rusty Sync can find the project root (looks for `Cargo.toml`).

## 📊 File Structure

When you initialize or clone a folder, Rusty Sync creates:

```
my-sync-folder/
├── your-files...
├── rusty-sync-structure.json    # Sync metadata (don't edit manually)
└── EXAMPLE.struct_git.json      # Git integration metadata
```

- **`rusty-sync-structure.json`**: Contains file metadata, sync timestamps, and machine IDs
- **`EXAMPLE.struct_git.json`**: Git repository information (if applicable)

## 🎯 Best Practices

### 1. Regular Status Checks
```bash
# Before making major changes
rusty-sync status

# After editing files
rusty-sync status
rusty-sync push bucket-name
```

### 2. Pull Before Editing
```bash
# Start your work session
rusty-sync pull my-bucket
# ... edit files ...
rusty-sync push my-bucket
```

### 3. Use Descriptive Bucket Names
- ✅ `project-documents`
- ✅ `family-photos-2024`
- ✅ `work-presentations`
- ❌ `bucket1`, `stuff`, `files`

### 4. Regular Syncing
```bash
# Create aliases for common operations
alias sync-docs='rusty-sync push my-documents'
alias pull-docs='rusty-sync pull my-documents'
alias check-docs='rusty-sync status'
```

## 🚨 Important Notes

### Security
- Store your `.env` file securely
- Don't commit MinIO credentials to version control
- Use appropriate MinIO access policies

### Performance
- Large files (>100MB) may take time to upload/download
- The system is optimized for incremental changes
- Only modified files are transferred

### Limitations
- Binary files are supported but not diffed
- No built-in conflict resolution UI (uses machine IDs for tracking)
- Requires network connectivity for all operations

## 🆘 Get Help

```bash
# Show all available commands
rusty-sync --help

# Command-specific help
rusty-sync -h
```

For issues or questions, check the project repository or create an issue.

---

**Happy Syncing!** 🎉

*Rusty Sync - Making file synchronization as easy as Git!*
