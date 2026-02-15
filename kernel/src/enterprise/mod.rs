//! Enterprise & Cloud Infrastructure (Phase 18)
//!
//! Provides capabilities for cloud deployment, enterprise security, and fleet management.

pub mod cloud;     // Phase 18.1
pub mod rbac;      // Phase 18.2
pub mod telemetry; // Phase 18.3

pub use cloud::{CloudManager, CLOUD_MANAGER};
pub use rbac::{AccessControl, RBAC_SYSTEM};
pub use telemetry::{TelemetryAgent, TELEMETRY_AGENT};
