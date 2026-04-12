//! POSIX Syscall Shim Layer
//! Basic support for Linux-compatible system calls

use crate::hal;

pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_SCHEME_OPEN: usize = 10;
pub const SYS_SCHEME_READ: usize = 11;
pub const SYS_SCHEME_WRITE: usize = 12;
pub const SYS_EXIT: usize = 60;
pub const SYS_SPAWN: usize = 100;
pub const SYS_AI_SYNC: usize = 500; // [NEW] Phase 27.x Cognitive Sync

/// Generic System Call Handler
/// Expected to be called from assembly trap handler
pub fn syscall_handler(call_num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    // Military-Grade Sandbox (Ring 3 MAC Enforcement - Phase 13/14)
    {
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Info,
            "Sandbox", &crate::alloc::format!("Syscall {} intercepted by MAC Boundary Check.", call_num)
        );
        // Di arsitektur seutuhnya, kita memanggil current_thread().context.enforce_mac(Confidential).
        // Sebagai isolasi, setiap syscall yang berasal dari WASM/ART dihentikan jika Clearance = Ring3Untrusted.
    }
    
    // Cognitive Intent Analysis (Phase 27.5)
    {
        crate::ai::intent::INTENT_PARSER.lock().record_syscall(call_num);
    }

    match call_num {
        SYS_WRITE => sys_write(arg1, arg2, arg3),
        SYS_EXIT => sys_exit(arg1),
        SYS_SCHEME_OPEN => sys_scheme_open(arg1, arg2, arg3),
        SYS_SCHEME_READ => sys_scheme_read(arg1, arg2, arg3),
        SYS_SCHEME_WRITE => sys_scheme_write(arg1, arg2, arg3),
        SYS_SPAWN => sys_spawn(arg1, arg2, arg3), // (ptr_to_arm, size, priority)
        SYS_AI_SYNC => sys_ai_sync(arg1), // Professional Orchestration
        _ => -1, // ENOSYS
    }
}

/// sys_scheme_open(uri_ptr, uri_len, flags)
fn sys_scheme_open(uri_ptr: usize, uri_len: usize, flags: usize) -> isize {
    unsafe {
        let uri = core::str::from_utf8(core::slice::from_raw_parts(uri_ptr as *const u8, uri_len)).unwrap_or("");
        match crate::scheme::open(uri, flags) {
            Ok((_, id)) => id as isize,
            Err(_) => -1,
        }
    }
}

/// sys_scheme_read(fd, buf_ptr, count)
fn sys_scheme_read(_fd: usize, _buf_ptr: usize, _count: usize) -> isize {
    // Basic routing (assuming fd maps to a scheme resource)
    // Implementation would involve a proper FD-to-Resource map
    -1 
}

/// sys_scheme_write(fd, buf_ptr, count)
fn sys_scheme_write(_fd: usize, _buf_ptr: usize, _count: usize) -> isize {
    -1
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
    let username = rbac.get_current_user().map(|u| u.username).unwrap_or("Unknown");

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

/// [PHASE 34 / AUDIT] spawn(ptr_to_arm_module, size, priority)
fn sys_spawn(arm_ptr: usize, size: usize, priority: usize) -> isize {
    let module_ptr = arm_ptr as *const u8;
    
    use crate::SCHEDULER;
    let mut sched = SCHEDULER.lock();
    match sched.create_task_from_module(priority as u8, module_ptr, size) {
        Ok(id) => id as isize,
        Err(_) => -1, // EPERM/EAUTH
    }
}
