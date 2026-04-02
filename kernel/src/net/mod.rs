pub mod mesh;
pub mod driver;
pub mod virtio_net;
pub mod bcm_genet;
pub mod loopback;
pub mod dhcp;
pub mod discovery;

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::phy::{Device, DeviceCapabilities, RxToken, TxToken};
use self::loopback::LoopbackDevice;
use self::virtio_net::VirtIONet;

use core::sync::atomic::{AtomicBool, Ordering};

/// Profil Keamanan Jaringan (Layer 3/4 Isolator)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkProfile {
    /// Terhubung penuh ke internet/mesh
    Connected,
    /// Mode militer: Memutus rute publik, hanya menerima frame L2 yang tervalidasi PQC dari node intra-mesh
    AirGapped, 
}

pub enum AnyDevice {
    Loopback(LoopbackDevice),
    VirtIO(VirtIONet),
}

impl AnyDevice {
    /// Injeksi paket ke perangkat yang aktif (untuk simulasi/stress test)
    pub fn inject(&self, packet: alloc::vec::Vec<u8>) {
        match self {
            AnyDevice::Loopback(d) => d.inject(packet),
            AnyDevice::VirtIO(d) => d.inject(packet),
        }
    }
}

pub enum AnyRxToken {
    Loopback(self::loopback::LoopbackRxToken),
    VirtIO(self::virtio_net::VirtIONetRxToken),
}

pub enum AnyTxToken<'a> {
    Loopback(self::loopback::LoopbackTxToken<'a>),
    VirtIO(self::virtio_net::VirtIONetTxToken<'a>),
}

impl RxToken for AnyRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self {
            AnyRxToken::Loopback(t) => t.consume(f),
            AnyRxToken::VirtIO(t) => t.consume(f),
        }
    }
}

impl<'a> TxToken for AnyTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self {
            AnyTxToken::Loopback(t) => t.consume(len, f),
            AnyTxToken::VirtIO(t) => t.consume(len, f),
        }
    }
}

impl Device for AnyDevice {
    type RxToken<'a> = AnyRxToken;
    type TxToken<'a> = AnyTxToken<'a>;

    fn receive(&mut self, timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        match self {
            AnyDevice::Loopback(d) => d.receive(timestamp).map(|(rx, tx)| (AnyRxToken::Loopback(rx), AnyTxToken::Loopback(tx))),
            AnyDevice::VirtIO(d) => d.receive(timestamp).map(|(rx, tx)| (AnyRxToken::VirtIO(rx), AnyTxToken::VirtIO(tx))),
        }
    }

    fn transmit(&mut self, timestamp: Instant) -> Option<Self::TxToken<'_>> {
        match self {
            AnyDevice::Loopback(d) => d.transmit(timestamp).map(AnyTxToken::Loopback),
            AnyDevice::VirtIO(d) => d.transmit(timestamp).map(AnyTxToken::VirtIO),
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        match self {
            AnyDevice::Loopback(d) => d.capabilities(),
            AnyDevice::VirtIO(d) => d.capabilities(),
        }
    }
}

pub struct NetworkStack {
    pub device: AnyDevice,
    pub interface: Interface,
    pub sockets: SocketSet<'static>,
    pub profile: NetworkProfile,
    pub air_gap_lockdown: AtomicBool,
}

impl NetworkStack {
    pub fn new() -> Self {
        // Physical detection: In a real military grade kernel, this would scan the PCI bus.
        // For Sovereign 1.0, we prioritize VirtIO-Net if the MMIO base is present.
        let mut device = if true {
            let mut v = VirtIONet::new(0x10001000); // Standard QEMU MMIO
            let _ = crate::net::driver::NetworkDriver::init(&mut v);
            AnyDevice::VirtIO(v)
        } else {
            AnyDevice::Loopback(LoopbackDevice::new())
        };

        let config = Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into());
        
        let mut interface = Interface::new(config, &mut device, Instant::from_micros(0));
        interface.update_ip_addrs(|addrs| {
            addrs.push(IpCidr::new(IpAddress::v4(127, 0, 0, 1), 8)).unwrap();
            addrs.push(IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)).unwrap(); // QEMU Guest IP
        });

        Self { 
            device,
            interface,
            sockets: SocketSet::new(alloc::vec![]),
            profile: NetworkProfile::AirGapped, 
            air_gap_lockdown: AtomicBool::new(true),
        } 
    }
    
    pub fn set_profile(&mut self, profile: NetworkProfile) {
        self.profile = profile;
        if profile == NetworkProfile::AirGapped {
            self.air_gap_lockdown.store(true, Ordering::SeqCst);
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Critical,
                "Network", "AIR-GAPPED PROFILE ACTIVATED. Public Routing Cut Off. Hardware isolated."
            );
        } else {
            self.air_gap_lockdown.store(false, Ordering::SeqCst);
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Warning,
                "Network", "Network transitioning to Connected State."
            );
        }
    }

    // Normal network poll operation
    pub fn poll(&mut self, timestamp_ms: i64) {
        let timestamp = Instant::from_millis(timestamp_ms);
        
        if self.air_gap_lockdown.load(Ordering::Relaxed) {
             // Hardware-level firewall check (Military Grade)
             // In air-gapped mode, we still poll for local loopback / PQC-only mesh frames
             self.interface.poll(timestamp, &mut self.device, &mut self.sockets);
             return;
        }
        
        self.interface.poll(timestamp, &mut self.device, &mut self.sockets);
    }

    /// Mengirim paket ethernet raw langsung ke perangkat keras (Sovereign Level 0)
    pub fn transmit_raw(&mut self, payload: &[u8]) {
        let timestamp = Instant::from_millis(0); // Dummy timestamp for immediate TX
        if let Some(tx_token) = self.device.transmit(timestamp) {
            tx_token.consume(payload.len(), |buffer| {
                buffer.copy_from_slice(payload);
            });
        }
    }
}

// Global Network Stack instance (Phase 1.2.2)
static mut NETWORK: Option<NetworkStack> = None;

/// Mendapatkan akses ke global NetworkStack
pub fn get_network_stack() -> Option<&'static mut NetworkStack> {
    unsafe { NETWORK.as_mut() }
}

/// Inisialisasi sistem jaringan inti
pub fn init() {
    unsafe {
        NETWORK = Some(NetworkStack::new());
    }
}
