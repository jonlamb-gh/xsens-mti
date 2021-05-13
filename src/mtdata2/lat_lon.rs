use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the latitude and longitude in degrees of the GNSS/INS position
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LatLon<T> {
    pub lat: T,
    pub lon: T,
}

impl<T: fmt::Display> fmt::Display for LatLon<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lat({:.3}), Lon({:.3})", self.lat, self.lon)
    }
}

mod float64 {
    use super::*;

    mod field {
        use crate::wire::Field;

        pub const LAT: Field = 0..8;
        pub const LON: Field = 8..16;
    }

    impl LatLon<f64> {
        pub const WIRE_SIZE: usize = 16;

        pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
            if bytes.len() < Self::WIRE_SIZE {
                Err(WireError::MissingBytes)
            } else {
                let lat = BigEndian::read_f64(&bytes[field::LAT]);
                let lon = BigEndian::read_f64(&bytes[field::LON]);
                Ok(LatLon { lat, lon })
            }
        }
    }
}
