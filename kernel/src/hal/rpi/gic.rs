//! GIC-400 (Generic Interrupt Controller) Driver for ARM Cortex-A
//! 
//! RPi4 uses GIC-400 at base 0xFF840000
//! 
//! Features:
//! - Full priority configuration (0-255, lower = higher priority)
//! - CPU targeting for multi-core systems
//! - Interrupt type configuration (level/edge)
//! - Pending/active status checking

use core::ptr::{read_volatile, write_volatile};

// GIC-400 base addresses for RPi4
const GICD_BASE: usize = 0xFF841000;  // Distributor
const GICC_BASE: usize = 0xFF842000;  // CPU Interface

// Distributor registers
const GICD_CTLR: usize = 0x000;       // Control Register
const GICD_TYPER: usize = 0x004;      // Interrupt Controller Type Register
const GICD_IGROUPR: usize = 0x080;    // Interrupt Group Registers
const GICD_ISENABLER: usize = 0x100;  // Interrupt Set-Enable Registers
const GICD_ICENABLER: usize = 0x180;  // Interrupt Clear-Enable Registers
const GICD_ISPENDR: usize = 0x200;    // Interrupt Set-Pending Registers
const GICD_ICPENDR: usize = 0x280;    // Interrupt Clear-Pending Registers
const GICD_ISACTIVER: usize = 0x300;  // Interrupt Set-Active Registers
const GICD_ICACTIVER: usize = 0x380;  // Interrupt Clear-Active Registers
const GICD_IPRIORITYR: usize = 0x400; // Interrupt Priority Registers
const GICD_ITARGETSR: usize = 0x800;  // Interrupt Processor Targets
const GICD_ICFGR: usize = 0xC00;      // Interrupt Configuration Registers

// CPU Interface registers
const GICC_CTLR: usize = 0x000;       // Control Register
const GICC_PMR: usize = 0x004;        // Priority Mask Register
const GICC_BPR: usize = 0x008;        // Binary Point Register
const GICC_IAR: usize = 0x00C;        // Interrupt Acknowledge Register
const GICC_EOIR: usize = 0x010;       // End of Interrupt Register
const GICC_RPR: usize = 0x014;        // Running Priority Register
const GICC_HPPIR: usize = 0x018;      // Highest Priority Pending Interrupt

// Interrupt numbers for RPi4
pub const IRQ_TIMER: u32 = 30;        // ARM Generic Timer (PPI ID 14 + 16)
pub const IRQ_UART: u32 = 153;        // PL011 UART interrupt
pub const IRQ_GPIO: u32 = 145;        // GPIO interrupt
pub const IRQ_MAILBOX: u32 = 65;      // Mailbox interrupt

/// Priority levels (lower value = higher priority)
pub mod priority {
    pub const HIGHEST: u8 = 0x00;     // Highest priority (critical)
    pub const TIMER: u8 = 0x10;       // Timer interrupts
    pub const IPC: u8 = 0x20;         // IPC/scheduler
    pub const DRIVER_HIGH: u8 = 0x40; // High-priority drivers
    pub const DRIVER_NORMAL: u8 = 0x80; // Normal drivers
    pub const DRIVER_LOW: u8 = 0xA0;  // Low-priority drivers
    pub const LOWEST: u8 = 0xF0;      // Lowest priority
    pub const DEFAULT: u8 = 0xA0;     // Default priority
}

/// Interrupt trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    LevelSensitive = 0,
    EdgeTriggered = 1,
}

/// CPU target mask
pub mod cpu_target {
    pub const CPU0: u8 = 0x01;
    pub const CPU1: u8 = 0x02;
    pub const CPU2: u8 = 0x04;
    pub const CPU3: u8 = 0x08;
    pub const ALL: u8 = 0x0F;
}

/// GIC statistics for debugging
#[derive(Debug, Default, Clone, Copy)]
pub struct GicStats {
    pub irq_count: u64,
    pub spurious_count: u64,
    pub max_irq_seen: u32,
}

static mut GIC_STATS: GicStats = GicStats {
    irq_count: 0,
    spurious_count: 0,
    max_irq_seen: 0,
};

pub struct Gic;

impl Gic {
    /// Initialize GIC-400 with optimal configuration
    pub unsafe fn init() {
        // 1. Disable distributor
        Self::write_gicd(GICD_CTLR, 0);

        // 2. Disable all interrupts
        for i in 0..32 {
            Self::write_gicd(GICD_ICENABLER + (i * 4), 0xFFFFFFFF);
        }

        // 3. Set default priority for all interrupts
        for i in 0..255 {
            Self::write_gicd(GICD_IPRIORITYR + i, priority::DEFAULT as u32 * 0x01010101);
        }

        // 4. Route all interrupts to CPU 0 by default
        for i in 0..255 {
            Self::write_gicd(GICD_ITARGETSR + i, 0x01010101);
        }

        // 5. Configure critical interrupts with higher priorities
        Self::set_priority(IRQ_TIMER, priority::TIMER);

        // 6. Enable distributor
        Self::write_gicd(GICD_CTLR, 1);

        // 7. Set priority mask (allow all priorities)
        Self::write_gicc(GICC_PMR, 0xFF);

        // 8. Set binary point (no priority grouping)
        Self::write_gicc(GICC_BPR, 0);

        // 9. Enable CPU interface
        Self::write_gicc(GICC_CTLR, 1);
    }

    /// Enable specific interrupt
    pub unsafe fn enable_interrupt(irq: u32) {
        let reg = GICD_ISENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        Self::write_gicd(reg, 1 << bit);
    }

    /// Disable specific interrupt
    pub unsafe fn disable_interrupt(irq: u32) {
        let reg = GICD_ICENABLER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        Self::write_gicd(reg, 1 << bit);
    }

    /// Set interrupt priority (0 = highest, 255 = lowest)
    pub unsafe fn set_priority(irq: u32, prio: u8) {
        let reg_offset = (irq / 4) as usize * 4;
        let byte_offset = (irq % 4) as usize;
        let reg = GICD_IPRIORITYR + reg_offset;
        
        let mut val = Self::read_gicd(reg);
        let mask = 0xFF << (byte_offset * 8);
        val &= !mask;
        val |= (prio as u32) << (byte_offset * 8);
        Self::write_gicd(reg, val);
    }

    /// Get interrupt priority
    pub unsafe fn get_priority(irq: u32) -> u8 {
        let reg_offset = (irq / 4) as usize * 4;
        let byte_offset = (irq % 4) as usize;
        let reg = GICD_IPRIORITYR + reg_offset;
        
        let val = Self::read_gicd(reg);
        ((val >> (byte_offset * 8)) & 0xFF) as u8
    }

    /// Set CPU target for interrupt (which CPUs can handle this IRQ)
    pub unsafe fn set_target(irq: u32, cpu_mask: u8) {
        let reg_offset = (irq / 4) as usize * 4;
        let byte_offset = (irq % 4) as usize;
        let reg = GICD_ITARGETSR + reg_offset;
        
        let mut val = Self::read_gicd(reg);
        let mask = 0xFF << (byte_offset * 8);
        val &= !mask;
        val |= (cpu_mask as u32) << (byte_offset * 8);
        Self::write_gicd(reg, val);
    }

    /// Get CPU target for interrupt
    pub unsafe fn get_target(irq: u32) -> u8 {
        let reg_offset = (irq / 4) as usize * 4;
        let byte_offset = (irq % 4) as usize;
        let reg = GICD_ITARGETSR + reg_offset;
        
        let val = Self::read_gicd(reg);
        ((val >> (byte_offset * 8)) & 0xFF) as u8
    }

    /// Set interrupt trigger type (level or edge)
    pub unsafe fn set_trigger_type(irq: u32, trigger: TriggerType) {
        let reg_offset = (irq / 16) as usize * 4;
        let bit_offset = ((irq % 16) * 2 + 1) as usize;
        let reg = GICD_ICFGR + reg_offset;
        
        let mut val = Self::read_gicd(reg);
        match trigger {
            TriggerType::LevelSensitive => val &= !(1 << bit_offset),
            TriggerType::EdgeTriggered => val |= 1 << bit_offset,
        }
        Self::write_gicd(reg, val);
    }

    /// Check if interrupt is pending
    pub unsafe fn is_pending(irq: u32) -> bool {
        let reg = GICD_ISPENDR + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        (Self::read_gicd(reg) & (1 << bit)) != 0
    }

    /// Clear pending interrupt
    pub unsafe fn clear_pending(irq: u32) {
        let reg = GICD_ICPENDR + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        Self::write_gicd(reg, 1 << bit);
    }

    /// Check if interrupt is active
    pub unsafe fn is_active(irq: u32) -> bool {
        let reg = GICD_ISACTIVER + ((irq / 32) * 4) as usize;
        let bit = irq % 32;
        (Self::read_gicd(reg) & (1 << bit)) != 0
    }

    /// Acknowledge interrupt (returns IRQ number)
    pub unsafe fn acknowledge() -> u32 {
        let irq = Self::read_gicc(GICC_IAR) & 0x3FF;
        
        // Update statistics
        if irq < 1020 {
            GIC_STATS.irq_count += 1;
            if irq > GIC_STATS.max_irq_seen {
                GIC_STATS.max_irq_seen = irq;
            }
        } else {
            GIC_STATS.spurious_count += 1;
        }
        
        irq
    }

    /// End of interrupt
    pub unsafe fn end_of_interrupt(irq: u32) {
        Self::write_gicc(GICC_EOIR, irq);
    }

    /// Get highest priority pending interrupt
    pub unsafe fn get_highest_pending() -> u32 {
        Self::read_gicc(GICC_HPPIR) & 0x3FF
    }

    /// Get running priority (current highest priority being serviced)
    pub unsafe fn get_running_priority() -> u8 {
        Self::read_gicc(GICC_RPR) as u8
    }

    /// Set priority mask (only interrupts with priority higher than mask are forwarded)
    pub unsafe fn set_priority_mask(mask: u8) {
        Self::write_gicc(GICC_PMR, mask as u32);
    }

    /// Get number of supported interrupt lines
    pub unsafe fn get_irq_count() -> u32 {
        let typer = Self::read_gicd(GICD_TYPER);
        ((typer & 0x1F) + 1) * 32
    }

    /// Get GIC statistics
    pub fn get_stats() -> GicStats {
        unsafe { GIC_STATS }
    }

    /// Reset GIC statistics
    pub fn reset_stats() {
        unsafe {
            GIC_STATS = GicStats::default();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_levels() {
        assert!(priority::HIGHEST < priority::TIMER);
        assert!(priority::TIMER < priority::IPC);
        assert!(priority::IPC < priority::DRIVER_HIGH);
        assert!(priority::DRIVER_HIGH < priority::DRIVER_NORMAL);
        assert!(priority::DRIVER_NORMAL < priority::LOWEST);
    }

    #[test]
    fn test_cpu_targets() {
        assert_eq!(cpu_target::CPU0, 0x01);
        assert_eq!(cpu_target::ALL, 0x0F);
        assert_eq!(cpu_target::CPU0 | cpu_target::CPU1, 0x03);
    }

    #[test]
    fn test_gic_stats_default() {
        let stats = GicStats::default();
        assert_eq!(stats.irq_count, 0);
        assert_eq!(stats.spurious_count, 0);
        assert_eq!(stats.max_irq_seen, 0);
    }
}
