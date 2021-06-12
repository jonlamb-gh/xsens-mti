use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the three Euler angles in degrees that represent the
/// orientation of the MT
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EulerAngles<T: PrecisionExt> {
    pub roll: T,
    pub pitch: T,
    pub yaw: T,
}

impl<T: PrecisionExt> fmt::Display for EulerAngles<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Roll({:.3}), Pitch({:.3}), Yaw({:.3})",
            self.roll, self.pitch, self.yaw,
        )
    }
}

precision_float32_3field_wire_impl!(EulerAngles, roll, pitch, yaw);
precision_float64_3field_wire_impl!(EulerAngles, roll, pitch, yaw);
precision_fp1220_3field_wire_impl!(EulerAngles, roll, pitch, yaw);
precision_fp1632_3field_wire_impl!(EulerAngles, roll, pitch, yaw);
