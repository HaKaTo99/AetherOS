@echo off
REM AetherOS x86_64 QEMU Launcher
REM Run this from Command Prompt (cmd.exe)

set KERNEL=D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel

echo === AetherOS x86_64 QEMU Launcher ===
echo Starting QEMU with kernel: %KERNEL%

REM Run QEMU with USB keyboard support for better input handling on Windows
qemu-system-x86_64 -kernel %KERNEL% -m 1024M -display none -serial stdio -device qemu-xhci -device usb-kbd
