#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_arch = "aarch64"))]
pub mod context {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct CpuContext {
        pub sp: u64,
    }
    
    impl CpuContext {
        pub const fn empty() -> Self {
            Self { sp: 0 }
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
pub use context::*;
