/// Supported DHT sensor types
pub enum SensorType {
    DHT11,
    DHT22,
}

impl SensorType {
    /// Returns the minimum delay between readings in microseconds
    pub fn min_delay_us(&self) -> u32 {
        match self {
            SensorType::DHT11 => 2_000_000, // 2 seconds
            SensorType::DHT22 => 2_000_000, // 2 seconds
        }
    }

    /// Returns the duration to hold the start signal low in microseconds
    pub fn signal_pulse_us(&self) -> u32 {
        match self {
            SensorType::DHT11 => 20_000, // 20 ms
            SensorType::DHT22 => 1_100,  // 1.1 ms
        }
    }
}

/// Maximum number of cycles to wait for a pulse
pub const MAX_CYCLES: u32 = 1_000_000;

/// Timeout value indicating a pulse was not detected
pub const TIMEOUT: u32 = core::u32::MAX;
