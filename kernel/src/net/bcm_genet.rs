//! BCM GENET Driver Stub (Phase 12.1)
//! Raspberry Pi 4 Ethernet controller

use crate::net::driver::{NetworkDriver, NetResult, NetError};
use alloc::vec::Vec;
use alloc::collections::VecDeque;

/// BCM GENET register block
const GENET_BASE: usize = 0xFD58_0000; // RPi4 BCM54213PE

/// GENET System registers
const SYS_REV_CTRL: usize = 0x00;
const SYS_PORT_CTRL: usize = 0x04;
const UMAC_CMD: usize = 0x808;
const UMAC_MAC0: usize = 0x80C;
const UMAC_MAC1: usize = 0x810;

/// BCM GENET Ethernet Driver
pub struct BcmGenet {
    base_addr: usize,
    mac: [u8; 6],
    rx_queue: VecDeque<Vec<u8>>,
    tx_queue: VecDeque<Vec<u8>>,
    initialized: bool,
    link_up: bool,
}

impl BcmGenet {
    pub const fn new() -> Self {
        Self {
            base_addr: GENET_BASE,
            mac: [0xDC, 0xA6, 0x32, 0x00, 0x00, 0x01], // RPi default prefix
            rx_queue: VecDeque::new(),
            tx_queue: VecDeque::new(),
            initialized: false,
            link_up: false,
        }
    }

    fn read_reg(&self, _offset: usize) -> u32 {
        // Would use volatile_load in real implementation
        0
    }

    fn write_reg(&self, _offset: usize, _value: u32) {
        // Would use volatile_store in real implementation
    }

    /// Check PHY link status
    pub fn link_status(&self) -> bool {
        self.link_up
    }
}

impl NetworkDriver for BcmGenet {
    fn init(&mut self) -> NetResult<()> {
        // 1. Read revision
        let _rev = self.read_reg(SYS_REV_CTRL);

        // 2. Reset UMAC
        self.write_reg(UMAC_CMD, 0);

        // 3. Clear MIB counters
        // ...

        // 4. Set MAC address
        let mac_hi = ((self.mac[0] as u32) << 24)
            | ((self.mac[1] as u32) << 16)
            | ((self.mac[2] as u32) << 8)
            | (self.mac[3] as u32);
        let mac_lo = ((self.mac[4] as u32) << 8) | (self.mac[5] as u32);
        self.write_reg(UMAC_MAC0, mac_hi);
        self.write_reg(UMAC_MAC1, mac_lo);

        // 5. Enable RX/TX DMA (simplified)
        self.write_reg(SYS_PORT_CTRL, 0);

        // 6. Enable UMAC
        self.write_reg(UMAC_CMD, 0x03); // TX_EN | RX_EN

        self.initialized = true;
        Ok(())
    }

    fn can_transmit(&self) -> bool {
        self.initialized && self.link_up
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
