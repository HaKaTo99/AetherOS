//! VirtIO Network Driver Stub (Phase 12.1)
//! Clean-room implementation for QEMU/Cloud

use crate::net::driver::{NetworkDriver, NetResult, NetError};
use alloc::vec::Vec;
use alloc::collections::VecDeque;

/// VirtIO-net device registers (MMIO offsets)
#[repr(C)]
pub struct VirtIONetRegs {
    pub magic: u32,          // 0x00: Magic value (0x74726976)
    pub version: u32,        // 0x04: Version (2)
    pub device_id: u32,      // 0x08: Device ID (1 for network)
    pub vendor_id: u32,      // 0x0C: Vendor ID
    pub status: u32,         // 0x70: Status register
}

/// VirtIO Network Device
pub struct VirtIONet {
    base_addr: usize,
    mac: [u8; 6],
    rx_queue: VecDeque<Vec<u8>>,
    tx_queue: VecDeque<Vec<u8>>,
    initialized: bool,
}

impl VirtIONet {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            mac: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56], // QEMU default
            rx_queue: VecDeque::new(),
            tx_queue: VecDeque::new(),
            initialized: false,
        }
    }

    fn read_reg(&self, _offset: usize) -> u32 {
        // MMIO read - would use volatile_load in real impl
        0
    }

    fn write_reg(&self, _offset: usize, _value: u32) {
        // MMIO write - would use volatile_store in real impl
    }
}

impl NetworkDriver for VirtIONet {
    fn init(&mut self) -> NetResult<()> {
        // 1. Check magic number
        let magic = self.read_reg(0x00);
        if magic != 0 && magic != 0x74726976 {
            return Err(NetError::NotReady);
        }

        // 2. Set ACKNOWLEDGE status bit
        self.write_reg(0x70, 1);

        // 3. Set DRIVER status bit
        self.write_reg(0x70, 3);

        // 4. Read features, negotiate
        // Simplified: accept all features

        // 5. Set FEATURES_OK
        self.write_reg(0x70, 11);

        // 6. Set DRIVER_OK
        self.write_reg(0x70, 15);

        self.initialized = true;
        Ok(())
    }

    fn can_transmit(&self) -> bool {
        self.initialized
    }

    fn can_receive(&self) -> bool {
        !self.rx_queue.is_empty()
    }

    fn transmit(&mut self, packet: &[u8]) -> NetResult<()> {
        if !self.initialized {
            return Err(NetError::NotReady);
        }
        self.tx_queue.push_back(packet.to_vec());
        Ok(())
    }

    fn receive(&mut self) -> NetResult<Vec<u8>> {
        self.rx_queue.pop_front().ok_or(NetError::RxBufferEmpty)
    }

    fn mac_address(&self) -> [u8; 6] {
        self.mac
    }
}
