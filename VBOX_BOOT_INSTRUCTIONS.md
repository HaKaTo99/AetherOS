# Cara Boot Kernel AetherOS di VirtualBox

## Masalah:
- VM saat ini: Memory 64MB (terlalu kecil)
- VDI lama mungkin kosong/tidak ada OS

## Solusi: Buat VM baru yang boot dari kernel

### Langkah di VirtualBox GUI:

1. **Hentikan VM dulu** (jika masih running)
   - Klik X → Power Off

2. **Hapus VM lama**
   - Klik kanan pada "AetherOS-VDI" → Remove → Delete all files

3. **Buat VM baru**
   - Klik **New**
   - Name: `AetherOS-Kernel`
   - Type: `Linux`
   - Version: `Linux 64-bit`
   - Memory: **2048 MB** (2GB)
   - Create

4. **Atur Settings VM baru:**
   - Klik **Settings**
   - **System** → 
     - Boot Order:uncheck semua, centang **EFI** (atau BIOS juga bisa)
     - Processor: 2 CPUs
   - **Storage** →
     - Klik "Empty" → centang "Live CD/DVD"
     - Browsedan pilih file ISO (belum ada - harus buat dulu)
     - ATAU langsung boot dari kernel file

5. **Boot dari kernel langsung:**
   - QEMU bisa boot langsung dari kernel file, VirtualBox tidak semudah itu
   - Perlu buat ISO dulu

---

## Alternatif: Pakai QEMU dengan perbaikan keyboard

Coba install WSL2 + QEMU di Linux:

```
bash
# Di WSL Ubuntu
sudo apt install qemu-system-x86
qemu-system-x86_64 -kernel /mnt/d/GitHub/AetherOS/target/x86_64-unknown-none/release/aetheros-kernel -m 2048 -display gtk
```

Ini akan kasih GUI window dengan keyboard support penuh.
