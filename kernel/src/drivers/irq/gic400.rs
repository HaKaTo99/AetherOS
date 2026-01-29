use crate::drivers::{Driver, DriverType};
use core::ptr::{read_volatile, write_volatile};

// Distributor registers
const GICD_CTLR: usize = 0x000;       // Control Register
const GICD_ISENABLER: usize = 0x100;  // Interrupt Set-Enable Registers
const GICD_ICENABLER: usize = 0x180;  // Interrupt Clear-Enable Registers
const GICD_IPRIORITYR: usize = 0x400; // Interrupt Priority Registers
const GICD_ITARGETSR: usize = 0x800;  // Interrupt Processor Targets

// CPU Interface registers
const GICC_CTLR: usize = 0x000;       // Control Register
const GICC_PMR: usize = 0x004;        // Priority Mask Register
const GICC_IAR: usize = 0x00C;        // Interrupt Acknowledge Register
const GICC_EOIR: usize = 0x010;       // End of Interrupt Register

pub const IRQ_TIMER: u32 = 30;

pub struct Gic400 {
    gicd_base: usize,
    gicc_base: usize,
}

impl Gic400 {
    pub const fn new(gicd_base: usize, gicc_base: usize) -> Self {
        Self { gicd_base, gicc_base }
    }

    /// Enable specific interrupt
    pub unsafe fn enable_interrupt(&self, irq: u32) {
        let reg = GICD_ISENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        let val = self.read_gicd(reg);
        self.write_gicd(reg, val | (1 << bit));
    }

    /// Disable specific interrupt
    pub unsafe fn disable_interrupt(&self, irq: u32) {
        let reg = GICD_ICENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        self.write_gicd(reg, 1 << bit);
    }

    /// Acknowledge interrupt (returns IRQ number)
    pub unsafe fn acknowledge(&self) -> u32 {
        self.read_gicc(GICC_IAR) & 0x3FF
    }

    /// End of interrupt
    pub unsafe fn end_of_interrupt(&self, irq: u32) {
        self.write_gicc(GICC_EOIR, irq);
    }

    #[inline]
    unsafe fn read_gicd(&self, offset: usize) -> u32 {
        read_volatile((self.gicd_base + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_gicd(&self, offset: usize, value: u32) {
        write_volatile((self.gicd_base + offset) as *mut u32, value);
    }

    #[inline]
    unsafe fn read_gicc(&self, offset: usize) -> u32 {
        read_volatile((self.gicc_base + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_gicc(&self, offset: usize, value: u32) {
        write_volatile((self.gicc_base + offset) as *mut u32, value);
    }
}

impl Driver for Gic400 {
    fn compatible(&self) -> &str {
        "arm,gic-400"
    }

    unsafe fn init(&mut self) -> Result<(), &'static str> {
        // 1. Disable distributor
        self.write_gicd(GICD_CTLR, 0);

        // 2. Disable all interrupts
        for i in 0..32 {
            self.write_gicd(GICD_ICENABLER + (i * 4), 0xFFFFFFFF);
        }

        // 3. Set priority for all interrupts (lower value = higher priority)
        // Default to median priority
        for i in 0..255 {
             // Access interrupt priority register. Each register holds 4 priority fields
             // (8 bits each, effectively). But for simplicity writing byte-wise or word-wise
             // This loop assumes direct byte access or word access needs calculation
             // Simplified: just write A0A0A0A0 to 32 bit registers covering 4 IRQs
             if i % 4 == 0 {
                  self.write_gicd(GICD_IPRIORITYR + i, 0xA0A0A0A0);
             }
        }

        // 4. Route all interrupts to CPU 0
        for i in 0..255 {
            if i % 4 == 0 {
                self.write_gicd(GICD_ITARGETSR + i, 0x01010101);
            }
        }

        // 5. Enable distributor
        self.write_gicd(GICD_CTLR, 1);

        // 6. Set priority mask (allow all priorities)
        self.write_gicc(GICC_PMR, 0xFF);

        // 7. Enable CPU interface
        self.write_gicc(GICC_CTLR, 1);

        Ok(())
    }

    fn device_type(&self) -> DriverType {
        DriverType::InterruptController
    }
}
