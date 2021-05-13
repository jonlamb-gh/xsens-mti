// TODO - consider using the Float trait from num crate for f64/f32/fixed-point Precision's
// also see https://docs.rs/nalgebra/0.26.2/nalgebra/trait.RealField.html
// and https://docs.rs/num-traits/0.2.14/num_traits/float/trait.Float.html
// maybe just roll my own to cover all the Precision variants

use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// Contains the three Euler angles in degrees that represent the
/// orientation of the MT
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EulerAngles<T> {
    pub roll: T,
    pub pitch: T,
    pub yaw: T,
}

impl<T: fmt::Display> fmt::Display for EulerAngles<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Roll({:.3}), Pitch({:.3}), Yaw({:.3})",
            self.roll, self.pitch, self.yaw,
        )
    }
}

mod float32 {
    use super::*;

    mod field {
        use crate::wire::Field;

        pub const ROLL: Field = 0..4;
        pub const PITCH: Field = 4..8;
        pub const YAW: Field = 8..12;
    }

    impl EulerAngles<f32> {
        pub const WIRE_SIZE: usize = 12;

        pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
            if bytes.len() < Self::WIRE_SIZE {
                Err(WireError::MissingBytes)
            } else {
                let roll = BigEndian::read_f32(&bytes[field::ROLL]);
                let pitch = BigEndian::read_f32(&bytes[field::PITCH]);
                let yaw = BigEndian::read_f32(&bytes[field::YAW]);
                Ok(EulerAngles { roll, pitch, yaw })
            }
        }
    }
}
