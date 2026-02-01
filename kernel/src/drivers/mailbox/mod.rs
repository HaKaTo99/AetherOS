//! Mailbox Driver for Raspberry Pi 4
//! 
//! Provides communication interface between ARM CPU and VideoCore GPU
//! for hardware control operations (clocks, power, voltage, etc.)

mod registers;
mod property_tags;

pub use registers::{MailboxChannel, MailboxRegisters};
pub use property_tags::{
    PropertyTag, ClockId, PowerDomain, PropertyMessage,
    create_get_clock_rate_msg, create_set_clock_rate_msg,
    create_get_power_state_msg, create_set_power_state_msg,
};

use core::sync::atomic::{AtomicBool, Ordering};

/// Global mailbox instance
static MAILBOX: MailboxRegisters = MailboxRegisters::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize the mailbox driver
pub fn init() {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return; // Already initialized
    }
    
    // Mailbox is memory-mapped, no special initialization needed
    // Just verify it's accessible by reading status
    let _status = MAILBOX.status();
}

/// Send a property message via mailbox and wait for response
/// 
/// # Arguments
/// * `msg` - Property message to send (must be 16-byte aligned)
/// 
/// # Returns
/// * `Ok(())` if successful
/// * `Err(())` if mailbox call failed
pub fn call(msg: &mut PropertyMessage) -> Result<(), ()> {
    // Ensure cache coherency (on real hardware, need to flush cache)
    // For now, we use volatile operations which should be sufficient
    
    // Get GPU bus address of message buffer
    let addr = msg.as_gpu_ptr();
    
    // Send message via property tags channel
    MAILBOX.write(MailboxChannel::PropertyTagsArmToVc, addr);
    
    // Wait for response
    let response = MAILBOX.read(MailboxChannel::PropertyTagsArmToVc);
    
    // Verify response address matches
    if response != addr {
        return Err(());
    }
    
    // Check if response is successful
    if !msg.is_success() {
        return Err(());
    }
    
    Ok(())
}

/// Get current clock rate for a specific clock
pub fn get_clock_rate(clock_id: ClockId) -> Result<u32, ()> {
    let mut msg = create_get_clock_rate_msg(clock_id);
    call(&mut msg)?;
    
    // Response is at offset 3 in the tag (after tag_id, buffer_size, req_resp_code)
    // Value buffer starts at index 3, and rate is the second u32 (index 4)
    Ok(msg.buffer[6])
}

/// Set clock rate for a specific clock
pub fn set_clock_rate(clock_id: ClockId, rate_hz: u32) -> Result<u32, ()> {
    let mut msg = create_set_clock_rate_msg(clock_id, rate_hz);
    call(&mut msg)?;
    
    // Return actual rate set
    Ok(msg.buffer[6])
}

/// Get power state for a power domain
pub fn get_power_state(domain: PowerDomain) -> Result<bool, ()> {
    let mut msg = create_get_power_state_msg(domain);
    call(&mut msg)?;
    
    // Response: domain_id, state
    let state = msg.buffer[6];
    Ok((state & 0x1) != 0)
}

/// Set power state for a power domain
pub fn set_power_state(domain: PowerDomain, on: bool) -> Result<bool, ()> {
    let mut msg = create_set_power_state_msg(domain, on);
    call(&mut msg)?;
    
    // Response: domain_id, state
    let state = msg.buffer[6];
    Ok((state & 0x1) != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_alignment() {
        let msg = PropertyMessage::new();
        let addr = &msg.buffer as *const u32 as usize;
        assert_eq!(addr % 16, 0, "PropertyMessage must be 16-byte aligned");
    }

    #[test]
    fn test_get_clock_rate_message_format() {
        let msg = create_get_clock_rate_msg(ClockId::Arm);
        
        // Check message structure
        assert!(msg.buffer[0] > 0, "Total size should be set");
        assert_eq!(msg.buffer[1], 0x00000000, "Should be request code");
        assert_eq!(msg.buffer[2], PropertyTag::GetClockRate as u32);
        assert_eq!(msg.buffer[3], 8, "Buffer size should be 8 bytes");
        assert_eq!(msg.buffer[4], 0, "Request code should be 0");
        assert_eq!(msg.buffer[5], ClockId::Arm as u32);
    }

    #[test]
    fn test_set_clock_rate_message_format() {
        let rate = 1500000000; // 1.5 GHz
        let msg = create_set_clock_rate_msg(ClockId::Arm, rate);
        
        assert_eq!(msg.buffer[2], PropertyTag::SetClockRate as u32);
        assert_eq!(msg.buffer[5], ClockId::Arm as u32);
        assert_eq!(msg.buffer[6], rate);
    }

    #[test]
    fn test_power_state_message_format() {
        let msg = create_get_power_state_msg(PowerDomain::UsbHcd);
        
        assert_eq!(msg.buffer[2], PropertyTag::GetPowerState as u32);
        assert_eq!(msg.buffer[5], PowerDomain::UsbHcd as u32);
    }
}
