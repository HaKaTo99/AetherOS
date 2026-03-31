@echo off
REM Simple test script - echo commands to QEMU
REM Usage: Run this in a separate terminal while QEMU is running

echo Testing AetherOS Shell Commands
echo =================================
echo.
echo Commands will be sent to QEMU...
echo.

echo h | "C:\Program Files\qemu\qemu-system-x86_64.exe" -kernel "D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel" -m 512 -nographic

pause
