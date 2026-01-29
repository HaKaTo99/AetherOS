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
static mut PL011: serial::pl011::Pl011Uart = serial::pl011::Pl011Uart::new(0xFE201000);
static mut GIC: irq::gic400::Gic400 = irq::gic400::Gic400::new(0xFF841000, 0xFF842000);
static mut TIMER: timer::arm_generic::ArmGenericTimer = timer::arm_generic::ArmGenericTimer::new();

impl DriverManager {
    pub unsafe fn init() {
        // Initialize IRQ Controller first
        if let Err(e) = GIC.init() {
             // Panic or log failure
        }
        
        // Initialize Serial
        if let Err(e) = PL011.init() {
             // Panic
        }
        
        // Initialize Timer
        if let Err(e) = TIMER.init() {
             // Panic
        }
    }

    pub fn get_serial() -> &'static mut impl Driver {
        unsafe { &mut PL011 }
    }

     pub fn get_irq_controller() -> &'static mut impl Driver {
        unsafe { &mut GIC }
    }

    pub fn get_timer() -> &'static mut impl Driver {
         unsafe { &mut TIMER }
    }
}

