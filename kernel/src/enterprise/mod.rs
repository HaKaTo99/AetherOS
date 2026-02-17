//! Enterprise & Cloud Infrastructure (Phase 18)
//!
//! Provides capabilities for cloud deployment, enterprise security, and fleet management.

pub mod cloud;     // Phase 18.1
pub mod rbac;      // Phase 18.2
pub mod marketplace; // [NEW] Phase 27.2
pub mod telemetry; // Phase 18.3
pub mod audit;     // [NEW] Phase 26.1
pub mod sovereign; // [NEW] Phase 26.1 / 29.1
pub mod ota;       // [NEW] Phase 26.3
pub mod policy;    // [NEW] Phase 26.7
pub mod lifecycle; // Phase 36.0
pub mod shell;     // Phase 38.2

pub use cloud::{CloudManager, CLOUD_MANAGER};
pub use rbac::{AccessControl, RBAC_SYSTEM};
pub use marketplace::{MarketplaceManager, MARKETPLACE};
pub use telemetry::{TelemetryAgent, TELEMETRY_AGENT};
pub use audit::{AuditLogger, AUDIT_LOGGER};
pub use ota::{OTAManager, OTA_MANAGER};
pub use policy::{AbilityPolicy, CORPORATE_POLICY};
pub use sovereign::{SovereignManager, SOVEREIGN_MANAGER};
pub use lifecycle::{PowerManager, POWER_MANAGER};
pub use shell::AetherShell;
