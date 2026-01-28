//! Panic handler for kernel

use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Try to get platform for serial output
    unsafe {
        if let Some(platform) = crate::hal::try_get_platform() {
            let _ = write!(PanicWriter(platform), "\n\n*** KERNEL PANIC ***\n");
            let _ = write!(PanicWriter(platform), "{}\n", info);
            let _ = write!(PanicWriter(platform), "\nSystem halted.\n");
        }
    }

    // Halt CPU
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

struct PanicWriter(&'static dyn crate::hal::Platform);

impl core::fmt::Write for PanicWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.0.put_char(byte);
        }
        Ok(())
    }
}
