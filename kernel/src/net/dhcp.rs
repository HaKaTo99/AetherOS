//! DHCP Client Stub (Phase 12.1)
//! Dynamic Host Configuration Protocol

use alloc::string::String;

/// DHCP lease information
#[derive(Debug, Clone)]
pub struct DhcpLease {
    pub ip_addr: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub gateway: [u8; 4],
    pub dns: [u8; 4],
    pub lease_time_secs: u32,
}

/// DHCP client state machine
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DhcpState {
    Init,
    Selecting,
    Requesting,
    Bound,
    Renewing,
    Rebinding,
}

/// DHCP Client
pub struct DhcpClient {
    state: DhcpState,
    lease: Option<DhcpLease>,
    _transaction_id: u32,
}

impl DhcpClient {
    pub const fn new() -> Self {
        Self {
            state: DhcpState::Init,
            lease: None,
            _transaction_id: 0x12345678,
        }
    }

    /// Start DHCP discovery
    pub fn discover(&mut self) -> &[u8] {
        self.state = DhcpState::Selecting;
        // Would construct DHCP DISCOVER packet
        &[]
    }

    /// Process DHCP offer
    pub fn process_offer(&mut self, _data: &[u8]) {
        self.state = DhcpState::Requesting;
        // Would parse DHCP OFFER and send REQUEST
    }

    /// Process DHCP acknowledgment
    pub fn process_ack(&mut self, _data: &[u8]) {
        self.state = DhcpState::Bound;
        self.lease = Some(DhcpLease {
            ip_addr: [192, 168, 1, 100],
            subnet_mask: [255, 255, 255, 0],
            gateway: [192, 168, 1, 1],
            dns: [8, 8, 8, 8],
            lease_time_secs: 86400,
        });
    }

    /// Get current state
    pub fn state(&self) -> DhcpState {
        self.state
    }

    /// Get current lease
    pub fn lease(&self) -> Option<&DhcpLease> {
        self.lease.as_ref()
    }
}
