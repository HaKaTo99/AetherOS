//! GIC-400 (Generic Interrupt Controller) Driver for ARM Cortex-A
//! 
//! RPi4 uses GIC-400 at base 0xFF840000

use core::ptr::{read_volatile, write_volatile};

// GIC-400 base addresses for RPi4
const GICD_BASE: usize = 0xFF841000;  // Distributor
const GICC_BASE: usize = 0xFF842000;  // CPU Interface

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

// Interrupt numbers for RPi4
pub const IRQ_TIMER: u32 = 30;  // ARM Generic Timer (PPI ID 14 + 16)

pub struct Gic;

impl Gic {
    /// Initialize GIC-400
    pub unsafe fn init() {
        // 1. Disable distributor
        Self::write_gicd(GICD_CTLR, 0);

        // 2. Disable all interrupts
        for i in 0..32 {
            Self::write_gicd(GICD_ICENABLER + (i * 4), 0xFFFFFFFF);
        }

        // 3. Set priority for all interrupts (lower value = higher priority)
        for i in 0..255 {
            Self::write_gicd(GICD_IPRIORITYR + i, 0xA0);
        }

        // 4. Route all interrupts to CPU 0
        for i in 0..255 {
            Self::write_gicd(GICD_ITARGETSR + i, 0x01010101);
        }

        // 5. Enable distributor
        Self::write_gicd(GICD_CTLR, 1);

        // 6. Set priority mask (allow all priorities)
        Self::write_gicc(GICC_PMR, 0xFF);

        // 7. Enable CPU interface
        Self::write_gicc(GICC_CTLR, 1);
    }

    /// Enable specific interrupt
    pub unsafe fn enable_interrupt(irq: u32) {
        let reg = GICD_ISENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        let val = Self::read_gicd(reg);
        Self::write_gicd(reg, val | (1 << bit));
    }

    /// Disable specific interrupt
    pub unsafe fn disable_interrupt(irq: u32) {
        let reg = GICD_ICENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        Self::write_gicd(reg, 1 << bit);
    }

    /// Acknowledge interrupt (returns IRQ number)
    pub unsafe fn acknowledge() -> u32 {
        Self::read_gicc(GICC_IAR) & 0x3FF
    }

    /// End of interrupt
    pub unsafe fn end_of_interrupt(irq: u32) {
        Self::write_gicc(GICC_EOIR, irq);
    }

    #[inline]
    unsafe fn read_gicd(offset: usize) -> u32 {
        read_volatile((GICD_BASE + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_gicd(offset: usize, value: u32) {
        write_volatile((GICD_BASE + offset) as *mut u32, value);
    }

    #[inline]
    unsafe fn read_gicc(offset: usize) -> u32 {
        read_volatile((GICC_BASE + offset) as *const u32)
    }

    #[inline]
    unsafe fn write_gicc(offset: usize, value: u32) {
        write_volatile((GICC_BASE + offset) as *mut u32, value);
    }
}
