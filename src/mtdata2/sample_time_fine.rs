use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the sample time of an output expressed in 10 kHz clock ticks.
/// When there is no GNSS-fix, this value is arbitrary for GNSS messages.
/// This outputs wraps around at:
///   * 0xFFFFFFFFF for the MTi 1-series and MTi 600-series.
///   * Exactly after one day (864000000 ticks) for the MTi 10-series and MTi 100-series.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SampleTimeFine(pub u32);

impl fmt::Display for SampleTimeFine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod field {
    use crate::wire::{field32, Field};

    pub const ST: Field = field32::F0;
}

impl SampleTimeFine {
    pub const WIRE_SIZE: usize = 4;

    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        if bytes.len() < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            let t = BigEndian::read_u32(&bytes[field::ST]);
            Ok(SampleTimeFine(t))
        }
    }
}
