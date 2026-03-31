@echo off
cd /d D:\GitHub\AetherOS
echo Starting QEMU test...
echo.
echo After boot, type: help
echo Then: calc
echo Then: clear
echo Then: exit
echo.
"C:\Program Files\qemu\qemu-system-x86_64.exe" -cdrom out\aetheros.iso -m 1024M -nographic
