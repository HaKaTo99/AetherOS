//! User Mode Demo
//! Manually switches to EL0 to execute a small piece of code and trigger a syscall.


// Shellcode that will run in User Mode
// 1. mov x0, #1337      (Argument for syscall)
// 2. mov x8, #1         (Syscall number 1 = WRITE, just a test)
// 3. svc #0             (Trigger syscall)
// 4. b .                (Spin loop)
const USER_CODE: [u8; 16] = [
    0x00, 0x47, 0x8a, 0xd2, // mov x0, #0x539 (1337)
    0x28, 0x00, 0x80, 0xd2, // mov x8, #1
    0x01, 0x00, 0x00, 0xd4, // svc #0
    0x00, 0x00, 0x00, 0x14, // b .
];

pub fn run_user_demo() {
    log::info!("Preparing User Mode Demo...");

    // 1. Allocate stack for user mode (just use a static array for now)
    static mut USER_STACK: [u8; 4096] = [0; 4096];
    
    // 2. Allocate code page (using unsafe to write to a page)
    // Ideally we should use the VMM, but for this demo we'll use a fixed address
    // if possible, or just execute from where the data is (if NX not set).
    // Note: Rust arrays are likely in .rodata or stack, which might be NX.
    // Let's copy it to a heap buffer which is usually RWX in our simple kernel.
    
    // Use the SMME allocator
    use alloc::vec::Vec;
    let mut code_page: Vec<u8> = Vec::with_capacity(4096);
    code_page.extend_from_slice(&USER_CODE);
    // Fill rest with zeros
    code_page.resize(4096, 0);
    
    let code_ptr = code_page.as_ptr() as u64;
    let stack_ptr = unsafe { USER_STACK.as_ptr() as u64 + 4096 };

    log::info!("Jumping to User Mode at {:#x}, SP={:#x}", code_ptr, stack_ptr);

    // 3. Perform the switch
    unsafe {
        switch_to_el0(code_ptr, stack_ptr);
    }
    
    log::info!("Back in Kernel Mode? (Should not be reached if user code loops)");
}

unsafe fn switch_to_el0(pc: u64, sp: u64) {
    // SPSR_EL1 configuration for EL0
    // M[3:0] = 0000 (EL0t)
    // F=0, I=0, A=0, D=0 (Interrupts unmasked)
    let spsr: u64 = 0; 
    
    core::arch::asm!(
        "msr spsr_el1, {spsr}",
        "msr elr_el1, {pc}",
        "msr sp_el0, {sp}",
        "eret",
        spsr = in(reg) spsr,
        pc = in(reg) pc,
        sp = in(reg) sp,
        options(noreturn)
    );
}
