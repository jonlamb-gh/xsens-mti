use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the calibrated MagneticField vector in x, y, and z axes in a.u.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MagneticField<T: PrecisionExt> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrecisionExt> fmt::Display for MagneticField<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

precision_float32_3field_wire_impl!(MagneticField, x, y, z);
precision_float64_3field_wire_impl!(MagneticField, x, y, z);
precision_fp1220_3field_wire_impl!(MagneticField, x, y, z);
precision_fp1632_3field_wire_impl!(MagneticField, x, y, z);
