@echo off
REM AetherOS CLI Test Script
REM Usage: test_shell.bat

echo ========================================
echo AetherOS Shell Command Test (CLI)
echo ========================================
echo.

set QEMU="C:\Program Files\qemu\qemu-system-x86_64.exe"
set KERNEL=D:\GitHub\AetherOS\target\x86_64-unknown-none\release\aetheros-kernel

echo Starting QEMU...
echo After boot, type: help
echo Then calc:
echo Then: clear
echoecho.

 Then: exit
%QEMU% -kernel %KERNEL% -m 1024M -nographic

echo.
echo Test complete!
pause
