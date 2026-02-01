#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
pub mod context {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct CpuContext {
        pub sp: u64,
    }

    #[derive(Debug, Clone, Copy, Default)]
    #[repr(C)]
    pub struct TrapFrame {
        pub elr: u64, // Dummy for x86
        pub elr_el1: u64, // GDB compat
        pub spsr_el1: u64, // GDB compat
        pub sp_el0: u64, // GDB compat
        pub x: [u64; 31], // GDB compat (Registers x0-x30)
    }
    
    impl CpuContext {
        pub const fn empty() -> Self {
            Self { sp: 0 }
        }
    }
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
pub use context::*;
