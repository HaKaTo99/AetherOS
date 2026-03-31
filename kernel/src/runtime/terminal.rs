//! Native Terminal Tools Runtime (Phase 16.2)
//! Implements PTY (Pseudo-Terminal) support and basic shell utilities.

use alloc::vec::Vec;

/// Pseudo-Terminal (PTY) State
pub struct PseudoTerminal {
    pub id: u32,
    pub buffer: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl PseudoTerminal {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            buffer: Vec::new(),
            width: 80,
            height: 24,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        // Simulate echo back to "screen"
        if let Ok(s) = core::str::from_utf8(data) {
            crate::println!("[PTY-{}] {}", self.id, s);
        }
    }

    pub fn send_signal(&self, signal: &str) {
        crate::println!("[PTY-{}] Signal Received: {}", self.id, signal);
    }
}

/// Terminal Manager
pub struct TerminalRuntime {
    active_pty: Option<PseudoTerminal>,
}

impl TerminalRuntime {
    pub const fn new() -> Self {
        Self { active_pty: None }
    }

    pub fn open_terminal(&mut self) -> u32 {
        let pty = PseudoTerminal::new(1);
        let id = pty.id;
        self.active_pty = Some(pty);
        crate::println!("[Terminal] New PTY allocated: pts/{}", id);
        id
    }

    pub fn run_command(&mut self, cmd: &str) {
        if let Some(pty) = &mut self.active_pty {
             pty.write(format!("$ {}\n", cmd).as_bytes());
             match cmd {
                 "vim" => crate::println!("[Terminal] Starting Vim 9.0... (Simulated)"),
                 "nano" => crate::println!("[Terminal] Starting GNU nano 6.2... (Simulated)"),
                 _ => crate::println!("[Terminal] Command not found: {}", cmd),
             }
        }
    }
}
