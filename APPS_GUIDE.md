# AetherOS Applications Guide
# Cara Menjalankan Aplikasi di AetherOS

## Cara Akses AetherShell

Setelah AetherOS boot, ketik perintah di AetherShell prompt:

```
AetherShell> help
```

## Daftar Aplikasi yang Tersedia

### 1. Kalkulator (Calculator)
```
bash
AetherShell> calc
```
*Catatan: Untuk saat ini kalkulator dalam mode demo. Untuk perhitungan, gunakan OmniLang:*

```
bash
AetherShell> omni 2+2
AetherShell> omni 10*5
AetherShell> omni 100/4
```

### 2. OmniLang (Bahasa Pemrograman AetherOS)
```
bash
AetherShell> omni [kode]
```
Contoh:
```
bash
AetherShell> omni println("Hello AetherOS")
AetherShell> omni 5+3*2
AetherShell> omni if true { println("Works!") }
```

### 3. Android App Bridge
```
bash
# List aplikasi
AetherShell> apk --list

# Install APK
AetherShell> apk --install com.example.app

# Jalankan APK
AetherShell> apk --run com.example.app
```

### 4. Linux/POSIX Environment
```
bash
# Buka shell Linux
AetherShell> linux --shell

# Jalankan binary
AetherShell> linux --run /bin/ls
```

### 5. Python
```
bash
AetherShell> python script.py
```

### 6. Node.js
```
bash
AetherShell> node app.js
```

### 7. Java
```
bash
AetherShell> java MyClass
```

### 8. PHP
```
bash
AetherShell> php app.php
```

### 9. Blender (Rendering)
```
bash
AetherShell> blender scene.blend
```

### 10. Media Player (VLC)
```
bash
AetherShell> vlc video.mp4
```

## Aplikasi Lainnya

### Identity Management (SSI)
```
bash
AetherShell> identity --create Herman
```

### AI Evolution
```
bash
AetherShell> evolve
```

### Tactical Mesh
```
bash
AetherShell> tactical --stealth
AetherShell> tactical --flash "message"
```

### OneMind (Collective Intelligence)
```bash
AetherShell> onemind --sync
```

### Neural Link (BCI)
```
bash
AetherShell> bci --sync
```

### Sectoral AI
```
bash
AetherShell> intent --sector industrial
AetherShell> intent --sector medical
AetherShell> intent --sector military
```

## Contoh Penggunaan Kalkulator via OmniLang

Karena fitur `calc` masih dalam pengembangan, gunakan `omni` untuk kalkulasi:

```
bash
AetherShell> omni 2+2
Output: 4

AetherShell> omni 10-5
Output: 5

AetherShell> omni 3*4
Output: 12

AetherShell> omni 100/25
Output: 4

AetherShell> omni (10+5)*2
Output: 30
```

## Cara Keluar

```
bash
AetherShell> exit
```

## Reset Layar

```
bash
AetherShell> clear
