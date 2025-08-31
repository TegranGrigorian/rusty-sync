# build-windows.ps1 - Windows build script for rusty-sync
param(
    [switch]$Release = $false,
    [switch]$Help = $false
)

if ($Help) {
    Write-Host "Usage: .\build-windows.ps1 [-Release] [-Help]"
    Write-Host "  -Release: Build in release mode (optimized)"
    Write-Host "  -Help:    Show this help message"
    exit 0
}

Write-Host "Building rusty-sync for Windows..." -ForegroundColor Green

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Error: Run this script from the rusty-sync project root directory"
    exit 1
}

# Determine build mode
$BuildMode = if ($Release) { "release" } else { "debug" }
$BuildFlag = if ($Release) { "--release" } else { "" }

Write-Host "Build mode: $BuildMode" -ForegroundColor Yellow

# Step 1: Build Rust binary
Write-Host "Step 1: Building Rust binary..." -ForegroundColor Cyan
if ($Release) {
    cargo build --release --target x86_64-pc-windows-msvc
} else {
    cargo build --target x86_64-pc-windows-msvc
}

if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed"
    exit 1
}

# Check if binary was created
$BinaryPath = "target\x86_64-pc-windows-msvc\$BuildMode\rusty-sync.exe"
if (-not (Test-Path $BinaryPath)) {
    Write-Error "Error: Binary not found at $BinaryPath"
    exit 1
}

# Step 2: Prepare installer directory structure
Write-Host "Step 2: Preparing installer directory structure..." -ForegroundColor Cyan
$InstallerDir = ".\installer"
$BinDir = "$InstallerDir\bin"
$PythonDir = "$InstallerDir\python"
$PythonSrcDir = "$PythonDir\src"

# Create directories
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $PythonDir | Out-Null
New-Item -ItemType Directory -Force -Path $PythonSrcDir | Out-Null

# Step 3: Copy files
Write-Host "Step 3: Copying files..." -ForegroundColor Cyan

# Copy main binary
Copy-Item $BinaryPath "$BinDir\rusty-sync.exe" -Force
Write-Host "  - Copied rusty-sync.exe"

# Copy Python scripts
if (Test-Path "src\core\minio\main.py") {
    Copy-Item "src\core\minio\main.py" "$PythonDir\" -Force
    Write-Host "  - Copied main.py"
}

if (Test-Path "src\core\minio\test.py") {
    Copy-Item "src\core\minio\test.py" "$PythonDir\" -Force
    Write-Host "  - Copied test.py"
}

if (Test-Path "src\core\minio\src\minio_util.py") {
    Copy-Item "src\core\minio\src\minio_util.py" "$PythonSrcDir\" -Force
    Write-Host "  - Copied minio_util.py"
}

# Copy requirements.txt if it exists
if (Test-Path "src\core\minio\requirements.txt") {
    Copy-Item "src\core\minio\requirements.txt" "$PythonDir\" -Force
    Write-Host "  - Copied requirements.txt"
}

# Test Python availability
Write-Host ""
Write-Host "Testing Python installation..." -ForegroundColor Yellow
try {
    $pythonVersion = py --version 2>$null
    if ($pythonVersion) {
        Write-Host "  - Python found: $pythonVersion" -ForegroundColor Green
    } else {
        Write-Host "  - Warning: 'py' launcher not found, trying 'python'..." -ForegroundColor Yellow
        $pythonVersion = python --version 2>$null
        if ($pythonVersion) {
            Write-Host "  - Python found: $pythonVersion" -ForegroundColor Green
        } else {
            Write-Host "  - Error: Python not found in PATH" -ForegroundColor Red
            Write-Host "  - Please install Python or add it to PATH" -ForegroundColor Red
        }
    }
} catch {
    Write-Host "  - Error testing Python: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host "Files prepared for installer in: $InstallerDir" -ForegroundColor Yellow
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "1. Install Python dependencies in the installer directory"
Write-Host "2. Run Inno Setup with installer.iss to create the Windows installer"
Write-Host ""
Write-Host "Binary size: $((Get-Item $BinaryPath).Length / 1MB) MB" -ForegroundColor Gray
