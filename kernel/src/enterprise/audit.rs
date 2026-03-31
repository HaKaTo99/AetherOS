//! Centralized Audit Logging (Phase 26.1)
//! Provides military-grade traceability for security and system events.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSeverity {
    Info,
    Warning,
    Critical,      // Potential breach or attack
    Emergency,     // System-wide compromise
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub severity: AuditSeverity,
    pub component: String,
    pub user: String,
    pub message: String,
}

pub struct AuditLogger {
    buffer: Vec<AuditEvent>,
    max_size: usize,
}

impl AuditLogger {
    pub const fn new(max_size: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_size,
        }
    }

    pub fn log(&mut self, severity: AuditSeverity, component: &str, user: &str, message: &str) {
        let timestamp = crate::hal::get_platform().get_ticks();
        let event = AuditEvent {
            timestamp,
            severity,
            component: String::from(component),
            user: String::from(user),
            message: String::from(message),
        };

        // Circular buffer logic
        if self.buffer.len() >= self.max_size {
            self.buffer.remove(0);
        }
        self.buffer.push(event);

        // Immediate output for Critical/Emergency
        if severity == AuditSeverity::Critical || severity == AuditSeverity::Emergency {
            crate::println!("[AUDIT] !! {:?} !! [{}][{}] : {}", severity, component, user, message);
        }
    }

    pub fn get_recent_logs(&self, count: usize) -> Vec<AuditEvent> {
        let start = if self.buffer.len() > count { self.buffer.len() - count } else { 0 };
        self.buffer[start..].to_vec()
    }
}

pub static AUDIT_LOGGER: Mutex<AuditLogger> = Mutex::new(AuditLogger::new(1024));

/// Helper for security event logging
pub fn log_security(severity: AuditSeverity, user: &str, message: &str) {
    let mut logger = AUDIT_LOGGER.lock();
    logger.log(severity, "Security", user, message);
}
