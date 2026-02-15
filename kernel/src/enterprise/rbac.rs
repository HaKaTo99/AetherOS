//! Enterprise Security (Phase 18.2)
//! Implements Role-Based Access Control (RBAC) and Audit Logging.

use alloc::string::String;
use alloc::collections::BTreeMap;
use spin::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,      // Full Access
    Auditor,    // Read-Only + Logs
    Developer,  // Push Code + Debug
    User,       // Run Apps Only
}

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub uid: u32,
    pub username: String,
    pub role: UserRole,
}

/// The Access Control System
pub struct AccessControl {
    users: BTreeMap<u32, UserIdentity>,
    current_user: Option<u32>,
}

impl AccessControl {
    pub const fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            current_user: None,
        }
    }

    pub fn init(&mut self) {
        // Create default admin
        self.users.insert(0, UserIdentity {
            uid: 0,
            username: String::from("root"),
            role: UserRole::Admin,
        });
        crate::println!("[RBAC] Security Subsystem Initialized. Root account active.");
    }

    pub fn login(&mut self, username: &str) -> bool {
        // Mock Login
        if let Some(user) = self.users.values().find(|u| u.username == username) {
            self.current_user = Some(user.uid);
            crate::println!("[RBAC] User '{}' logged in. Role: {:?}", username, user.role);
            true
        } else {
            crate::println!("[RBAC] Login failed for '{}'", username);
            false
        }
    }

    pub fn authorize(&self, action: &str) -> bool {
        if let Some(uid) = self.current_user {
            if let Some(user) = self.users.get(&uid) {
                match user.role {
                    UserRole::Admin => true,
                    UserRole::Auditor => action.starts_with("read"),
                    UserRole::Developer => action == "deploy" || action == "debug" || action.starts_with("read"),
                    UserRole::User => action == "run",
                }
            } else { false }
        } else { false }
    }
}

pub static RBAC_SYSTEM: Mutex<AccessControl> = Mutex::new(AccessControl::new());
