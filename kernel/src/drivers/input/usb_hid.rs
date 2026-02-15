//! USB HID Driver (Phase 13.2)

use super::{InputEvent, KeyCode, KeyState};

/// HID device types
#[derive(Debug, Clone, Copy)]
pub enum HidDeviceType {
    Keyboard,
    Mouse,
    Gamepad,
    TouchScreen,
}

/// USB HID Device
pub struct UsbHidDevice {
    pub device_type: HidDeviceType,
    pub vendor_id: u16,
    pub product_id: u16,
    connected: bool,
}

impl UsbHidDevice {
    pub fn new(device_type: HidDeviceType, vid: u16, pid: u16) -> Self {
        Self { device_type, vendor_id: vid, product_id: pid, connected: false }
    }

    pub fn connect(&mut self) { self.connected = true; }
    pub fn disconnect(&mut self) { self.connected = false; }
    pub fn is_connected(&self) -> bool { self.connected }

    pub fn poll(&self) -> Option<InputEvent> {
        if !self.connected { return None; }
        // Would read from USB endpoint
        None
    }
}
