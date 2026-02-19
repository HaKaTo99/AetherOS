//! Aether Interactive Shell (Phase 38.2)
//!
//! Encapsulates the boot-time login and command handling logic.

use crate::hal;

pub struct AetherShell;

impl AetherShell {
    /// Starts the interactive shell session.
    ///
    /// Versi demo: tidak ada login/password, langsung masuk ke Sovereign Shell.
    /// Perbaikan ekstrem: bypass OmniLang input, pakai pembacaan serial blokir langsung.
    pub fn start() {
        let platform = hal::get_platform();

        // Helper: baca satu baris dari serial (blocking), echo, handle backspace.
        fn read_line(platform: &dyn hal::Platform) -> alloc::string::String {
            let mut buf = alloc::string::String::new();
            loop {
                let c = platform.get_char(); // blocking sampai ada data
                // filter noise
                if c == 0 || c == 0xFF {
                    continue;
                }

                // newline: selesai
                if c == b'\r' || c == b'\n' {
                    platform.puts("\r\n");
                    break;
                }

                // backspace
                if c == 8 || c == 127 {
                    if !buf.is_empty() {
                        buf.pop();
                        platform.puts("\x08 \x08");
                    }
                    continue;
                }

                // normal char
                buf.push(c as char);
                platform.put_char(c);
            }
            buf
        }

        platform.puts("--- AetherOS v10.0 Sovereign Shell (Calculator Demo) ---\r\n");
        platform.puts("[AUTHORITY] Welcome, Architect Herman Krisnanto.\r\n");
        platform.puts("Simple Calculator: hanya mendukung angka 0-9 dan operasi +, -, *, /\r\n\r\n");

        platform.puts("Masukkan angka pertama (0-9): ");
        let a_str = read_line(platform);

        platform.puts("Operator (+, -, *, /): ");
        let op_str = read_line(platform);

        platform.puts("Masukkan angka kedua (0-9): ");
        let b_str = read_line(platform);

        // Konversi digit
        let a = match a_str.as_str() {
            "0" => 0, "1" => 1, "2" => 2, "3" => 3, "4" => 4, "5" => 5, "6" => 6, "7" => 7, "8" => 8, "9" => 9,
            _ => -1,
        };
        let b = match b_str.as_str() {
            "0" => 0, "1" => 1, "2" => 2, "3" => 3, "4" => 4, "5" => 5, "6" => 6, "7" => 7, "8" => 8, "9" => 9,
            _ => -1,
        };

        let mut valid = true;
        if a < 0 || b < 0 {
            valid = false;
            platform.puts("\r\nError: hanya menerima digit 0-9.\r\n");
        }

        let mut result = 0;
        if valid {
            match op_str.as_str() {
                "+" => result = a + b,
                "-" => result = a - b,
                "*" => result = a * b,
                "/" => {
                    if b == 0 {
                        valid = false;
                        platform.puts("\r\nError: pembagian dengan nol tidak diizinkan.\r\n");
                    } else {
                        result = a / b;
                    }
                }
                _ => {
                    valid = false;
                    platform.puts("\r\nError: operator tidak dikenal. Gunakan +, -, *, /.\r\n");
                }
            }
        }

        if valid {
            platform.puts("\r\nHasil: ");
            platform.puts(&alloc::format!("{}", result));
            platform.puts("\r\n");
        }

        platform.puts("\r\nTekan ENTER untuk mematikan demo kalkulator...\r\n");
        let _ = read_line(platform);
        platform.shutdown();
    }
}
