use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the altitude of the GNSS/INS in meters above the WGS-84 Ellipsoid
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AltitudeEllipsoid<T>(pub T);

impl<T: fmt::Display> fmt::Display for AltitudeEllipsoid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

mod float64 {
    use super::*;

    mod field {
        use crate::wire::Field;

        pub const ALT: Field = 0..8;
    }

    impl AltitudeEllipsoid<f64> {
        pub const WIRE_SIZE: usize = 8;

        pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
            if bytes.len() < Self::WIRE_SIZE {
                Err(WireError::MissingBytes)
            } else {
                let alt = BigEndian::read_f64(&bytes[field::ALT]);
                Ok(AltitudeEllipsoid(alt))
            }
        }
    }
}
