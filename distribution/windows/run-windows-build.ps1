# run-windows-build.ps1 - Launcher script to run Windows build from project root
param(
    [switch]$Release = $false,
    [switch]$Setup = $false,
    [switch]$Help = $false
)

if ($Help) {
    Write-Host "RustySync Windows Build Launcher" -ForegroundColor Green
    Write-Host "Usage: .\run-windows-build.ps1 [-Release] [-Setup] [-Help]"
    Write-Host ""
    Write-Host "  -Release: Build in release mode (optimized)"
    Write-Host "  -Setup:   Run complete setup including prerequisites"
    Write-Host "  -Help:    Show this help message"
    Write-Host ""
    Write-Host "This script runs the Windows build tools from the project root."
    exit 0
}

# Verify we're in the project root
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Error: Run this script from the rusty-sync project root directory"
    exit 1
}

if ($Setup) {
    Write-Host "Running complete Windows setup..." -ForegroundColor Green
    & "distribution\windows\setup-windows.ps1"
} else {
    Write-Host "Running Windows build..." -ForegroundColor Green
    if ($Release) {
        & "distribution\windows\build-windows.ps1" -Release
    } else {
        & "distribution\windows\build-windows.ps1"
    }
}
