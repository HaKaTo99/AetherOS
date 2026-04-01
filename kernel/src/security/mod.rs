pub mod capabilities;
pub mod hardening;
pub mod crypto; // [NEW] PQC Layer

pub use crypto::SecurityLevel;

pub mod homomorphic; // [NEW] Phase 24 (v6.0) FHE
pub mod attestation; // [NEW] Phase 24 (v6.0) Zero-Trust
pub mod identity;    // [NEW] Phase 28.2 Universal Data Sovereignty
pub mod tpm;         // [NEW] Phase 11 TPM 2.0 Hardening
pub mod air_gap;     // [NEW] Phase 29 Air Gapped Networking Profile
pub mod sandbox;     // [NEW] Quarantined Container MAC

// High-level security checks
pub fn check_permission(subject: &capabilities::SecurityContext, object: u32, perm: u32) -> bool {
    subject.has_permission(object, perm)
}
