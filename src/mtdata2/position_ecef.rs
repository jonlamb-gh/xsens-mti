use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the position of the GNSS/INS in the Earth-Centered, Earth-Fixed (ECEF)
/// coordinate system in meters. Note that position in ECEF cannot be represented in
/// Fixed Point values because of the limited range of fixed point representations. Use
/// double or float representation instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PositionEcef<T: PrecisionExt> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrecisionExt> fmt::Display for PositionEcef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X({:.3}), Y({:.3}), Z({:.3})", self.x, self.y, self.z)
    }
}

precision_float32_3field_wire_impl!(PositionEcef, x, y, z);
precision_float64_3field_wire_impl!(PositionEcef, x, y, z);
