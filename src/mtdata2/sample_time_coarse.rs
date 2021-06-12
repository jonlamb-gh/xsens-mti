use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the sample time of an output expressed in seconds.
/// When there is noGNSS-fix, this value is arbitrary for GNSS messages.
///
/// Combining XDI_SampleTimeCoarse and XDI_SampleTimeFine allows for creating
/// a big range timestamp (expressed as a real number) using:
/// BigTimestamp = [SampleTimeCoarse + (SampleTimeFine mod 10000) / 10000]
/// (seconds)For MTi 1-series devices, this computation is valid only until the
/// wraparound of XDI_SampleTimeFine occurs.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SampleTimeCoarse(pub u32);

impl fmt::Display for SampleTimeCoarse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod field {
    use crate::wire::{field32, Field};

    pub const ST: Field = field32::F0;
}

impl SampleTimeCoarse {
    pub const WIRE_SIZE: usize = 4;

    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        if bytes.len() < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            let t = BigEndian::read_u32(&bytes[field::ST]);
            Ok(SampleTimeCoarse(t))
        }
    }
}
