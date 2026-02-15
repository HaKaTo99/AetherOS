//! Network Driver Abstraction (Phase 12.1)
//! Hardware-agnostic interface for network devices

use alloc::vec::Vec;

/// Network driver errors
#[derive(Debug, Clone, Copy)]
pub enum NetError {
    NotReady,
    TxBufferFull,
    RxBufferEmpty,
    InvalidPacket,
}

pub type NetResult<T> = Result<T, NetError>;

/// Network Driver Trait
/// All devices (Loopback, VirtIO, GENET) implement this
pub trait NetworkDriver: Send {
    fn init(&mut self) -> NetResult<()>;
    fn can_transmit(&self) -> bool;
    fn can_receive(&self) -> bool;
    fn transmit(&mut self, packet: &[u8]) -> NetResult<()>;
    fn receive(&mut self) -> NetResult<Vec<u8>>;
    fn mac_address(&self) -> [u8; 6];
    fn mtu(&self) -> usize { 1500 }
}
