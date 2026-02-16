use x86_64::instructions::port::Port;

pub struct DebugExit {
    port: Port<u32>,
}

impl DebugExit {
    pub const fn new() -> Self {
        DebugExit {
            port: Port::new(0xf4),
        }
    }

    pub fn exit_success(&mut self) {
        unsafe {
            self.port.write(0x10); // QEMU exit code (odd number << 1) | 1 ? No, just standard debug exit
        }
    }
}
