# setup-windows.ps1 - Complete Windows setup script for RustySync
param(
    [switch]$SkipPrerequisites = $false,
    [switch]$Help = $false
)

if ($Help) {
    Write-Host "Complete Windows setup script for RustySync"
    Write-Host "Usage: .\setup-windows.ps1 [-SkipPrerequisites] [-Help]"
    Write-Host "  -SkipPrerequisites: Skip installation of Rust, Python, etc."
    Write-Host "  -Help: Show this help message"
    exit 0
}

Write-Host "=== RustySync Windows Setup ===" -ForegroundColor Green

# Phase 1: Prerequisites
if (-not $SkipPrerequisites) {
    Write-Host "`n1. Installing Prerequisites..." -ForegroundColor Cyan
    
    Write-Host "Installing Rust..."
    winget install Rustlang.Rust --silent
    
    Write-Host "Installing Visual Studio Build Tools..."
    winget install Microsoft.VisualStudio.2022.BuildTools --silent
    
    Write-Host "Installing Python..."
    winget install Python.Python.3.11 --silent
    
    Write-Host "Installing Inno Setup..."
    winget install JRSoftware.InnoSetup --silent
    
    Write-Host "Please restart PowerShell and run again with -SkipPrerequisites" -ForegroundColor Yellow
    exit 0
}

# Phase 2: Configure Rust
Write-Host "`n2. Configuring Rust..." -ForegroundColor Cyan
rustup target add x86_64-pc-windows-msvc

# Phase 3: Set up Python environment
Write-Host "`n3. Setting up Python environment..." -ForegroundColor Cyan

# Test Python availability
Write-Host "Testing Python installation..."
$pythonCmd = $null
try {
    py --version | Out-Null
    $pythonCmd = "py"
    Write-Host "Using 'py' launcher" -ForegroundColor Green
} catch {
    try {
        python --version | Out-Null
        $pythonCmd = "python"
        Write-Host "Using 'python' command" -ForegroundColor Green
    } catch {
        Write-Error "Python not found! Please install Python or add it to PATH"
        exit 1
    }
}

if (Test-Path "src\core\minio") {
    Push-Location "src\core\minio"
    
    if (-not (Test-Path ".venv")) {
        & $pythonCmd -m venv .venv
    }
    
    .\.venv\Scripts\Activate.ps1
    pip install -r requirements.txt
    deactivate
    
    Pop-Location
    Write-Host "Python environment ready!" -ForegroundColor Green
} else {
    Write-Host "Warning: minio directory not found" -ForegroundColor Yellow
}

# Phase 4: Build application
Write-Host "`n4. Building application..." -ForegroundColor Cyan
.\build-windows.ps1 -Release

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}

# Phase 5: Set up installer Python environment
Write-Host "`n5. Setting up installer Python environment..." -ForegroundColor Cyan
if (Test-Path "installer\python") {
    Push-Location "installer\python"
    
    & $pythonCmd -m venv .
    .\Scripts\Activate.ps1
    pip install -r requirements.txt
    deactivate
    
    Pop-Location
    Write-Host "Installer Python environment ready!" -ForegroundColor Green
}

# Phase 6: Create installer
Write-Host "`n6. Creating installer..." -ForegroundColor Cyan
$InnoSetupPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
if (Test-Path $InnoSetupPath) {
    & $InnoSetupPath "installer.iss"
    
    if (Test-Path "output\RustySync-Setup-0.1.0.exe") {
        Write-Host "`n=== SUCCESS! ===" -ForegroundColor Green
        Write-Host "Installer created: output\RustySync-Setup-0.1.0.exe" -ForegroundColor Green
        Write-Host "File size: $((Get-Item "output\RustySync-Setup-0.1.0.exe").Length / 1MB) MB" -ForegroundColor Gray
    } else {
        Write-Error "Installer creation failed!"
    }
} else {
    Write-Error "Inno Setup not found at: $InnoSetupPath"
}

Write-Host "`nSetup complete! You can now test the installer." -ForegroundColor Green
