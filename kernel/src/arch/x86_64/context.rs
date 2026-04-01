#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuContext {
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,
    pub sp: u64,
}

impl CpuContext {
    pub const fn empty() -> Self {
        Self {
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: 0,
            sp: 0,
        }
    }
}

/// Military Grade `TrapFrame` representing the exact layout pushed by the CPU + ISR Stub
/// No dummy registers or aarch64 compatibility layers.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct TrapFrame {
    // 1. General Purpose Registers (Pushed by our Assembly ISR Stub in exact order)
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    // 2. Exception/Interrupt Specific Data 
    pub int_num: u64,
    pub error_code: u64,

    // 3. Hardware Auto-Pushed State (Exact x86_64 CPU behavior on Interrupt ring 3 -> ring 0)
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl TrapFrame {
    /// Scrubbing function to zero out non-essential registers upon exit to prevent 
    /// data leakage (Cold-Boot/Cross-Thread forensic).
    pub fn scrub_registers(&mut self) {
        self.r15 = 0; self.r14 = 0; self.r13 = 0; self.r12 = 0;
        self.r11 = 0; self.r10 = 0; self.r9 = 0; self.r8 = 0;
        self.rdi = 0; self.rsi = 0; self.rdx = 0; self.rcx = 0;
        self.rbx = 0; self.rax = 0;
        // Leave RBP, RIP, CS, RFLAGS, RSP, SS alone for return context.
    }
}
