pub mod capabilities;

// High-level security checks
pub fn check_permission(subject: &capabilities::SecurityContext, object: u32, perm: u32) -> bool {
    subject.has_permission(object, perm)
}
