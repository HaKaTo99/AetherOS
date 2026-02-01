//! Mailbox Property Tags Interface
//! 
//! Property tags are used to communicate with VideoCore GPU for various
//! hardware operations like clock management, power control, etc.

/// Property tag IDs
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PropertyTag {
    // Clock tags
    GetClockRate = 0x00030002,
    SetClockRate = 0x00038002,
    GetClockRateMeasured = 0x00030047,
    GetMaxClockRate = 0x00030004,
    GetMinClockRate = 0x00030007,
    
    // Power tags
    GetPowerState = 0x00020001,
    SetPowerState = 0x00028001,
    
    // Voltage tags
    GetVoltage = 0x00030003,
    SetVoltage = 0x00038003,
    
    // Temperature tags
    GetTemperature = 0x00030006,
    GetMaxTemperature = 0x0003000A,
}

/// Clock IDs for clock-related property tags
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ClockId {
    Reserved = 0,
    Emmc = 1,
    Uart = 2,
    Arm = 3,
    Core = 4,
    V3d = 5,
    H264 = 6,
    Isp = 7,
    Sdram = 8,
    Pixel = 9,
    Pwm = 10,
    Hevc = 11,
    Emmc2 = 12,
    M2mc = 13,
    PixelBvb = 14,
}

/// Power domain IDs
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PowerDomain {
    SdCard = 0,
    Uart0 = 1,
    Uart1 = 2,
    UsbHcd = 3,
    I2c0 = 4,
    I2c1 = 5,
    I2c2 = 6,
    Spi = 7,
    Ccp2tx = 8,
}

/// Request/Response codes
const REQUEST_CODE: u32 = 0x00000000;
const RESPONSE_SUCCESS: u32 = 0x80000000;
const RESPONSE_ERROR: u32 = 0x80000001;

/// Property message buffer (must be 16-byte aligned)
#[repr(C, align(16))]
pub struct PropertyMessage {
    pub buffer: [u32; 256],
    pub size: usize,
}

impl PropertyMessage {
    /// Create a new property message
    pub const fn new() -> Self {
        Self {
            buffer: [0; 256],
            size: 0,
        }
    }

    /// Initialize message with header
    pub fn init(&mut self) {
        self.buffer[0] = 0; // Total size (filled later)
        self.buffer[1] = REQUEST_CODE;
        self.size = 2;
    }

    /// Add a property tag to the message
    pub fn add_tag(&mut self, tag: PropertyTag, request_size: usize, response_size: usize) -> usize {
        let tag_start = self.size;
        
        self.buffer[self.size] = tag as u32;
        self.size += 1;
        
        // Buffer size (max of request and response)
        let buffer_size = core::cmp::max(request_size, response_size);
        self.buffer[self.size] = buffer_size as u32;
        self.size += 1;
        
        // Request/response code (0 for request)
        self.buffer[self.size] = 0;
        self.size += 1;
        
        // Reserve space for value buffer
        let value_start = self.size;
        self.size += (buffer_size + 3) / 4; // Round up to u32 boundary
        
        value_start
    }

    /// Finalize message (add end tag and set total size)
    pub fn finalize(&mut self) {
        // Add end tag
        self.buffer[self.size] = 0;
        self.size += 1;
        
        // Set total size in bytes
        self.buffer[0] = (self.size * 4) as u32;
    }

    /// Check if response is successful
    pub fn is_success(&self) -> bool {
        self.buffer[1] == RESPONSE_SUCCESS
    }

    /// Get buffer pointer (for mailbox call)
    pub fn as_ptr(&self) -> u32 {
        &self.buffer as *const u32 as usize as u32
    }

    /// Get buffer pointer with GPU bus address offset
    /// RPi4 uses 0xC0000000 offset for GPU access
    pub fn as_gpu_ptr(&self) -> u32 {
        let cpu_addr = &self.buffer as *const u32 as usize;
        // Convert to GPU bus address (add VC4 bus offset)
        (cpu_addr | 0xC0000000) as u32
    }
}

/// Helper: Create a "Get Clock Rate" message
pub fn create_get_clock_rate_msg(clock_id: ClockId) -> PropertyMessage {
    let mut msg = PropertyMessage::new();
    msg.init();
    
    let value_offset = msg.add_tag(PropertyTag::GetClockRate, 4, 8);
    msg.buffer[value_offset] = clock_id as u32;
    
    msg.finalize();
    msg
}

/// Helper: Create a "Set Clock Rate" message
pub fn create_set_clock_rate_msg(clock_id: ClockId, rate_hz: u32) -> PropertyMessage {
    let mut msg = PropertyMessage::new();
    msg.init();
    
    let value_offset = msg.add_tag(PropertyTag::SetClockRate, 12, 8);
    msg.buffer[value_offset] = clock_id as u32;
    msg.buffer[value_offset + 1] = rate_hz;
    msg.buffer[value_offset + 2] = 0; // Skip setting turbo
    
    msg.finalize();
    msg
}

/// Helper: Create a "Get Power State" message
pub fn create_get_power_state_msg(domain: PowerDomain) -> PropertyMessage {
    let mut msg = PropertyMessage::new();
    msg.init();
    
    let value_offset = msg.add_tag(PropertyTag::GetPowerState, 4, 8);
    msg.buffer[value_offset] = domain as u32;
    
    msg.finalize();
    msg
}

/// Helper: Create a "Set Power State" message
pub fn create_set_power_state_msg(domain: PowerDomain, on: bool) -> PropertyMessage {
    let mut msg = PropertyMessage::new();
    msg.init();
    
    let value_offset = msg.add_tag(PropertyTag::SetPowerState, 8, 8);
    msg.buffer[value_offset] = domain as u32;
    msg.buffer[value_offset + 1] = if on { 1 } else { 0 } | (1 << 1); // State | Wait bit
    
    msg.finalize();
    msg
}
