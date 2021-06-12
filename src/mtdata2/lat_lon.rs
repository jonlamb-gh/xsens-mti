use crate::precision::PrecisionExt;
use core::fmt;

/// Contains the latitude and longitude in degrees of the GNSS/INS position
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LatLon<T: PrecisionExt> {
    pub lat: T,
    pub lon: T,
}

impl<T: PrecisionExt> fmt::Display for LatLon<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lat({:.3}), Lon({:.3})", self.lat, self.lon)
    }
}

precision_float32_2field_wire_impl!(LatLon, lat, lon);
precision_float64_2field_wire_impl!(LatLon, lat, lon);
precision_fp1220_2field_wire_impl!(LatLon, lat, lon);
precision_fp1632_2field_wire_impl!(LatLon, lat, lon);
