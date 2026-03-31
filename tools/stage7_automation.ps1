Param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("OMNILANG","OMNILANG_EXECUTE","BLENDER","WIN32_OFFICE","APK_RUNTIME","LINUX","UNIX","WINDOWS","MAC","HARMONY","SYMBIAN","WEBOS")]
    [string]$Component,

    [string]$WslDistro = "Ubuntu"
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoRoot = Resolve-Path (Join-Path $scriptDir "..")
$kernelFile = Join-Path $repoRoot "kernel\src\lib.rs"

if (-not (Test-Path $kernelFile)) {
    Write-Error "kernel file not found: $kernelFile"
    exit 1
}

$timestamp = (Get-Date).ToString('yyyyMMdd-HHmmss')
$backup = "${kernelFile}.bak.$timestamp"
Copy-Item $kernelFile $backup -Force
Write-Host "Backup created: $backup"

$content = Get-Content $kernelFile -Raw

# Normalize all STAGE7_FULL_VERIFY_* flags to false first
$content = [regex]::Replace($content, 'const\s+(STAGE7_FULL_VERIFY_[A-Z0-9_]+)\s*:\s*bool\s*=\s*(true|false)\s*;', 'const $1: bool = false;')

$map = @{
    'OMNILANG' = 'STAGE7_FULL_VERIFY_OMNILANG'
    'OMNILANG_EXECUTE' = 'STAGE7_FULL_VERIFY_OMNILANG_EXECUTE'
    'BLENDER' = 'STAGE7_FULL_VERIFY_BLENDER'
    'WIN32_OFFICE' = 'STAGE7_FULL_VERIFY_WIN32_OFFICE'
    'APK_RUNTIME' = 'STAGE7_FULL_VERIFY_APK_RUNTIME'
    'LINUX' = 'STAGE7_FULL_VERIFY_LINUX'
    'UNIX' = 'STAGE7_FULL_VERIFY_UNIX'
    'WINDOWS' = 'STAGE7_FULL_VERIFY_WINDOWS'
    'MAC' = 'STAGE7_FULL_VERIFY_MAC'
    'HARMONY' = 'STAGE7_FULL_VERIFY_HARMONY'
    'SYMBIAN' = 'STAGE7_FULL_VERIFY_SYMBIAN'
    'WEBOS' = 'STAGE7_FULL_VERIFY_WEBOS'
}

$flag = $map[$Component]
if (-not $flag) {
    Write-Error "Unknown component: $Component"
    exit 1
}

# Set the selected flag to true (for all occurrences)
$pattern = "const\s+($flag)\s*:\s*bool\s*=\s*(true|false)\s*;"
$replacement = 'const ' + $flag + ': bool = true;'
$content = [regex]::Replace($content, $pattern, [string]$replacement)

Set-Content $kernelFile $content -Encoding UTF8
Write-Host "Set $flag = true in $kernelFile"

# Invoke the existing rebuild wrapper
$rebuild = Join-Path $repoRoot "tools\rebuild_vm_iso.ps1"
if (-not (Test-Path $rebuild)) {
    Write-Host "Rebuild script not found at $rebuild. Skipping ISO build."
    exit 0
}

Write-Host "Running rebuild script (WslDistro=$WslDistro)..."
& $rebuild -WslDistro $WslDistro

Write-Host "Automation complete. Created backup: $backup"
