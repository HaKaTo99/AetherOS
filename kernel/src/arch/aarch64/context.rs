//! AArch64 CPU Context
//! Stores callee-saved registers for context switching

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuContext {
    // x19-x29 (Callee-saved)
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64, // Frame Pointer
    pub x30: u64, // Link Register
    pub sp: u64,  // Stack Pointer
}

impl CpuContext {
    pub const fn empty() -> Self {
        Self {
            x19: 0, x20: 0, x21: 0, x22: 0, x23: 0,
            x24: 0, x25: 0, x26: 0, x27: 0, x28: 0,
            x29: 0, x30: 0, sp: 0,
        }
    }
}
