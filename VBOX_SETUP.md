# AetherOS VirtualBox Setup Guide

## Opsi 1: Gunakan VDI yang sudah ada

1. Buka VirtualBox (sudah terbuka)
2. Klik **New**
   - Name: `AetherOS`
   - Type: `Linux`
   - Version: `Linux 64-bit`
3. Memory: **2048 MB**
4. Hard disk: **Use an existing virtual hard disk file**
   - Browse ke: `D:\GitHub\AetherOS\vbox_vms\AetherOS.vdi`
5. Klik **Create**
6. Klik **Start** untuk menjalankan

---

## Opsi 2: Boot langsung dari kernel (rekomendasi)

1. Klik **New**
   - Name: `AetherOS`
   - Type: `Linux`  
   - Version: `Linux 64-bit`
2. Memory: **2048 MB**
3. Hard disk: **Do not add a hard disk** (nanti kita buat)
4. Klik **Create**
5. Klik **Settings** pada VM baru:
   - **System** → Boot Order: Uncheck semua, centang **EFI**
   - **Storage** → klik "Empty" → centang "Live CD/DVD"
   - Browse ke file ISO (belum ada - harus buat dulu)
6. Klik **OK**
7. Klik **Start**

---

## Catatan

- VDI existing mungkin tidak cocok dengan kernel versi terbaru
- Kalau mau update kernel, perlu rebuild VDI atau buat ISO baru

## Troubleshooting

**Keyboard tidak bekerja:**
- Pastikan VM window aktif
- Coba klik dalam VM window dulu
- Kalau masih tidak, instal Guest Additions
