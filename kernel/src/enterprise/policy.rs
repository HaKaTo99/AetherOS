//! Corporate Ability Policy (Phase 26.7)
//! Definisi kebijakan untuk distribusi beban kerja di lingkungan enterprise.

use alloc::string::String;
use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::enterprise::audit::{AuditSeverity, log_security};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyAction {
    Allow,
    Deny,
    Audit,
    Encrypt,
}

pub struct AbilityPolicy {
    rules: BTreeMap<String, PolicyAction>,
}

impl AbilityPolicy {
    pub const fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
        }
    }

    pub fn set_rule(&mut self, ability: &str, action: PolicyAction) {
        self.rules.insert(String::from(ability), action);
        log_security(AuditSeverity::Info, "Policy", &format!("Rule updated for {}: {:?}", ability, action));
    }

    pub fn evaluate(&self, ability: &str) -> PolicyAction {
        *self.rules.get(ability).unwrap_or(&PolicyAction::Deny)
    }
}

pub static CORPORATE_POLICY: Mutex<AbilityPolicy> = Mutex::new(AbilityPolicy::new());
