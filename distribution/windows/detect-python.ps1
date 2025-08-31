# detect-python.ps1 - Python detection and PATH fixing script

Write-Host "=== Python Detection Tool ===" -ForegroundColor Green

# Test different Python commands
$pythonCommands = @("py", "python", "python3", "python.exe")
$workingPython = $null

foreach ($cmd in $pythonCommands) {
    try {
        $version = & $cmd --version 2>$null
        if ($version) {
            Write-Host "✓ Found: $cmd -> $version" -ForegroundColor Green
            if (-not $workingPython) { $workingPython = $cmd }
        }
    } catch {
        Write-Host "✗ Not found: $cmd" -ForegroundColor Red
    }
}

if ($workingPython) {
    Write-Host "`nRecommended Python command: $workingPython" -ForegroundColor Yellow
} else {
    Write-Host "`nNo Python installation found!" -ForegroundColor Red
}

# Check Python installations
Write-Host "`n=== Python Installation Locations ===" -ForegroundColor Cyan

# Check common locations
$commonPaths = @(
    "$env:LOCALAPPDATA\Programs\Python",
    "$env:PROGRAMFILES\Python*",
    "$env:PROGRAMFILES(X86)\Python*",
    "$env:APPDATA\Local\Microsoft\WindowsApps\python.exe"
)

foreach ($path in $commonPaths) {
    if (Test-Path $path) {
        Write-Host "Found: $path" -ForegroundColor Green
        if ($path -like "*.exe") {
            try {
                $version = & $path --version 2>$null
                Write-Host "  Version: $version" -ForegroundColor Gray
            } catch {}
        }
    }
}

# Check Windows Store Python
$storePython = Get-AppxPackage -Name "*Python*" 2>$null
if ($storePython) {
    Write-Host "`nWindows Store Python found:" -ForegroundColor Yellow
    $storePython | ForEach-Object { Write-Host "  $($_.Name) - $($_.Version)" -ForegroundColor Gray }
}

# Check PATH
Write-Host "`n=== PATH Analysis ===" -ForegroundColor Cyan
$pathEntries = $env:PATH -split ';' | Where-Object { $_ -like "*python*" -or $_ -like "*Python*" }
if ($pathEntries) {
    Write-Host "Python-related PATH entries:"
    $pathEntries | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
} else {
    Write-Host "No Python entries found in PATH" -ForegroundColor Yellow
}

# Provide fix suggestions
Write-Host "`n=== Recommendations ===" -ForegroundColor Green

if ($workingPython) {
    Write-Host "1. Use '$workingPython' in your scripts instead of 'python'"
    Write-Host "2. Update your scripts to detect Python automatically"
} else {
    Write-Host "1. Install Python from: https://python.org/downloads/"
    Write-Host "2. Or install via winget: winget install Python.Python.3.11"
    Write-Host "3. Make sure to check 'Add to PATH' during installation"
}

Write-Host "`nTo fix PATH manually:"
Write-Host "1. Open System Properties -> Environment Variables"
Write-Host "2. Add Python installation directory to PATH"
Write-Host "3. Restart PowerShell"
