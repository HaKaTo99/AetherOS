pub mod serial;
pub mod irq;
pub mod timer;
pub mod dtb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Serial,
    InterruptController,
    Timer,
    BlockDevice,
    Network,
    Unknown,
}

/// Core trait that all drivers must implement
pub trait Driver: Sync + Send {
    /// Device compatibility string (e.g. "arm,pl011")
    fn compatible(&self) -> &str;
    
    /// Initialize the device
    /// Returns Ok if success, Err if failed
    unsafe fn init(&mut self) -> Result<(), &'static str>;
    
    /// Return the driver type
    fn device_type(&self) -> DriverType;
    
    /// Handle an interrupt (optional)
    fn handle_irq(&mut self, _irq_num: u32) {
        // Default implementation does nothing
    }
}

/// Simple static driver manager
pub struct DriverManager {
    // For now, simple fixed slots. In future with alloc, use Vec<Box<dyn Driver>>.
    // Making this generic is tricky with static mut without alloc.
    // For Phase 2.1, we'll keep hardcoded slots hidden behind this interface
    // to allow easy refactoring later.
}

// Global instances
// Global instances (ARM)
#[cfg(target_arch = "aarch64")]
static mut PL011: serial::pl011::Pl011Uart = serial::pl011::Pl011Uart::new(0xFE201000);
#[cfg(target_arch = "aarch64")]
static mut GIC: irq::gic400::Gic400 = irq::gic400::Gic400::new(0xFF841000, 0xFF842000);
#[cfg(target_arch = "aarch64")]
static mut TIMER: timer::arm_generic::ArmGenericTimer = timer::arm_generic::ArmGenericTimer::new();

impl DriverManager {
    pub unsafe fn init(dtb_ptr: usize) {
        #[cfg(target_arch = "aarch64")]
        {
            // If DTB is provided, try to discover devices
            if dtb_ptr != 0 {
                use crate::drivers::dtb::{DeviceTree, DtbItem};
                if let Some(dt) = DeviceTree::from_raw(dtb_ptr as *const u8) {
                    for item in dt.nodes() {
                        if let DtbItem::Property { name, value } = item {
                            if name == "compatible" {
                                // Check for GIC
                                if contains(value, "arm,gic-400") || contains(value, "arm,cortex-a15-gic") {
                                    let _ = GIC.init();
                                }
                                // Check for PL011
                                if contains(value, "arm,pl011") || contains(value, "arm,primecell") {
                                     let _ = PL011.init();
                                }
                                // Check for Generic Timer
                                if contains(value, "arm,armv8-timer") || contains(value, "arm,armv7-timer") {
                                     let _ = TIMER.init();
                                }
                            }
                        }
                    }
                }
            } else {
                // Fallback to static init if no DTB (e.g. static RPi compile)
                // Initialize IRQ Controller first
                if let Err(_e) = GIC.init() {
                     // Panic or log failure
                }
                
                // Initialize Serial
                if let Err(_e) = PL011.init() {
                     // Panic
                }
                
                // Initialize Timer
                if let Err(_e) = TIMER.init() {
                     // Panic
                }
            }
        }
    }
}

// Helper to check if byte slice contains string
fn contains(haystack: &[u8], needle: &str) -> bool {
    // Very basic substring check for now
    let needle_bytes = needle.as_bytes();
    if haystack.len() < needle_bytes.len() { return false; }
    
    for i in 0..=(haystack.len() - needle_bytes.len()) {
            if &haystack[i..i+needle_bytes.len()] == needle_bytes {
                return true;
            }
    }
    false
}



impl DriverManager {
    #[cfg(target_arch = "aarch64")]
    pub fn get_serial() -> &'static mut impl Driver {
        unsafe { &mut PL011 }
    }

    #[cfg(target_arch = "aarch64")]
    pub fn get_irq_controller() -> &'static mut impl Driver {
        unsafe { &mut GIC }
    }

    #[cfg(target_arch = "aarch64")]
    pub fn get_timer() -> &'static mut impl Driver {
         unsafe { &mut TIMER }
    }
}

