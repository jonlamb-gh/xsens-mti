//! Types and traits used to make the handling of the various precisions more generic
//! in the MTData2 types.
//!
//! There are four precision types available:
//! * Float32: single precision IEEE 32-bit floating point number
//! * Float64: double precision IEEE 64-bit floating point number
//! * Fp1220: fixed point 12.20 32-bit number
//! * Fp1632: fixed point 16.32 48-bit number

use crate::wire::Precision;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

// TODO - consider using num or num_traits crate
// - better integration with wire::Precision,
//   this trait is probably more than what is necessary

pub trait PrecisionExt:
    private::Sealed + PartialEq + Copy + Clone + fmt::Debug + fmt::Display
{
    type NativeType;
    const PRECISION: Precision;

    /// Read a single wire type field from big-endian bytes
    fn read_field(buf: &[u8]) -> Self::NativeType;
}

impl PrecisionExt for f32 {
    type NativeType = f32;
    const PRECISION: Precision = Precision::Float32;

    fn read_field(buf: &[u8]) -> Self::NativeType {
        BigEndian::read_f32(buf)
    }
}

impl PrecisionExt for f64 {
    type NativeType = f64;
    const PRECISION: Precision = Precision::Float64;

    fn read_field(buf: &[u8]) -> Self::NativeType {
        BigEndian::read_f64(buf)
    }
}

impl PrecisionExt for u32 {
    type NativeType = u32;
    const PRECISION: Precision = Precision::Fp1220;

    fn read_field(buf: &[u8]) -> Self::NativeType {
        BigEndian::read_u32(buf)
    }
}

impl PrecisionExt for u64 {
    type NativeType = u64;
    const PRECISION: Precision = Precision::Fp1632;

    fn read_field(buf: &[u8]) -> Self::NativeType {
        BigEndian::read_u64(buf)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}
