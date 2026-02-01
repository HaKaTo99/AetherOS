//! Network Stack Wrapper
//! using smoltcp (TCP/IP)

pub mod driver;
pub mod loopback;
pub mod discovery; // Device Discovery Protocol

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use loopback::LoopbackDevice;

static mut LOOPBACK_DEVICE: Option<LoopbackDevice> = None;

pub struct NetworkStack<'a> {
    pub interface: Interface,
    pub sockets: SocketSet<'a>,
    pub device: &'a mut LoopbackDevice,
}

impl<'a> NetworkStack<'a> {
    /// Initialize the network stack with loopback device
    /// 
    /// # Safety
    /// Must be called only once during kernel initialization
    pub unsafe fn init() -> Self {
        // Initialize loopback device
        LOOPBACK_DEVICE = Some(LoopbackDevice::new());
        let device = LOOPBACK_DEVICE.as_mut().unwrap();
        
        // Configure interface
        let mac_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
        let config = Config::new(mac_addr.into());
        let mut interface = Interface::new(config, device, Instant::from_millis(0));
        
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
        let _ = self.interface.poll(timestamp, self.device, &mut self.sockets);
    }
}

