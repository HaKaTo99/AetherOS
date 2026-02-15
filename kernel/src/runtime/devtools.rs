//! Developer Tools (Phase 14.3)
//! LSP, profiler, CI templates

use alloc::string::String;
use alloc::vec::Vec;

// ===========================
// Language Server Protocol (LSP) for AetherScript
// ===========================

/// LSP diagnostic severity
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// LSP diagnostic
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: u32,
    pub col: u32,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/// LSP server stub for AetherScript
pub struct LanguageServer {
    pub documents: Vec<(String, String)>, // (uri, content)
}

impl LanguageServer {
    pub fn new() -> Self {
        Self { documents: Vec::new() }
    }

    /// Handle textDocument/didOpen
    pub fn did_open(&mut self, uri: &str, content: &str) {
        self.documents.push((String::from(uri), String::from(content)));
    }

    /// Run diagnostics
    pub fn diagnose(&self, uri: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if let Some((_, content)) = self.documents.iter().find(|(u, _)| u == uri) {
            // Basic syntax checks
            let mut braces = 0i32;
            for (i, line) in content.lines().enumerate() {
                for ch in line.chars() {
                    match ch {
                        '{' => braces += 1,
                        '}' => braces -= 1,
                        _ => {}
                    }
                }
                if braces < 0 {
                    diagnostics.push(Diagnostic {
                        line: i as u32,
                        col: 0,
                        message: String::from("Unmatched closing brace"),
                        severity: DiagnosticSeverity::Error,
                    });
                }
            }
            if braces != 0 {
                diagnostics.push(Diagnostic {
                    line: 0, col: 0,
                    message: String::from("Mismatched braces"),
                    severity: DiagnosticSeverity::Error,
                });
            }
        }
        diagnostics
    }

    /// Provide completions
    pub fn completions(&self, _uri: &str, _line: u32, _col: u32) -> Vec<String> {
        // AetherScript keyword completions
        vec![
            String::from("fn"), String::from("let"), String::from("if"),
            String::from("else"), String::from("return"), String::from("while"),
            String::from("@memory"), String::from("@distributed"), String::from("@realtime"),
        ]
    }
}

// ===========================
// Profiler
// ===========================

/// Profiling sample
#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub function: String,
    pub duration_ns: u64,
    pub call_count: u64,
}

/// Kernel profiler
pub struct Profiler {
    samples: Vec<ProfileSample>,
    enabled: bool,
}

impl Profiler {
    pub const fn new() -> Self {
        Self { samples: Vec::new(), enabled: false }
    }

    pub fn start(&mut self) { self.enabled = true; }
    pub fn stop(&mut self) { self.enabled = false; }

    pub fn record(&mut self, function: &str, duration_ns: u64) {
        if !self.enabled { return; }
        if let Some(sample) = self.samples.iter_mut().find(|s| s.function == function) {
            sample.duration_ns += duration_ns;
            sample.call_count += 1;
        } else {
            self.samples.push(ProfileSample {
                function: String::from(function),
                duration_ns,
                call_count: 1,
            });
        }
    }

    pub fn hotspots(&self) -> Vec<&ProfileSample> {
        let mut sorted: Vec<_> = self.samples.iter().collect();
        sorted.sort_by(|a, b| b.duration_ns.cmp(&a.duration_ns));
        sorted.truncate(10); // top 10
        sorted
    }

    pub fn reset(&mut self) { self.samples.clear(); }
}

use spin::Mutex;
pub static PROFILER: Mutex<Profiler> = Mutex::new(Profiler::new());
