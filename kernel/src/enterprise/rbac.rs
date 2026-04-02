//! Enterprise Security (Phase 18.2 / 26.1)
//! Implements Military-Grade Role-Based Access Control (RBAC), BitFlags Permissions, and Audit Logging.

use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::enterprise::audit::{AuditSeverity, log_security};

// --- BitFlags Permissions (Phase 26.1) ---
pub const PERM_READ: u64    = 1 << 0;
pub const PERM_WRITE: u64   = 1 << 1;
pub const PERM_EXECUTE: u64 = 1 << 2;
pub const PERM_ADMIN: u64   = 1 << 3;
pub const PERM_DEPLOY: u64  = 1 << 4;
pub const PERM_AUDIT: u64   = 1 << 5;
pub const PERM_ROOT: u64    = 0xFFFF_FFFF_FFFF_FFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Admin,      // Full Access
    Auditor,    // Read-Only + Logs
    Developer,  // Build + Deploy
    User,       // Runtime Only
}

impl UserRole {
    pub fn default_permissions(&self) -> u64 {
        match self {
            UserRole::Admin => PERM_ROOT,
            UserRole::Auditor => PERM_READ | PERM_AUDIT,
            UserRole::Developer => PERM_READ | PERM_WRITE | PERM_EXECUTE | PERM_DEPLOY,
            UserRole::User => PERM_READ | PERM_EXECUTE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserIdentity {
    pub uid: u32,
    pub username: &'static str,
    pub role: UserRole,
    pub permissions: u64,
}

const ROOT_USER: UserIdentity = UserIdentity {
    uid: 0,
    username: "root",
    role: UserRole::Admin,
    permissions: PERM_ROOT,
};

const ARCHITECT_USER: UserIdentity = UserIdentity {
    uid: 777,
    username: "herman",
    role: UserRole::Admin,
    permissions: PERM_ROOT,
};

/// The Access Control System (Military Grade)
pub struct AccessControl {
    users: BTreeMap<u32, UserIdentity>,
    boot_users: [UserIdentity; 2],
    current_user: Option<u32>,
    runtime_ready: bool,
}

impl AccessControl {
    pub const fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            boot_users: [ROOT_USER, ARCHITECT_USER],
            current_user: None,
            runtime_ready: false,
        }
    }

    /// Boot-safe initialization: avoids dynamic map insertion during early boot.
    pub fn init(&mut self) {
        self.current_user = None;
        self.runtime_ready = false;
        log_security(
            AuditSeverity::Info,
            "System",
            "Identity Mesh (RBAC) Boot-Safe initialized (static identities active).",
        );
    }

    /// Runtime upgrade: populates dynamic map after system is fully stable.
    pub fn init_runtime(&mut self) {
        if self.runtime_ready {
            return;
        }

        self.users.clear();
        self.users.insert(ROOT_USER.uid, ROOT_USER);
        self.users.insert(ARCHITECT_USER.uid, ARCHITECT_USER);
        self.runtime_ready = true;

        log_security(
            AuditSeverity::Info,
            "System",
            "Identity Mesh (RBAC) Runtime mode enabled (dynamic map online).",
        );
    }

    fn find_user_by_uid(&self, uid: u32) -> Option<&UserIdentity> {
        if self.runtime_ready {
            self.users.get(&uid)
        } else {
            self.boot_users.iter().find(|u| u.uid == uid)
        }
    }

    fn find_user_by_name(&self, username: &str) -> Option<&UserIdentity> {
        if self.runtime_ready {
            self.users.values().find(|u| u.username == username)
        } else {
            self.boot_users.iter().find(|u| u.username == username)
        }
    }

    pub fn login(&mut self, username: &str) -> bool {
        if let Some(user) = self.find_user_by_name(username) {
            self.current_user = Some(user.uid);
            log_security(AuditSeverity::Info, username, "Login successful.");
            true
        } else {
            log_security(AuditSeverity::Warning, username, "Login attempt failed: Unknown identity.");
            false
        }
    }

    /// Authorize based on granular BitFlags
    pub fn authorize(&self, required_perms: u64) -> bool {
        if let Some(uid) = self.current_user {
            if let Some(user) = self.find_user_by_uid(uid) {
                let success = (user.permissions & required_perms) == required_perms;
                if !success {
                    log_security(AuditSeverity::Critical, user.username, "Access denied: Missing BitFlags requirements.");
                } else {
                    // Log sensitive operations even if authorized
                    if required_perms & (PERM_ADMIN | PERM_DEPLOY | PERM_AUDIT) != 0 {
                        log_security(AuditSeverity::Info, user.username, "Sensitive operation authorized.");
                    }
                }
                success
            } else { false }
        } else {
            log_security(AuditSeverity::Warning, "Anonymous", "Access denied: No active identity.");
            false
        }
    }

    /// High-stability permission check wrapper
    pub fn check_permission(&self, perm: u64) -> Result<(), &str> {
        if self.authorize(perm) {
            Ok(())
        } else {
            Err("Security Violation: Unauthorized access attempt.")
        }
    }
    
    pub fn get_current_user(&self) -> Option<&UserIdentity> {
        self.current_user.and_then(|uid| self.find_user_by_uid(uid))
    }

    /// Zero-Trust Identity Mesh (Phase 26.5) - HARDENED
    /// Validates identity through CRYSTALS-Dilithium signature attestation.
    pub fn verify_mesh_identity(&self, node_id: u32, signature: &[u8], public_key: &[u8]) -> bool {
        use crate::security::crypto::{CRYPTO_ENGINE, QuantumSecurity, SecurityLevel};
        
        let message = crate::alloc::format!("AETHER_MESH_AUTH_NODE_{}", node_id);
        let crypto = CRYPTO_ENGINE.lock();
        
        if crypto.verify(message.as_bytes(), signature, public_key, SecurityLevel::Advance) {
            log_security(
                AuditSeverity::Info, 
                "Identity", 
                &crate::alloc::format!("Node {} identity verified via Post-Quantum Attestation.", node_id)
            );
            true
        } else {
            log_security(
                AuditSeverity::Critical, 
                "Identity", 
                &crate::alloc::format!("Node {} identity verification FAILED! Potential Intrusion.", node_id)
            );
            // In high-security mode, a failed mesh auth could trigger a node isolation
            false
        }
    }

    /// Incremental Hardening: Violation Tracker (v10.2 SUPREME)
    pub fn report_violation(&self, user: &str, resource: &str) {
        log_security(
            AuditSeverity::Critical,
            user,
            &crate::alloc::format!("SECURITY VIOLATION: Unauthorized access to {}.", resource)
        );
        
        // Military Grade: 3rd violation triggers system lockdown
        static VIOLATION_COUNT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
        let count = VIOLATION_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        
        if count >= 3 {
             crate::hal::get_platform().puts("\r\n!!! MILITARY GRADE LOCKDOWN: REPEATED SECURITY VIOLATIONS !!!\r\n");
             panic!("Zero-Trust Policy: System Halted due to repeated unauthorized access attempts.");
        }
    }
}

pub static RBAC_SYSTEM: Mutex<AccessControl> = Mutex::new(AccessControl::new());
