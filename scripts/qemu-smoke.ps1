#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSCommandPath
$targetDir = Join-Path $root "..\target"
$logPathCanonical = Join-Path $targetDir "qemu-smoke.log"
$logPathRun = Join-Path $targetDir ("qemu-smoke-{0:yyyyMMddHHmmssfff}.log" -f (Get-Date))
$bootBin = Join-Path $targetDir "x86_64-unknown-none\release\aetheros-kernel"
$memMb = if ($env:MEM_MB) { $env:MEM_MB } else { 1024 }
$smp = if ($env:SMP_CORES) { $env:SMP_CORES } else { 2 }
$cpuModel = if ($env:CPU_MODEL) { $env:CPU_MODEL } else { "qemu64" }
$timeoutSeconds = if ($env:TIMEOUT_SECONDS) { [int]$env:TIMEOUT_SECONDS } else { 90 }
$bootMarker = if ($env:BOOT_MARKER) { $env:BOOT_MARKER } else { "AetherShell>" }
$requiredMarkers = @(
  "HAL Initialized"
  "GDT/IDT Initialized"
  "Cognitive Listener"
  "Harmony Baseline"
  "Identity Mesh"
  "Entropy Chain"
  "Readiness: 100%"
)
if ($env:REQUIRED_MARKERS) {
  $requiredMarkers = $env:REQUIRED_MARKERS -split "\r?\n" | Where-Object { $_ -ne "" }
}

New-Item -ItemType Directory -Force -Path (Split-Path $logPathCanonical) | Out-Null

Write-Host "[setup] rust target nightly + x86_64-unknown-none"
& rustup toolchain install nightly --component rust-src | Out-Null
& rustup target add x86_64-unknown-none --toolchain nightly | Out-Null

Write-Host "[build] cargo +nightly -Z bindeps build --release --manifest-path "$((Join-Path $root "..\kernel\Cargo.toml"))""
$env:CARGO_TARGET_DIR = $targetDir
$env:CARGO_UNSTABLE_BETA_BINDEPS = 1
& cargo +nightly -Z bindeps build --release --manifest-path (Join-Path $root "..\kernel\Cargo.toml")
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
Remove-Item Env:CARGO_UNSTABLE_BETA_BINDEPS -ErrorAction SilentlyContinue

if (-not (Test-Path $bootBin)) {
  Write-Error "Kernel binary tidak ditemukan di $bootBin"
}

$qemuArgs = @(
  "-kernel", $bootBin,
  "-m", "${memMb}M",
  "-smp", "$smp",
  "-cpu", $cpuModel,
  "-serial", "file:$logPathRun",
  "-display", "none",
  "-no-reboot",
  "-no-shutdown",
  "-nographic"
)

Remove-Item -ErrorAction SilentlyContinue $logPathRun
Remove-Item -ErrorAction SilentlyContinue $logPathCanonical

$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = "qemu-system-x86_64"
$psi.Arguments = [string]::Join(' ', $qemuArgs)
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError = $true
$psi.UseShellExecute = $false
$psi.CreateNoWindow = $true
$psi.WorkingDirectory = (Join-Path $root "..")

Write-Host "[run] qemu-system-x86_64 $($psi.Arguments) (timeout ${timeoutSeconds}s)"
$proc = [System.Diagnostics.Process]::Start($psi)
$waitOk = $proc.WaitForExit($timeoutSeconds * 1000)
if (-not $waitOk) {
  Write-Warning "QEMU timed out setelah ${timeoutSeconds}s"
  try { $proc.Kill() } catch {}
  $exitCode = 124
} else {
  $exitCode = $proc.ExitCode
}

$stdout = $proc.StandardOutput.ReadToEnd()
$stderr = $proc.StandardError.ReadToEnd()

if (Test-Path $logPathRun) {
  $content = Get-Content -Raw -Path $logPathRun
  try { Copy-Item -Force -Path $logPathRun -Destination $logPathCanonical } catch {}
} else {
  $content = ($stdout + $stderr)
  Set-Content -Path $logPathCanonical -Value $content
}

# Lampirkan stderr/stdout ke log untuk diagnosa bila serial gagal
if ($stderr -or $stdout) {
  $append = "`n[stdout/stderr]`n$stdout$stderr"
  $appended = $false
  for ($i = 0; $i -lt 3 -and -not $appended; $i++) {
    try {
      Add-Content -Path $logPathCanonical -Value $append
      $appended = $true
    } catch {
      Start-Sleep -Milliseconds 50
    }
  }
  if ($appended) {
    $content = Get-Content -Raw -Path $logPathCanonical
  } else {
    Write-Warning "Gagal append stdout/stderr ke log (file in use); melanjutkan dengan konten yang ada"
    $content = $content + $append
  }
}

$fail = 0
if ($content -match [regex]::Escape($bootMarker)) {
  Write-Host "[ok] Boot marker '$bootMarker' ditemukan"
} else {
  Write-Host "[fail] Boot marker '$bootMarker' tidak ditemukan"
  $fail = 1
}
foreach ($marker in $requiredMarkers) {
  if ($content -match [regex]::Escape($marker)) {
    Write-Host "[ok] Marker '$marker' ditemukan"
  } else {
    Write-Host "[fail] Marker '$marker' tidak ditemukan"
    $fail = 1
  }
}

if ($exitCode -ne 0 -and $exitCode -ne 124) {
  Write-Host "[warn] QEMU exit code: $exitCode"
  $fail = 1
}

if ($fail -ne 0) {
  Write-Host "------ log tail ------"
  $content -split "\r?\n" | Select-Object -Last 120 | ForEach-Object { $_ }
  Write-Host "----------------------"
  exit 1
}
Write-Host "[ok] Semua marker ditemukan"
exit 0
