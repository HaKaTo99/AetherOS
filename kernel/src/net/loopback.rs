//! Loopback Network Device
//! Simple loopback implementation for testing

use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

pub struct LoopbackDevice {
    queue: Mutex<VecDeque<Vec<u8>>>,
}

impl LoopbackDevice {
    pub const fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Inject a packet into the receive queue (for testing)
    pub fn inject(&self, packet: Vec<u8>) {
        self.queue.lock().push_back(packet);
    }
}

pub struct LoopbackRxToken {
    buffer: Vec<u8>,
}

pub struct LoopbackTxToken<'a> {
    queue: &'a Mutex<VecDeque<Vec<u8>>>,
}

impl RxToken for LoopbackRxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.buffer[..])
    }
}

impl<'a> TxToken for LoopbackTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = Vec::with_capacity(len);
        buffer.resize(len, 0);
        let result = f(&mut buffer[..]);
        
        // Loopback: push to rx queue
        self.queue.lock().push_back(buffer);
        result
    }
}

impl Device for LoopbackDevice {
    type RxToken<'a> = LoopbackRxToken;
    type TxToken<'a> = LoopbackTxToken<'a>;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut queue = self.queue.lock();
        if let Some(buffer) = queue.pop_front() {
            Some((
                LoopbackRxToken { buffer },
                LoopbackTxToken { queue: &self.queue },
            ))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(LoopbackTxToken { queue: &self.queue })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1500;
        caps.max_burst_size = Some(1);
        caps
    }
}
