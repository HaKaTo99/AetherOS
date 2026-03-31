#![allow(dead_code)]
//! Mailbox Hardware Registers for Raspberry Pi 4 (BCM2711)
//! 
//! The mailbox is a communication mechanism between the ARM CPU and VideoCore GPU.
//! Base address: 0xFE00B880 (BCM2711 peripheral base + mailbox offset)

use core::ptr::{read_volatile, write_volatile};

/// Mailbox base address for BCM2711 (RPi4)
const MAILBOX_BASE: usize = 0xFE00B880;

/// Mailbox register offsets
const MAILBOX_READ: usize = 0x00;
const MAILBOX_POLL: usize = 0x10;
const MAILBOX_SENDER: usize = 0x14;
const MAILBOX_STATUS: usize = 0x18;
const MAILBOX_CONFIG: usize = 0x1C;
const MAILBOX_WRITE: usize = 0x20;

/// Status register flags
const MAILBOX_FULL: u32 = 0x80000000;
const MAILBOX_EMPTY: u32 = 0x40000000;

/// Mailbox channels
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MailboxChannel {
    PowerManagement = 0,
    FrameBuffer = 1,
    VirtualUart = 2,
    Vchiq = 3,
    Leds = 4,
    Buttons = 5,
    TouchScreen = 6,
    PropertyTagsArmToVc = 8,
    PropertyTagsVcToArm = 9,
}

/// Mailbox peripheral interface
pub struct MailboxRegisters {
    base: usize,
}

impl MailboxRegisters {
    /// Create a new mailbox register interface
    pub const fn new() -> Self {
        Self {
            base: MAILBOX_BASE,
        }
    }

    /// Read from mailbox
    #[inline]
    unsafe fn read_reg(&self, offset: usize) -> u32 {
        read_volatile((self.base + offset) as *const u32)
    }

    /// Write to mailbox
    #[inline]
    unsafe fn write_reg(&self, offset: usize, value: u32) {
        write_volatile((self.base + offset) as *mut u32, value);
    }

    /// Check if mailbox is full (can't write)
    pub fn is_full(&self) -> bool {
        unsafe { (self.read_reg(MAILBOX_STATUS) & MAILBOX_FULL) != 0 }
    }

    /// Check if mailbox is empty (can't read)
    pub fn is_empty(&self) -> bool {
        unsafe { (self.read_reg(MAILBOX_STATUS) & MAILBOX_EMPTY) != 0 }
    }

    /// Write to mailbox (blocking)
    pub fn write(&self, channel: MailboxChannel, data: u32) {
        // Wait until mailbox is not full
        while self.is_full() {
            core::hint::spin_loop();
        }

        // Combine data (upper 28 bits) with channel (lower 4 bits)
        let msg = (data & 0xFFFFFFF0) | (channel as u32 & 0xF);
        
        unsafe {
            self.write_reg(MAILBOX_WRITE, msg);
        }
    }

    /// Read from mailbox (blocking)
    pub fn read(&self, channel: MailboxChannel) -> u32 {
        loop {
            // Wait until mailbox is not empty
            while self.is_empty() {
                core::hint::spin_loop();
            }

            let msg = unsafe { self.read_reg(MAILBOX_READ) };
            let msg_channel = (msg & 0xF) as u8;
            
            // Check if this message is for our channel
            if msg_channel == channel as u8 {
                return msg & 0xFFFFFFF0;
            }
        }
    }

    /// Get status register value
    pub fn status(&self) -> u32 {
        unsafe { self.read_reg(MAILBOX_STATUS) }
    }
}

// Make it Sync for static usage
unsafe impl Sync for MailboxRegisters {}
