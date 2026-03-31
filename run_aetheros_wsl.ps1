# AetherOS QEMU Launcher via WSL PowerShell
# Run this in PowerShell

Write-Host "=== AetherOS x86_64 QEMU (WSL) ===" -ForegroundColor Cyan

# Run QEMU via WSL
wsl -d Ubuntu -e bash -c "qemu-system-x86_64 -kernel /mnt/d/GitHub/AetherOS/target/x86_64-unknown-none/release/aetheros-kernel -m 1024M -nographic -serial mon:stdio"
