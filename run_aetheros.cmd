@echo off
echo ================================================
echo AetherOS Quick Start (CMD)
echo ================================================
echo.
echo Starting QEMU with AetherOS Kernel...
echo.
echo NOTE: Wait for AetherShell> prompt, then type: help
echo.
"C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 1024M -display gtk
