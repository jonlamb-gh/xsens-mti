use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the calibrated rate of turn vector in x, y, and z axes in rad/s
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RateOfTurn<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: fmt::Display> fmt::Display for RateOfTurn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

mod float32 {
    use super::*;

    mod field {
        use crate::wire::Field;

        pub const X: Field = 0..4;
        pub const Y: Field = 4..8;
        pub const Z: Field = 8..12;
    }

    impl RateOfTurn<f32> {
        pub const WIRE_SIZE: usize = 12;

        pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
            if bytes.len() < Self::WIRE_SIZE {
                Err(WireError::MissingBytes)
            } else {
                let x = BigEndian::read_f32(&bytes[field::X]);
                let y = BigEndian::read_f32(&bytes[field::Y]);
                let z = BigEndian::read_f32(&bytes[field::Z]);
                Ok(RateOfTurn { x, y, z })
            }
        }
    }
}
