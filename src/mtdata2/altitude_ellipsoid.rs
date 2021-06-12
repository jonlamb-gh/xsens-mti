use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the altitude of the GNSS/INS in meters above the WGS-84 Ellipsoid
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AltitudeEllipsoid<T: PrecisionExt>(pub T);

impl<T: PrecisionExt> fmt::Display for AltitudeEllipsoid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

precision_float32_newtype_wire_impl!(AltitudeEllipsoid);
precision_float64_newtype_wire_impl!(AltitudeEllipsoid);
precision_fp1220_newtype_wire_impl!(AltitudeEllipsoid);
precision_fp1632_newtype_wire_impl!(AltitudeEllipsoid);
