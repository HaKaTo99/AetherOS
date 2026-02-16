pub mod capabilities;
pub mod hardening;
pub mod crypto; // [NEW] PQC Layer

pub use crypto::SecurityLevel;

pub mod homomorphic; // [NEW] Phase 24 (v6.0) FHE
pub mod attestation; // [NEW] Phase 24 (v6.0) Zero-Trust

// High-level security checks
pub fn check_permission(subject: &capabilities::SecurityContext, object: u32, perm: u32) -> bool {
    subject.has_permission(object, perm)
}
