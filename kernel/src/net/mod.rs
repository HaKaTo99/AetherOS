//! Network Stack Wrapper
//! using smoltcp (TCP/IP)

pub mod driver;
pub mod loopback;
pub mod discovery; // Device Discovery Protocol
pub mod virtio_net; // [NEW] VirtIO-net driver (Phase 12.1)
pub mod bcm_genet;  // [NEW] BCM GENET RPi4 ethernet (Phase 12.1)
pub mod dhcp;       // [NEW] DHCP client (Phase 12.1)

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use loopback::LoopbackDevice;



pub struct NetworkStack {
    pub interface: Interface,
    pub sockets: SocketSet<'static>,
    pub device: LoopbackDevice,
}

impl NetworkStack {
    /// Initialize the network stack with loopback device
    pub fn new() -> Self {
        // Initialize loopback device
        let mut device = LoopbackDevice::new();
        
        // Configure interface
        let mac_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
        let config = Config::new(mac_addr.into());
        let mut interface = Interface::new(config, &mut device, Instant::from_millis(0));
        
        // Assign IP address (127.0.0.1/8 for loopback)
        interface.update_ip_addrs(|ip_addrs| {
            ip_addrs.push(IpCidr::new(
                IpAddress::Ipv4(Ipv4Address::new(127, 0, 0, 1)),
                8,
            )).ok();
        });
        
        // Create socket set
        let sockets = SocketSet::new(vec![]);
        
        Self {
            interface,
            sockets,
            device,
        }
    }
    
    /// Poll the network stack (call from scheduler loop)
    pub fn poll(&mut self, timestamp_ms: i64) {
        let timestamp = Instant::from_millis(timestamp_ms);
        let _ = self.interface.poll(timestamp, &mut self.device, &mut self.sockets);
    }
}

