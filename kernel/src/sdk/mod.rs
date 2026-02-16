//! AetherOS SDK - v5.4
//! 
//! Public System Call Interface for third-party applications.
//! Stable ABI for v6.0+.

pub mod syscalls {
    pub fn print(s: &str) {
        crate::print!("{}", s);
    }

    pub fn draw_window(title: &str, w: u32, h: u32) {
        crate::println!("[SDK] Creating Window '{}' ({}x{})", title, w, h);
    }

    pub fn connect_mesh(peer_id: u64) {
        crate::println!("[SDK] Connecting to mesh peer 0x{:X}", peer_id);
    }
}
