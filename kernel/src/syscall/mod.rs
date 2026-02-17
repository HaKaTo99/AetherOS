//! POSIX Syscall Shim Layer
//! Basic support for Linux-compatible system calls

use crate::hal;

pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_EXIT: usize = 60;
pub const SYS_AI_SYNC: usize = 500; // [NEW] Phase 27.x Cognitive Sync

/// Generic System Call Handler
/// Expected to be called from assembly trap handler
pub fn syscall_handler(call_num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    // Cognitive Intent Analysis (Phase 27.5)
    {
        crate::ai::intent::INTENT_PARSER.lock().record_syscall(call_num);
    }

    match call_num {
        SYS_WRITE => sys_write(arg1, arg2, arg3),
        SYS_EXIT => sys_exit(arg1),
        SYS_AI_SYNC => sys_ai_sync(arg1), // Professional Orchestration
        _ => -1, // ENOSYS
    }
}

/// write(fd, buf, count)
fn sys_write(fd: usize, buf_ptr: usize, count: usize) -> isize {
    use crate::enterprise::rbac::{RBAC_SYSTEM, PERM_WRITE};
    use crate::enterprise::audit::{AuditSeverity, log_security};

    // RBAC Enforcement (Phase 26.1)
    let rbac = RBAC_SYSTEM.lock();
    if let Err(e) = rbac.check_permission(PERM_WRITE) {
        log_security(AuditSeverity::Critical, "Syscall", e);
        return -13; // EACCES
    }
    let username = rbac.get_current_user().map(|u| u.username.as_str()).unwrap_or("Unknown");

    // For now, assume fd 1 (stdout) and 2 (stderr) go to serial console
    if fd == 1 || fd == 2 {
        unsafe {
            if let Some(platform) = hal::try_get_platform() {
                let slice = core::slice::from_raw_parts(buf_ptr as *const u8, count);
                for &b in slice {
                    platform.put_char(b);
                }
                
                // Optional: Log large writes for audit
                if count > 256 {
                    log_security(AuditSeverity::Info, username, "Large buffer write to console.");
                }
                
                return count as isize;
            }
        }
    }
    -1 // EBADF
}

/// exit(status)
fn sys_exit(status: usize) -> isize {
    unsafe {
        if let Some(platform) = hal::try_get_platform() {
             // In a real OS, this would terminate the process.
             // Here, we log it and halt for demonstration.
             platform.put_char(b'E');
             platform.put_char(b'X');
             platform.put_char(b'I');
             platform.put_char(b'T');
             platform.put_char(b' ');
             platform.put_char(b'0' + (status as u8)); // Simple digit print
             platform.put_char(b'\n');
        }
    }
    loop {} // Halt execution
}

/// sys_ai_sync(options) - Phase 27.x
/// Synchronizes user intent with kernel fabric state.
fn sys_ai_sync(options: usize) -> isize {
    use crate::enterprise::audit::{AuditSeverity, log_security};
    
    log_security(AuditSeverity::Info, "AI-Sync", &format!("Professional Sync Requested (Options: {})", options));
    
    // Harmony Logic: Align thread priority with predicted intent
    let intent = crate::ai::intent::INTENT_PARSER.lock().predict_intent();
    match intent {
        crate::ai::intent::UserIntent::Development => {
            log_security(AuditSeverity::Info, "AI-Sync", "Aligning for Development workload.");
        }
        _ => {}
    }
    
    0 // Success
}
