// Copied from https://github.com/smoltcp-rs/smoltcp/blob/master/src/macros.rs
macro_rules! enum_with_unknown {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident($ty:ty) {
            $(
              $( #[$variant_attr:meta] )*
              $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
        $( #[$enum_attr] )*
        pub enum $name {
            $(
              $( #[$variant_attr] )*
              $variant
            ),*,
            Unknown($ty)
        }

        impl $name {
            pub(crate) const fn into_inner(self) -> $ty {
                match self {
                    $( $name::$variant => $value ),*,
                    $name::Unknown(other) => other
                }
            }
        }

        impl ::core::convert::From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( $value => $name::$variant ),*,
                    other => $name::Unknown(other)
                }
            }
        }

        impl ::core::convert::From<$name> for $ty {
            fn from(value: $name) -> Self {
                value.into_inner()
            }
        }
    }
}

// TODO
// - consider making a derive proc macro crate instead of these macros
// - add encoding side
// - add generated tests for round trip

macro_rules! precision_float32_newtype_wire_impl {
    ($name:ident) => {
        mod generated_1field_float32 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    1 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let inner = Inner::read_field(&bytes[field32::F0]);
                        Ok($crate::prelude::$name(inner))
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_1field_float32 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f
            const BYTES: [u8; 4] = [0x3F, 0x8C, 0xCC, 0xCD];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f32>::WIRE_SIZE);
                let t = $name::<f32>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.0, 1.1);
            }
        }
    };
}

macro_rules! precision_fp1220_newtype_wire_impl {
    ($name:ident) => {
        mod generated_1field_fp1220 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    1 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let inner = Inner::read_field(&bytes[field32::F0]);
                        Ok($crate::prelude::$name(inner))
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_1field_fp1220 {
            use crate::prelude::$name;
            // Big-endian 1,
            const BYTES: [u8; 4] = [0x00, 0x00, 0x00, 0x01];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u32>::WIRE_SIZE);
                let t = $name::<u32>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.0, 1);
            }
        }
    };
}

macro_rules! precision_float64_newtype_wire_impl {
    ($name:ident) => {
        mod generated_1field_float64 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    1 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let inner = Inner::read_field(&bytes[field64::F0]);
                        Ok($crate::prelude::$name(inner))
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_1field_float64 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f
            const BYTES: [u8; 8] = [0x3F, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f64>::WIRE_SIZE);
                let t = $name::<f64>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.0, 1.1);
            }
        }
    };
}

macro_rules! precision_fp1632_newtype_wire_impl {
    ($name:ident) => {
        mod generated_1field_fp1632 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    1 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let inner = Inner::read_field(&bytes[field64::F0]);
                        Ok($crate::prelude::$name(inner))
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_fp1632 {
            use crate::prelude::$name;
            // Big-endian 1,
            const BYTES: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u64>::WIRE_SIZE);
                let t = $name::<u64>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.0, 1);
            }
        }
    };
}

macro_rules! precision_float32_2field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident) => {
        mod generated_2field_float32 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    2 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field32::F0]);
                        let $f1 = Inner::read_field(&bytes[field32::F1]);
                        Ok($crate::prelude::$name { $f0, $f1 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_2field_float32 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f, 2.2f
            const BYTES: [u8; 8] = [0x3F, 0x8C, 0xCC, 0xCD, 0x40, 0x0C, 0xCC, 0xCD];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f32>::WIRE_SIZE);
                let t = $name::<f32>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.$f0, 1.1);
                assert_relative_eq!(t.$f1, 2.2);
            }
        }
    };
}

macro_rules! precision_fp1220_2field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident) => {
        mod generated_2field_fp1220 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    2 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field32::F0]);
                        let $f1 = Inner::read_field(&bytes[field32::F1]);
                        Ok($crate::prelude::$name { $f0, $f1 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_2field_fp1220 {
            use crate::prelude::$name;
            // Big-endian 1, 2,
            const BYTES: [u8; 8] = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u32>::WIRE_SIZE);
                let t = $name::<u32>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.$f0, 1);
                assert_eq!(t.$f1, 2);
            }
        }
    };
}

macro_rules! precision_float64_2field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident) => {
        mod generated_2field_float64 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    2 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field64::F0]);
                        let $f1 = Inner::read_field(&bytes[field64::F1]);
                        Ok($crate::prelude::$name { $f0, $f1 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_2field_float64 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f, 2.2f
            const BYTES: [u8; 16] = [
                0x3F, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A, 0x40, 0x01, 0x99, 0x99, 0x99, 0x99,
                0x99, 0x9A,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f64>::WIRE_SIZE);
                let t = $name::<f64>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.$f0, 1.1);
                assert_relative_eq!(t.$f1, 2.2);
            }
        }
    };
}

macro_rules! precision_fp1632_2field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident) => {
        mod generated_2field_fp1632 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    2 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field64::F0]);
                        let $f1 = Inner::read_field(&bytes[field64::F1]);
                        Ok($crate::prelude::$name { $f0, $f1 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_fp1632 {
            use crate::prelude::$name;
            // Big-endian 1, 2
            const BYTES: [u8; 16] = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x02,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u64>::WIRE_SIZE);
                let t = $name::<u64>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.$f0, 1);
                assert_eq!(t.$f1, 2);
            }
        }
    };
}

macro_rules! precision_float32_3field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident, $f2:ident) => {
        mod generated_3field_float32 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    3 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field32::F0]);
                        let $f1 = Inner::read_field(&bytes[field32::F1]);
                        let $f2 = Inner::read_field(&bytes[field32::F2]);
                        Ok($crate::prelude::$name { $f0, $f1, $f2 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_float32 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f, 2.2f, 3.3f
            const BYTES: [u8; 12] = [
                0x3F, 0x8C, 0xCC, 0xCD, 0x40, 0x0C, 0xCC, 0xCD, 0x40, 0x53, 0x33, 0x33,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f32>::WIRE_SIZE);
                let t = $name::<f32>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.$f0, 1.1);
                assert_relative_eq!(t.$f1, 2.2);
                assert_relative_eq!(t.$f2, 3.3);
            }
        }
    };
}

macro_rules! precision_fp1220_3field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident, $f2:ident) => {
        mod generated_3field_fp1220 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field32, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u32;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    3 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field32::F0]);
                        let $f1 = Inner::read_field(&bytes[field32::F1]);
                        let $f2 = Inner::read_field(&bytes[field32::F2]);
                        Ok($crate::prelude::$name { $f0, $f1, $f2 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_fp1220 {
            use crate::prelude::$name;
            // Big-endian 1, 2, 3
            const BYTES: [u8; 12] = [
                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u32>::WIRE_SIZE);
                let t = $name::<u32>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.$f0, 1);
                assert_eq!(t.$f1, 2);
                assert_eq!(t.$f2, 3);
            }
        }
    };
}

macro_rules! precision_float64_3field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident, $f2:ident) => {
        mod generated_3field_float64 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = f64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    3 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field64::F0]);
                        let $f1 = Inner::read_field(&bytes[field64::F1]);
                        let $f2 = Inner::read_field(&bytes[field64::F2]);
                        Ok($crate::prelude::$name { $f0, $f1, $f2 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_float64 {
            use crate::prelude::$name;
            use approx::assert_relative_eq;
            // Big-endian 1.1f, 2.2f, 3.3f
            const BYTES: [u8; 24] = [
                0x3F, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A, 0x40, 0x01, 0x99, 0x99, 0x99, 0x99,
                0x99, 0x9A, 0x40, 0x0A, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<f64>::WIRE_SIZE);
                let t = $name::<f64>::from_be_slice(&BYTES).unwrap();
                assert_relative_eq!(t.$f0, 1.1);
                assert_relative_eq!(t.$f1, 2.2);
                assert_relative_eq!(t.$f2, 3.3);
            }
        }
    };
}

macro_rules! precision_fp1632_3field_wire_impl {
    ($name:ident, $f0:ident, $f1:ident, $f2:ident) => {
        mod generated_3field_fp1632 {
            use crate::precision::PrecisionExt;
            use crate::wire::{field64, WireError};
            use core::mem;
            use static_assertions::assert_eq_size;

            type Inner = u64;
            assert_eq_size!(Inner, <Inner as PrecisionExt>::NativeType);

            impl $crate::prelude::$name<Inner> {
                pub const WIRE_SIZE: usize =
                    3 * mem::size_of::<<Inner as PrecisionExt>::NativeType>();

                pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
                    if bytes.len() < Self::WIRE_SIZE {
                        Err(WireError::MissingBytes)
                    } else {
                        let $f0 = Inner::read_field(&bytes[field64::F0]);
                        let $f1 = Inner::read_field(&bytes[field64::F1]);
                        let $f2 = Inner::read_field(&bytes[field64::F2]);
                        Ok($crate::prelude::$name { $f0, $f1, $f2 })
                    }
                }
            }
        }
        #[cfg(test)]
        mod generated_tests_3field_fp1632 {
            use crate::prelude::$name;
            // Big-endian 1, 2, 3
            const BYTES: [u8; 24] = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
            ];
            #[test]
            fn decode_from_be_slice() {
                assert_eq!(BYTES.len(), $name::<u64>::WIRE_SIZE);
                let t = $name::<u64>::from_be_slice(&BYTES).unwrap();
                assert_eq!(t.$f0, 1);
                assert_eq!(t.$f1, 2);
                assert_eq!(t.$f2, 3);
            }
        }
    };
}
