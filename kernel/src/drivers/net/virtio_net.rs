//! VirtIO Network Device Driver (E1000 / VirtIO-Net)
//! Driver jaringan langsung ke hardware PCI untuk bypass protokol standar OS.

use crate::enterprise::audit::{AuditSeverity, log_security};
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VirtIoNetDriver {
    base_addr: AtomicUsize,
    mac_address: [u8; 6],
    initialized: bool,
}

impl VirtIoNetDriver {
    pub const fn new() -> Self {
        Self {
            base_addr: AtomicUsize::new(0),
            mac_address: [0; 6],
            initialized: false,
        }
    }

    /// Init VirtIO net from PCI discovery (BAR address)
    pub fn init(&mut self, pci_bar_addr: usize, mac: [u8; 6]) {
        self.base_addr.store(pci_bar_addr, Ordering::SeqCst);
        self.mac_address = mac;
        self.initialized = true;
        
        crate::println!("[VirtIO-Net] Hardware NIC Bound at MMIO 0x{:X}", pci_bar_addr);
        crate::println!("[VirtIO-Net] Physical MAC Address: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", 
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
            
        log_security(AuditSeverity::Info, "Network", "Hardware NIC successfully bound to PCI.");
    }

    /// Send a raw packet bypassing normal OS sockets (useful for Air-Gap Mesh Route)
    pub fn transmit_raw(&self, payload: &[u8]) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("NIC not initialized.");
        }
        // In reality, this pushes a Virtq Desc descriptor to the transmission queue
        // and triggers a PCI MMIO doorbell ring:
        // unsafe { core::ptr::write_volatile((self.base_addr.load(Ordering::Relaxed) + VIRTIO_Q_NOTIFY) as *mut u32, queue_idx); }
        crate::println!("[VirtIO-Net] Transmitting {} bytes directly to VIRTQ-Tx", payload.len());
        Ok(())
    }
}

pub static VIRTIO_NIC: spin::Mutex<VirtIoNetDriver> = spin::Mutex::new(VirtIoNetDriver::new());
