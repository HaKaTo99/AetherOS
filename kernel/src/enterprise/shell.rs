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



        platform.puts("--- AetherOS v10.1 Sovereign Shell ---\r\n");
        platform.puts("[AUTHORITY] Welcome, Architect Herman Krisnanto.\r\n");
        platform.puts("Type 'help' for commands.\r\n");

        loop {
            platform.puts("\r\nAetherShell> ");
            let input = read_line(platform);
            let cmd = input.trim();

            if cmd == "exit" {
                break;
            } else if cmd == "help" {
                platform.puts("\r\nAvailable Commands:\r\n");
                platform.puts("  omni [code]   : Compile and run OmniLang code\r\n");
                platform.puts("  blender [file]: Start headless render job (.blend)\r\n");
                platform.puts("  vlc [file]    : Play multimedia resource\r\n");
                platform.puts("  calc          : Run simple calculator demo\r\n");
                platform.puts("  clear         : Clear screen\r\n");
                platform.puts("  exit          : Shutdown shell\r\n");
            } else if cmd.starts_with("omni ") {
                let source = &cmd[5..];
                platform.puts("\r\n[OmniBridge] Compiling...\r\n");
                if let Ok(output) = crate::runtime::omnilang_bridge::OmniBridge::compile_and_run(source) {
                    platform.puts("[Output] ");
                    platform.puts(&output);
                    platform.puts("\r\n");
                }
            } else if cmd.starts_with("blender ") {
                let filename = cmd[8..].trim();
                platform.puts("\r\n[Blender] Initializing Compute Node...\r\n");
                let mut node = crate::runtime::media::blender::BlenderComputeNode::new();
                match node.start_render(filename) {
                    Ok(res) => {
                         platform.puts(&res);
                         platform.puts("\r\n");
                    },
                    Err(e) => {
                         platform.puts("Error: ");
                         platform.puts(e);
                         platform.puts("\r\n");
                    }
                }
            } else if cmd.starts_with("vlc ") {
                let filename = cmd[4..].trim();
                platform.puts("\r\n[VLC] Initializing Universal Media Player...\r\n");
                if let Ok(mut player) = crate::runtime::MediaRuntime::new(filename) {
                    let _ = player.play();
                } else {
                    platform.puts("Error: Failed to load Media Runtime.\r\n");
                }
            } else if cmd == "calc" {
                 // Calculator Logic
                 platform.puts("\r\n[Calculator] Mode Active (Press Ctrl+C to exit - simulation)\r\n");
                 // ... (Simplified calculator logic could go here or be a separate function)
                 platform.puts("Calculator demo skipped for shell responsiveness.\r\n");
            } else if cmd == "clear" {
                platform.clear();
            } else if !cmd.is_empty() {
                platform.puts("\r\nUnknown command. Type 'help'.\r\n");
            }
        }

        platform.puts("\r\nShutting down...\r\n");
        platform.shutdown();
    }
}

// Helper: baca satu baris dari serial (blocking), echo, handle backspace.
fn read_line(platform: &dyn hal::Platform) -> alloc::string::String {
    let mut buf = alloc::string::String::new();
    loop {
        let c = platform.get_char(); // blocking sampai ada data
        // filter noise
        if c == 0 || c == 0xFF { continue; }

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
