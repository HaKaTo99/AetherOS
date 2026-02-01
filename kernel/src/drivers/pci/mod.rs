//! PCI Bus Driver
//! Handles enumeration and configuration of PCI devices

use core::arch::asm;
// use crate::hal::get_platform;

const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
}

/// Read a 32-bit word from PCI Config Space
pub unsafe fn pci_read_32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = (1u32 << 31) | // Enable bit
                  ((bus as u32) << 16) |
                  ((slot as u32) << 11) |
                  ((func as u32) << 8) |
                  ((offset as u32) & 0xFC);

    outl(PCI_CONFIG_ADDRESS, address);
    inl(PCI_CONFIG_DATA)
}

/// Read a 16-bit word from PCI Config Space
pub unsafe fn pci_read_16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let raw = pci_read_32(bus, slot, func, offset);
    let shift = (offset & 2) * 8;
    ((raw >> shift) & 0xFFFF) as u16
}

/// Check if a specific device exists
pub unsafe fn check_device(bus: u8, device: u8) -> Option<PciDevice> {
    let vendor_id = pci_read_16(bus, device, 0, 0);
    if vendor_id == 0xFFFF {
        return None; // Device doesn't exist
    }
    
    let device_id = pci_read_16(bus, device, 0, 2);
    
    Some(PciDevice {
        bus,
        device,
        function: 0,
        vendor_id,
        device_id,
    })
}

/// Brute-force scan of all PCI buses
pub fn scan_pci_bus() -> [Option<PciDevice>; 32] {
    let mut devices = [None; 32];
    let mut count = 0;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        for bus in 0..255 {
            for slot in 0..32 {
                if let Some(dev) = check_device(bus, slot) {
                    if count < 32 {
                        devices[count] = Some(dev);
                        count += 1;
                    }
                }
            }
        }
    }
    
    // On non-x86, this is a no-op for now unless we implement MMIO PCI
    
    devices
}

// Helper IO for x86 (Should ideally be in HAL, but duplicating here for driver self-containment/simplicity in phase 3)
#[cfg(target_arch = "x86_64")]
unsafe fn outl(port: u16, val: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") val, options(nostack, nomem, preserves_flags));
}

#[cfg(target_arch = "x86_64")]
unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    asm!("in eax, dx", out("eax") ret, in("dx") port, options(nostack, nomem, preserves_flags));
    ret
}

// Dummy impls for non-x86 to allow compilation
#[cfg(not(target_arch = "x86_64"))]
unsafe fn outl(_port: u16, _val: u32) {}
#[cfg(not(target_arch = "x86_64"))]
unsafe fn inl(_port: u16) -> u32 { 0 }
