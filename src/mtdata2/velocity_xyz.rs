use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the X, Y and Z components of the GNSS/INS velocity in m/s
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VelocityXYZ<T: PrecisionExt> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrecisionExt> fmt::Display for VelocityXYZ<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

precision_float32_3field_wire_impl!(VelocityXYZ, x, y, z);
precision_float64_3field_wire_impl!(VelocityXYZ, x, y, z);
precision_fp1220_3field_wire_impl!(VelocityXYZ, x, y, z);
precision_fp1632_3field_wire_impl!(VelocityXYZ, x, y, z);
