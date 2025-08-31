# RustySync

I hate onedrive. A cross-platform file synchronization tool built in Rust with MinIO backend support.

## Features

- Cross-platform compatibility (Windows, Linux, macOS)
- MinIO object storage integration
- Git repository detection and handling
- Intelligent file change detection
- CLI interface for easy automation

## Installation

### Windows
1. Download the latest `RustySync-Setup-x.x.x.exe` from the releases page
2. Run the installer and follow the setup wizard
3. Optionally add RustySync to your PATH during installation
4. Open Command Prompt and run `rusty-sync --help` to verify installation

### Linux (Debian/Ubuntu)
```bash
# Download the .deb package from releases
sudo dpkg -i rusty-sync.deb

# Or build from source
git clone https://github.com/TegranGrigorian/rusty-sync
cd rusty-sync
cargo build --release
```

## Prerequisites

- **Python 3.8+**: Required for MinIO operations
- **Python packages**: Will be installed automatically on Linux, manually on Windows

### Windows Python Setup
After installing RustySync on Windows:
```cmd
cd "C:\Program Files\RustySync\python"
pip install -r requirements.txt
```

## Quick Start

1. **Configure MinIO server**:
   ```bash
   rusty-sync config
   ```

2. **Initialize a folder for sync**:
   ```bash
   rusty-sync init /path/to/folder
   ```

3. **Push files to MinIO**:
   ```bash
   rusty-sync push my-bucket /path/to/folder
   ```

4. **Pull files from MinIO**:
   ```bash
   rusty-sync pull my-bucket /path/to/folder
   ```

## Configuration

RustySync stores configuration in:
- **Windows**: `%USERPROFILE%\.rusty-sync\config.json`
- **Linux/macOS**: `~/.rusty-sync/config.json`

## Building from Source

### Windows
```powershell
# Install Rust with MSVC toolchain
rustup target add x86_64-pc-windows-msvc

# Option 1: Use launcher script (recommended)
.\run-windows-build.ps1 -Release

# Option 2: Use build script directly
.\distribution\windows\build-windows.ps1 -Release

# Create installer (requires Inno Setup)
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" "distribution\windows\installer.iss"
```

### Linux
```bash
# Build Debian package
./build-deb.sh

# Or just build binary
cargo build --release
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

- [Issues](https://github.com/TegranGrigorian/rusty-sync/issues)
- [Documentation](docs/)
- [Releases](https://github.com/TegranGrigorian/rusty-sync/releases)
