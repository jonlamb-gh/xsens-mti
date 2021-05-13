use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the position of the GNSS/INS in the Earth-Centered, Earth-Fixed (ECEF)
/// coordinate system in meters. Note that position in ECEF cannot be represented in
/// Fixed Point values because of the limited range of fixed point representations. Use
/// double or float representation instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PositionEcef<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: fmt::Display> fmt::Display for PositionEcef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

mod float64 {
    use super::*;

    mod field {
        use crate::wire::Field;

        pub const X: Field = 0..8;
        pub const Y: Field = 8..16;
        pub const Z: Field = 16..24;
    }

    impl PositionEcef<f64> {
        pub const WIRE_SIZE: usize = 24;

        pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
            if bytes.len() < Self::WIRE_SIZE {
                Err(WireError::MissingBytes)
            } else {
                let x = BigEndian::read_f64(&bytes[field::X]);
                let y = BigEndian::read_f64(&bytes[field::Y]);
                let z = BigEndian::read_f64(&bytes[field::Z]);
                Ok(PositionEcef { x, y, z })
            }
        }
    }
}
