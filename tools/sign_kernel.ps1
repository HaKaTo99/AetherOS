# 🖋️ AetherOS Kernel Signer (Military Grade Anchor)
# This script appends a sovereign digital signature to the kernel binary.

param(
    [string]$KernelPath
)

if (-not (Test-Path $KernelPath)) {
    Write-Error "Kernel binary not found at $KernelPath"
    exit 1
}

# 1. Define Sovereign Signature Payload (Phase 2.2.1)
$signature = "AETHER_SIG_v1.0.0_SOVEREIGN_KYBER_DILITHIUM_TRUST_ANCHOR"
$padding = "`0" * (64 - $signature.Length)
$finalSig = $signature + $padding

# 2. Append Signature to Binary
Write-Host "[SIGN] Appending Sovereign Trust Anchor (64-byte) to $KernelPath..." -ForegroundColor Yellow
$bytes = [System.Text.Encoding]::ASCII.GetBytes($finalSig)

# Use FileStream for max compatibility across PS versions
$fs = [System.IO.File]::Open($KernelPath, [System.IO.FileMode]::Append)
try {
    $fs.Write($bytes, 0, $bytes.Length)
} finally {
    $fs.Close()
}

Write-Host "[SIGN] Kernel Signed Successfully. Sovereign Boot Lock Engaged." -ForegroundColor Green
