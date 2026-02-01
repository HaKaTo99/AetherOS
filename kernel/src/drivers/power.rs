//! Power Domain Management
//! 
//! Controls power states for various hardware peripherals via mailbox interface

use crate::drivers::mailbox::{self, PowerDomain as MailboxPowerDomain};

/// Power domain controller
pub struct PowerDomainController {
    initialized: bool,
}

impl PowerDomainController {
    /// Create a new power domain controller
    pub const fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    /// Initialize power domain controller
    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        mailbox::init();
        self.initialized = true;
    }

    /// Enable a power domain
    pub fn enable(&self, domain: PowerDomain) -> Result<(), ()> {
        let mailbox_domain = domain.to_mailbox_domain();
        mailbox::set_power_state(mailbox_domain, true)?;
        Ok(())
    }

    /// Disable a power domain
    pub fn disable(&self, domain: PowerDomain) -> Result<(), ()> {
        let mailbox_domain = domain.to_mailbox_domain();
        mailbox::set_power_state(mailbox_domain, false)?;
        Ok(())
    }

    /// Get power state of a domain
    pub fn get_state(&self, domain: PowerDomain) -> Result<bool, ()> {
        let mailbox_domain = domain.to_mailbox_domain();
        mailbox::get_power_state(mailbox_domain)
    }

    /// Check if a domain is powered on
    pub fn is_powered(&self, domain: PowerDomain) -> bool {
        self.get_state(domain).unwrap_or(false)
    }
}

/// Power domains available on RPi4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerDomain {
    SdCard,
    Uart0,
    Uart1,
    UsbHcd,
    I2c0,
    I2c1,
    I2c2,
    Spi,
    Ccp2tx,
}

impl PowerDomain {
    /// Convert to mailbox power domain
    fn to_mailbox_domain(self) -> MailboxPowerDomain {
        match self {
            PowerDomain::SdCard => MailboxPowerDomain::SdCard,
            PowerDomain::Uart0 => MailboxPowerDomain::Uart0,
            PowerDomain::Uart1 => MailboxPowerDomain::Uart1,
            PowerDomain::UsbHcd => MailboxPowerDomain::UsbHcd,
            PowerDomain::I2c0 => MailboxPowerDomain::I2c0,
            PowerDomain::I2c1 => MailboxPowerDomain::I2c1,
            PowerDomain::I2c2 => MailboxPowerDomain::I2c2,
            PowerDomain::Spi => MailboxPowerDomain::Spi,
            PowerDomain::Ccp2tx => MailboxPowerDomain::Ccp2tx,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            PowerDomain::SdCard => "SD Card",
            PowerDomain::Uart0 => "UART0",
            PowerDomain::Uart1 => "UART1",
            PowerDomain::UsbHcd => "USB Host Controller",
            PowerDomain::I2c0 => "I2C0",
            PowerDomain::I2c1 => "I2C1",
            PowerDomain::I2c2 => "I2C2",
            PowerDomain::Spi => "SPI",
            PowerDomain::Ccp2tx => "CCP2TX",
        }
    }
}

/// Global power domain controller
static mut POWER_CONTROLLER: PowerDomainController = PowerDomainController::new();

/// Initialize global power domain controller
pub fn init() {
    unsafe {
        POWER_CONTROLLER.init();
    }
}

/// Enable a power domain
pub fn enable(domain: PowerDomain) -> Result<(), ()> {
    unsafe { POWER_CONTROLLER.enable(domain) }
}

/// Disable a power domain
pub fn disable(domain: PowerDomain) -> Result<(), ()> {
    unsafe { POWER_CONTROLLER.disable(domain) }
}

/// Get power state of a domain
pub fn get_state(domain: PowerDomain) -> Result<bool, ()> {
    unsafe { POWER_CONTROLLER.get_state(domain) }
}

/// Check if a domain is powered on
pub fn is_powered(domain: PowerDomain) -> bool {
    unsafe { POWER_CONTROLLER.is_powered(domain) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_domain_names() {
        assert_eq!(PowerDomain::UsbHcd.name(), "USB Host Controller");
        assert_eq!(PowerDomain::SdCard.name(), "SD Card");
    }

    #[test]
    fn test_power_domain_conversion() {
        let domain = PowerDomain::UsbHcd;
        let mailbox_domain = domain.to_mailbox_domain();
        assert_eq!(mailbox_domain as u32, MailboxPowerDomain::UsbHcd as u32);
    }
}
