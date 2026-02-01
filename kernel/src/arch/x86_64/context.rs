#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuContext {
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub sp: u64,
}

impl CpuContext {
    pub const fn empty() -> Self {
        Self {
            rbx: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            sp: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub elr: u64, // Dummy for x86
    pub elr_el1: u64, // GDB compat
    pub spsr_el1: u64, // GDB compat
    pub sp_el0: u64, // GDB compat
    pub x: [u64; 31], // GDB compat
}
