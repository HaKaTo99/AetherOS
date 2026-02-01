<#
.SYNOPSIS
    Generate Secure Boot Signing Keys
    Creates X.509 keys for simulating Secure Boot signing.
    
.DESCRIPTION
    This script generates:
    1. Platform Key (PK)
    2. Key Exchange Key (KEK)
    3. Database Key (db) - used for signing Kernel/Bootloader
    
    Output format: PEM (Private Key) and DER/CRT (Public Cert).
    
    NOTE: Requires OpenSSL installed and in PATH.
#>

$KeyDir = "$PSScriptRoot\keys"
$OpenSsl = "openssl"

Write-Host "AetherOS Secure Boot Key Generator" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan

if (-not (Test-Path $KeyDir)) {
    New-Item -ItemType Directory -Path $KeyDir | Out-Null
}

function Generate-Key ($Name, $CN) {
    Write-Host "Generating $Name..." -NoNewline
    
    $PrivKey = "$KeyDir\$Name.key"
    $Cert = "$KeyDir\$Name.crt"
    $Der = "$KeyDir\$Name.der"
    
    # Generate Private Key and Self-Signed Cert
    & $OpenSsl req -new -x509 -newkey rsa:2048 -nodes -keyout $PrivKey -out $Cert -days 365 -subj "/CN=$CN" 2>&1 | Out-Null
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host " [FAILED] (OpenSSL error or not found)" -ForegroundColor Red
        return
    }
    
    # Convert CRT to DER (for UEFI enrollment simulation)
    & $OpenSsl x509 -in $Cert -outform DER -out $Der
    
    Write-Host " [OK]" -ForegroundColor Green
    Write-Host "  -> $PrivKey" -ForegroundColor Gray
    Write-Host "  -> $Cert" -ForegroundColor Gray
}

# Autoscan for common OpenSSL paths
if (-not (Get-Command $OpenSsl -ErrorAction SilentlyContinue)) {
    $PossiblePaths = @(
        "C:\Program Files\Git\usr\bin\openssl.exe",
        "C:\Program Files (x86)\Git\usr\bin\openssl.exe",
        "C:\OpenSSL-Win64\bin\openssl.exe"
    )
    
    foreach ($path in $PossiblePaths) {
        if (Test-Path $path) {
            $OpenSsl = $path
            break
        }
    }
}

if (-not (Get-Command $OpenSsl -ErrorAction SilentlyContinue) -and -not (Test-Path $OpenSsl)) {
    Write-Warning "OpenSSL not found. Generating MOCK keys for testing flow."
    
    function Generate-Mock ($Name) {
        Write-Host "Creating Mock $Name..." -NoNewline
        Set-Content -Path "$KeyDir\$Name.key" -Value "MOCK PRIVATE KEY DATA"
        Set-Content -Path "$KeyDir\$Name.crt" -Value "MOCK CERTIFICATE DATA"
        Set-Content -Path "$KeyDir\$Name.der" -Value "MOCK DER DATA"
        Write-Host " [MOCKED]" -ForegroundColor Yellow
    }
    
    Generate-Mock "PK"
    Generate-Mock "KEK"
    Generate-Mock "db"
    
    Write-Host "`nMOCK Keys generated. Install OpenSSL for real keys." -ForegroundColor Yellow
    exit 0
}

Generate-Key "PK" "AetherOS Platform Key"
Generate-Key "KEK" "AetherOS Key Exchange Key"
Generate-Key "db" "AetherOS Driver Signing Key"

Write-Host "`nKeys generated successfully in $KeyDir" -ForegroundColor Green
Write-Host "Use 'db.key' to sign the Kernel."
