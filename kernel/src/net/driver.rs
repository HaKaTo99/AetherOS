//! Network Driver Traits

use alloc::vec::Vec;

pub trait NetworkDriver {
    fn transmit(&mut self, packet: &[u8]);
    fn receive(&mut self) -> Option<Vec<u8>>;
}
