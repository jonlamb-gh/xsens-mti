use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the calibrated acceleration vector in x, y, and z axes in m/s^2
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Acceleration<T: PrecisionExt> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrecisionExt> fmt::Display for Acceleration<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

precision_float32_3field_wire_impl!(Acceleration, x, y, z);
precision_float64_3field_wire_impl!(Acceleration, x, y, z);
precision_fp1220_3field_wire_impl!(Acceleration, x, y, z);
precision_fp1632_3field_wire_impl!(Acceleration, x, y, z);
