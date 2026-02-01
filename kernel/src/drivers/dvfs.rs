//! Dynamic Voltage and Frequency Scaling (DVFS) for ARM CPU
//! 
//! Provides CPU frequency scaling capabilities using Operating Performance Points (OPP)
//! parsed from Device Tree and controlled via mailbox interface.

use crate::drivers::mailbox::{self, ClockId};
use crate::drivers::dtb::{DeviceTree, DtbItem};

/// Operating Performance Point (frequency + voltage)
#[derive(Debug, Clone, Copy)]
pub struct Opp {
    pub freq_hz: u32,
    pub voltage_uv: u32,
}

/// CPU Frequency Scaler
pub struct CpuFrequency {
    current_freq: u32,
    available_opps: [Option<Opp>; 16],
    opp_count: usize,
}

impl CpuFrequency {
    /// Create a new CPU frequency scaler
    pub const fn new() -> Self {
        Self {
            current_freq: 0,
            available_opps: [None; 16],
            opp_count: 0,
        }
    }

    /// Initialize DVFS by parsing OPP table from Device Tree
    pub fn init(&mut self, dtb: Option<&DeviceTree>) {
        // Initialize mailbox
        mailbox::init();

        // Get current frequency from hardware
        if let Ok(freq) = mailbox::get_clock_rate(ClockId::Arm) {
            self.current_freq = freq;
        }

        // Parse OPP table from DTB if available
        if let Some(dt) = dtb {
            self.parse_opp_table(dt);
        }

        // If no OPP table found, use default frequencies
        if self.opp_count == 0 {
            self.set_default_opps();
        }
    }

    /// Parse OPP table from Device Tree
    fn parse_opp_table(&mut self, dtb: &DeviceTree) {
        let mut in_opp_table = false;
        let mut in_opp_node = false;
        let mut current_freq = 0u32;
        let mut current_voltage = 0u32;

        for item in dtb.nodes() {
            match item {
                DtbItem::BeginNode(name) => {
                    if name.starts_with("opp-table") || name.starts_with("operating-points") {
                        in_opp_table = true;
                    } else if in_opp_table && name.starts_with("opp") {
                        in_opp_node = true;
                        current_freq = 0;
                        current_voltage = 0;
                    }
                }
                DtbItem::EndNode => {
                    if in_opp_node {
                        // Add OPP if both freq and voltage are set
                        if current_freq > 0 && self.opp_count < 16 {
                            self.available_opps[self.opp_count] = Some(Opp {
                                freq_hz: current_freq,
                                voltage_uv: current_voltage,
                            });
                            self.opp_count += 1;
                        }
                        in_opp_node = false;
                    } else if in_opp_table {
                        in_opp_table = false;
                    }
                }
                DtbItem::Property { name, value } => {
                    if in_opp_node {
                        if name == "opp-hz" && value.len() >= 8 {
                            // Read 64-bit frequency (big-endian)
                            current_freq = u32::from_be_bytes([
                                value[4], value[5], value[6], value[7]
                            ]);
                        } else if name == "opp-microvolt" && value.len() >= 4 {
                            // Read 32-bit voltage (big-endian)
                            current_voltage = u32::from_be_bytes([
                                value[0], value[1], value[2], value[3]
                            ]);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Set default OPP values for RPi4 (if DTB parsing fails)
    fn set_default_opps(&mut self) {
        // Default frequencies for BCM2711 (RPi4)
        let default_freqs = [
            600_000_000,   // 600 MHz
            1_000_000_000, // 1.0 GHz
            1_500_000_000, // 1.5 GHz
            1_800_000_000, // 1.8 GHz (max)
        ];

        for (i, &freq) in default_freqs.iter().enumerate() {
            self.available_opps[i] = Some(Opp {
                freq_hz: freq,
                voltage_uv: 0, // Voltage control not available without DTB
            });
            self.opp_count += 1;
        }
    }

    /// Get current CPU frequency
    pub fn get_current(&self) -> u32 {
        self.current_freq
    }

    /// Get list of available frequencies
    pub fn list_available(&self) -> &[Option<Opp>] {
        &self.available_opps[..self.opp_count]
    }

    /// Set CPU frequency to the closest available OPP
    pub fn set_frequency(&mut self, target_hz: u32) -> Result<u32, ()> {
        // Find closest OPP
        let mut closest_opp: Option<Opp> = None;
        let mut min_diff = u32::MAX;

        for opp in self.available_opps.iter().take(self.opp_count).flatten() {
            let diff = if opp.freq_hz > target_hz {
                opp.freq_hz - target_hz
            } else {
                target_hz - opp.freq_hz
            };

            if diff < min_diff {
                min_diff = diff;
                closest_opp = Some(*opp);
            }
        }

        if let Some(opp) = closest_opp {
            // Set frequency via mailbox
            match mailbox::set_clock_rate(ClockId::Arm, opp.freq_hz) {
                Ok(actual_freq) => {
                    self.current_freq = actual_freq;
                    Ok(actual_freq)
                }
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }

    /// Set CPU to maximum frequency
    pub fn set_max_frequency(&mut self) -> Result<u32, ()> {
        let max_freq = self.available_opps
            .iter()
            .take(self.opp_count)
            .flatten()
            .map(|opp| opp.freq_hz)
            .max()
            .unwrap_or(1_800_000_000);

        self.set_frequency(max_freq)
    }

    /// Set CPU to minimum frequency (power saving)
    pub fn set_min_frequency(&mut self) -> Result<u32, ()> {
        let min_freq = self.available_opps
            .iter()
            .take(self.opp_count)
            .flatten()
            .map(|opp| opp.freq_hz)
            .min()
            .unwrap_or(600_000_000);

        self.set_frequency(min_freq)
    }
}

/// Global CPU frequency scaler instance
static mut CPU_FREQ: CpuFrequency = CpuFrequency::new();

/// Initialize global DVFS
pub fn init(dtb: Option<&DeviceTree>) {
    unsafe {
        CPU_FREQ.init(dtb);
    }
}

/// Get current CPU frequency
pub fn get_current_frequency() -> u32 {
    unsafe { CPU_FREQ.get_current() }
}

/// Set CPU frequency
pub fn set_frequency(freq_hz: u32) -> Result<u32, ()> {
    unsafe { CPU_FREQ.set_frequency(freq_hz) }
}

/// Set maximum CPU frequency
pub fn set_max_frequency() -> Result<u32, ()> {
    unsafe { CPU_FREQ.set_max_frequency() }
}

/// Set minimum CPU frequency
pub fn set_min_frequency() -> Result<u32, ()> {
    unsafe { CPU_FREQ.set_min_frequency() }
}

/// Get list of available frequencies
pub fn list_available_frequencies() -> &'static [Option<Opp>] {
    unsafe { CPU_FREQ.list_available() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_opps() {
        let mut dvfs = CpuFrequency::new();
        dvfs.set_default_opps();
        
        assert_eq!(dvfs.opp_count, 4);
        assert!(dvfs.available_opps[0].is_some());
        assert_eq!(dvfs.available_opps[0].unwrap().freq_hz, 600_000_000);
    }

    #[test]
    fn test_find_closest_frequency() {
        let mut dvfs = CpuFrequency::new();
        dvfs.set_default_opps();
        
        // Test finding closest frequency
        // Target 700 MHz should select 600 MHz (closest)
        let target = 700_000_000;
        let mut closest_freq = 0;
        let mut min_diff = u32::MAX;

        for opp in dvfs.available_opps.iter().take(dvfs.opp_count).flatten() {
            let diff = if opp.freq_hz > target {
                opp.freq_hz - target
            } else {
                target - opp.freq_hz
            };

            if diff < min_diff {
                min_diff = diff;
                closest_freq = opp.freq_hz;
            }
        }

        assert_eq!(closest_freq, 600_000_000);
    }
}
