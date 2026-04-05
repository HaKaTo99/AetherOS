//! Sovereign Terminal Buffer (SDE-v3)
//! Manages global console history for visual window rendering

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

const MAX_TERMINAL_LINES: usize = 50;

pub struct TerminalLog {
    lines: Vec<String>,
}

impl TerminalLog {
    pub const fn new() -> Self {
        Self {
            lines: Vec::new(),
        }
    }

    pub fn push(&mut self, s: &str) {
        // [v10.3 SUPREME] Handle multi-line strings and newlines
        for line in s.split('\n') {
            let line = line.trim_end_matches('\r');
            if line.is_empty() && s.len() > 1 { continue; }
            
            if self.lines.len() >= MAX_TERMINAL_LINES {
                self.lines.remove(0);
            }
            self.lines.push(String::from(line));
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

// Global Terminal Log Buffer
pub static TERMINAL_LOG: Mutex<TerminalLog> = Mutex::new(TerminalLog::new());

/// Public Hook for Console Redirection
pub fn log_to_terminal(s: &str) {
    let mut log = TERMINAL_LOG.lock();
    log.push(s);
}
