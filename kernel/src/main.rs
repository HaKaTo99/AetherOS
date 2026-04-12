#![no_std]
#![no_main]

use aetheros_kernel::{kernel_init, BOOT_PARAMS};
use aetheros_kernel::hal::Platform;

// [v10.4.15] Multiboot2 header HANYA didefinisikan di boot.rs via global_asm!
// Section .multiboot_header di-link pertama oleh linker script fabric_v10_gold.ld
// sehingga GRUB dapat menemukannya dalam 8KB pertama dari binary.

#[no_mangle]
pub extern "C" fn kernel_main_grub(magic: u32, info_ptr: usize) -> ! {
    static X86: aetheros_kernel::hal::x86_64::X86Platform = aetheros_kernel::hal::x86_64::X86Platform::new();

    // [MILITARY GRADE] Early hardware initialization to unlock diagnostics
    X86.init();

    // [v10.4.15] SOVEREIGN SIGNAL: Sovereign Singularity Active.
    X86.puts("\r\n[v10.4.15] SOLAR CORE: Sovereign Singularity Active.\r\n");

    if magic != 0x36d76289 {
        panic!("Not booted by Multiboot2");
    }

    let cmdline = unsafe { aetheros_kernel::boot::cmdline::find_multiboot2_cmdline(info_ptr) }.unwrap_or("");
    let params = aetheros_kernel::boot::cmdline::parse_cmdline(cmdline);
    
    // [v10.4.7] LFB Audit Logic: Diagnostic Sovereignty
    if let Some(_info) = unsafe { aetheros_kernel::boot::cmdline::find_multiboot2_framebuffer(info_ptr) } {
        X86.puts("[v10.4.7] LFB DETECTED\r\n");
    } else {
        X86.puts("[v10.4.7] WARNING: NO LFB TAG\r\n");
    }

    {
        let mut lock = BOOT_PARAMS.lock();
        *lock = params;
    }

    // Pass the Multiboot2 Info Pointer for full tag discovery
    kernel_init(info_ptr);
    
    // kernel_init enters AetherShell which is infinite loop and never returns
    panic!("kernel_init should not return!");
}

