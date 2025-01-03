use core::fmt;

/// Errors that can occur while interacting with the DHT sensor
#[derive(Debug)]
pub enum DhtError {
    /// Timeout while waiting for a pulse
    Timeout,
    /// Checksum validation failed
    ChecksumMismatch,
    /// Invalid sensor type
    InvalidSensorType,
    /// I/O Error (e.g., failed to set pin state)
    IoError,
}

impl fmt::Display for DhtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhtError::Timeout => write!(f, "Timeout while waiting for pulse"),
            DhtError::ChecksumMismatch => write!(f, "Checksum validation failed"),
            DhtError::InvalidSensorType => write!(f, "Invalid sensor type"),
            DhtError::IoError => write!(f, "I/O Error occurred"),
        }
    }
}

impl core::error::Error for DhtError {}
