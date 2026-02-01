# Android Bootloader Unlock Guide for AetherOS

## 1. Google Pixel Devices
1. Enable **Developer Options**: Settings -> About Phone -> Tap Build Number 7 times.
2. Enable **OEM Unlocking**: Settings -> System -> Developer Options -> Toggle "OEM Unlocking".
3. Enter Fastboot Mode:
   ```bash
   adb reboot bootloader
   ```
4. Unlock Command:
   ```bash
   fastboot flashing unlock
   ```
5. Confirm on device screen using Volume keys.

## 2. OnePlus Devices
1. Enable **OEM Unlocking** in Developer Options.
2. Reboot to Fastboot.
3. Unlock Command:
   ```bash
   fastboot oem unlock
   ```

## 3. Verify Unlock
Run `fastboot getvar unlocked`. It should return `yes`.
