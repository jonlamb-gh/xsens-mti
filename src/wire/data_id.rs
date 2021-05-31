//! MTData2 Data Identifier
//!
//! The data identifier is a 16-bit unsigned integer describing the contents
//! of an MTData2 packet.
//!
//! It consists of the following fields:
//! * Format: 4-bits describing the data precision and coordinate system
//! * Type: lower 4-bits of the DataType
//! * Group: upper 5-bits of the DataType
//!
//! ```no_compile
//! B15                            B0
//!  ┊                             ┊
//!  ┊                             ┊
//!  ┊                             ┊
//! ║  second byte  ║  first byte   ║
//! ╟───────────────╫───────────────╢
//! ║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
//! ╟────────-╫────-╫────────╫──────╢
//! ║  group  ║     ║  type  ║      ║
//!              ┊               ┊
//!              ┊               ┊
//!           reserved         format
//! ```

// TODO
// add all the datatype ids
// docs on page 31
// do a nicer Display impl

use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use core::mem;
use static_assertions::const_assert_eq;

const_assert_eq!(WireDataId::<&[u8]>::WIRE_SIZE, mem::size_of::<u16>());

enum_with_unknown! {
    pub enum Precision(u8) {
        /// Single precision IEEE 32-bit floating point number
        Float32 = 0x0,
        /// Fixed point 12.20 32-bit number
        Fp1220  = 0x1,
        /// Fixed point 16.32 48-bit number
        Fp1632  = 0x2,
        /// Double precision IEEE 64-bit floating point number
        Float64 = 0x3,
    }
}

impl Default for Precision {
    fn default() -> Self {
        Precision::Float32
    }
}

enum_with_unknown! {
    pub enum CoordinateSystem(u8) {
        /// East-North-Up coordinate system
        Enu = 0x0,
        /// North-East-Down coordinate system
        Ned = 0x4,
        /// North-West-Up coordinate system
        Nwu = 0x8,
    }
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        CoordinateSystem::Enu
    }
}

enum_with_unknown! {
    pub enum DataType(u16) {
        // TemperatureGroup     = 0x08x0
        Temperature             = 0x0810,

        // TimestampGroup       = 0x10x0
        UtcTime                 = 0x1010,
        PacketCounter           = 0x1020,
        SampleTimeFine          = 0x1060,
        SampleTimeCoarse        = 0x1070,

        // OrientationGroup     = 0x20xy
        Quaternion              = 0x2010,
        EulerAngles             = 0x2030,

        // AccelerationGroup    = 0x40xy
        Acceleration            = 0x4020,

        // PositionGroup        = 0x50xy
        AltitudeEllipsoid       = 0x5020,
        PositionEcef            = 0x5030,
        LatLon                  = 0x5040,

        // AngularVelocityGroup = 0x80xy
        RateOfTurn              = 0x8020,

        // VelocityGroup        = 0xD0xy
        VelocityXYZ             = 0xD010,

        // StatusGroup          = 0xE0x0
        StatusByte              = 0xE010,
        StatusWord              = 0xE020,
    }
}

impl DataType {
    /// Shave off the format bits (B0:B3) and the reserved bits (B8:B10)
    /// to yield the type and group bits that form the DataType
    const MASK: u16 = 0b1111_1000_1111_0000;
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Unknown(t) => write!(f, "Unknown(0x{:04X})", t),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DataId {
    pub data_type: DataType,
    pub precision: Precision,
    pub coordinate_system: CoordinateSystem,
}

impl DataId {
    pub fn new(
        data_type: DataType,
        precision: Precision,
        coordinate_system: CoordinateSystem,
    ) -> Self {
        DataId {
            data_type,
            precision,
            coordinate_system,
        }
    }

    pub fn to_wire(self) -> u16 {
        let group_type = DataType::MASK & u16::from(self.data_type);
        let precision = 0b11 & u8::from(self.precision) as u16;
        let coordinate_system = 0b1100 & u8::from(self.coordinate_system) as u16;
        group_type | coordinate_system | precision
    }

    pub fn from_wire(value: u16) -> Self {
        let precision = value & 0b11;
        let coordinate_system = value & 0b1100;
        let group_type = value & DataType::MASK;
        DataId {
            data_type: DataType::from(group_type),
            precision: Precision::from(precision as u8),
            coordinate_system: CoordinateSystem::from(coordinate_system as u8),
        }
    }

    pub fn from_data_type(data_type: DataType) -> Self {
        DataId {
            data_type,
            precision: Precision::default(),
            coordinate_system: CoordinateSystem::default(),
        }
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }

    pub fn coordinate_system(&self) -> CoordinateSystem {
        self.coordinate_system
    }
}

impl From<u16> for DataId {
    fn from(value: u16) -> Self {
        DataId::from_wire(value)
    }
}

impl From<DataId> for u16 {
    fn from(value: DataId) -> Self {
        value.to_wire()
    }
}

impl fmt::Display for DataId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DataId(0x{:04X}, {}, {:?}, {:?})",
            self.to_wire(),
            self.data_type(),
            self.precision(),
            self.coordinate_system(),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WireDataId<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::wire::Field;

    pub const DATA_ID: Field = 0..2;
}

impl<T: AsRef<[u8]>> WireDataId<T> {
    pub const WIRE_SIZE: usize = mem::size_of::<u16>();

    pub fn new_unchecked(buffer: T) -> WireDataId<T> {
        WireDataId { buffer }
    }

    pub fn new(buffer: T) -> Result<WireDataId<T>, WireError> {
        let f = Self::new_unchecked(buffer);
        f.check_len()?;
        Ok(f)
    }

    pub fn check_len(&self) -> Result<(), WireError> {
        let len = self.buffer.as_ref().len();
        if len < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            Ok(())
        }
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }

    #[inline]
    pub fn buffer_len() -> usize {
        Self::WIRE_SIZE
    }

    #[inline]
    pub fn data_id(&self) -> DataId {
        let data = self.buffer.as_ref();
        let value = BigEndian::read_u16(&data[field::DATA_ID]);
        DataId::from(value)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> WireDataId<T> {
    #[inline]
    pub fn set_data_id(&mut self, value: DataId) {
        let data = self.buffer.as_mut();
        BigEndian::write_u16(&mut data[field::DATA_ID], u16::from(value));
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for WireDataId<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

#[cfg(test)]
pub(crate) mod propt {
    use super::*;
    use proptest::{
        arbitrary::Arbitrary,
        num,
        prelude::{any, RngCore},
        prop_compose,
        strategy::{NewTree, Strategy, ValueTree},
        test_runner::TestRunner,
    };

    impl Precision {
        const MASK: u8 = Precision::Float32.into_inner()
            | Precision::Fp1220.into_inner()
            | Precision::Fp1632.into_inner()
            | Precision::Float64.into_inner();
    }

    pub struct PrecisionBinarySearch(num::u8::BinarySearch);

    impl ValueTree for PrecisionBinarySearch {
        type Value = Precision;

        fn current(&self) -> Precision {
            let v = self.0.current();
            Precision::from(v & Precision::MASK)
        }

        fn simplify(&mut self) -> bool {
            self.0.simplify()
        }

        fn complicate(&mut self) -> bool {
            self.0.complicate()
        }
    }

    #[derive(Debug)]
    pub struct AnyPrecision;

    impl Strategy for AnyPrecision {
        type Tree = PrecisionBinarySearch;
        type Value = Precision;

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            Ok(PrecisionBinarySearch(num::u8::BinarySearch::new(
                runner.rng().next_u32() as u8,
            )))
        }
    }

    impl Arbitrary for Precision {
        type Parameters = ();
        type Strategy = AnyPrecision;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            AnyPrecision
        }
    }

    pub fn gen_precision() -> impl Strategy<Value = Precision> {
        any::<Precision>()
    }

    impl CoordinateSystem {
        const MASK: u8 = CoordinateSystem::Enu.into_inner()
            | CoordinateSystem::Ned.into_inner()
            | CoordinateSystem::Nwu.into_inner();
    }

    pub struct CoordinateSystemBinarySearch(num::u8::BinarySearch);

    impl ValueTree for CoordinateSystemBinarySearch {
        type Value = CoordinateSystem;

        fn current(&self) -> CoordinateSystem {
            let v = self.0.current();
            CoordinateSystem::from(v & CoordinateSystem::MASK)
        }

        fn simplify(&mut self) -> bool {
            self.0.simplify()
        }

        fn complicate(&mut self) -> bool {
            self.0.complicate()
        }
    }

    #[derive(Debug)]
    pub struct AnyCoordinateSystem;

    impl Strategy for AnyCoordinateSystem {
        type Tree = CoordinateSystemBinarySearch;
        type Value = CoordinateSystem;

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            Ok(CoordinateSystemBinarySearch(num::u8::BinarySearch::new(
                runner.rng().next_u32() as u8,
            )))
        }
    }

    impl Arbitrary for CoordinateSystem {
        type Parameters = ();
        type Strategy = AnyCoordinateSystem;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            AnyCoordinateSystem
        }
    }

    pub fn gen_coordinate_system() -> impl Strategy<Value = CoordinateSystem> {
        any::<CoordinateSystem>()
    }

    pub struct DataTypeBinarySearch(num::u16::BinarySearch);

    impl ValueTree for DataTypeBinarySearch {
        type Value = DataType;

        fn current(&self) -> DataType {
            let v = self.0.current();
            DataType::from(v & DataType::MASK)
        }

        fn simplify(&mut self) -> bool {
            self.0.simplify()
        }

        fn complicate(&mut self) -> bool {
            self.0.complicate()
        }
    }

    #[derive(Debug)]
    pub struct AnyDataType;

    impl Strategy for AnyDataType {
        type Tree = DataTypeBinarySearch;
        type Value = DataType;

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            Ok(DataTypeBinarySearch(num::u16::BinarySearch::new(
                runner.rng().next_u32() as u16,
            )))
        }
    }

    impl Arbitrary for DataType {
        type Parameters = ();
        type Strategy = AnyDataType;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            AnyDataType
        }
    }

    pub fn gen_data_type() -> impl Strategy<Value = DataType> {
        any::<DataType>()
    }

    prop_compose! {
        pub fn gen_data_id()(
            data_type in gen_data_type(),
            precision in gen_precision(),
            coordinate_system in gen_coordinate_system(),
        ) -> DataId {
            DataId::new(data_type, precision, coordinate_system)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use propt::*;
    use proptest::prelude::*;

    static WIRE_BYTES: [u8; 2] = [0x20, 0x16];

    #[test]
    fn buffer_len() {
        assert_eq!(WireDataId::<&[u8]>::buffer_len(), mem::size_of::<u16>());
        assert_eq!(WireDataId::<&[u8]>::WIRE_SIZE, mem::size_of::<u16>());
    }

    #[test]
    fn construct() {
        let mut bytes = [0xFF; 2];
        let mut w = WireDataId::new_unchecked(&mut bytes[..]);
        assert_eq!(w.check_len(), Ok(()));
        w.set_data_id(DataId::new(
            DataType::Quaternion,
            Precision::Fp1632,
            CoordinateSystem::Ned,
        ));
        assert_eq!(&w.into_inner()[..], &WIRE_BYTES[..]);
    }

    #[test]
    fn deconstruct() {
        let w = WireDataId::new(&WIRE_BYTES[..]).unwrap();
        assert_eq!(
            w.data_id(),
            DataId::new(
                DataType::Quaternion,
                Precision::Fp1632,
                CoordinateSystem::Ned,
            )
        );
    }

    #[test]
    fn missing_bytes() {
        let bytes = [0xFF; 2 - 1];
        assert_eq!(bytes.len(), WireDataId::<&[u8]>::buffer_len() - 1);
        let w = WireDataId::new(&bytes[..]);
        assert_eq!(w.unwrap_err(), WireError::MissingBytes);
    }

    proptest! {
        #[test]
        fn round_trip_precision(v_in in gen_precision()) {
            let wire = u8::from(v_in);
            let v_out = Precision::from(wire);
            assert_eq!(v_in, v_out);
        }

        #[test]
        fn round_trip_coordinate_system(v_in in gen_coordinate_system()) {
            let wire = u8::from(v_in);
            let v_out = CoordinateSystem::from(wire);
            assert_eq!(v_in, v_out);
        }

        #[test]
        fn round_trip_data_type(v_in in gen_data_type()) {
            let wire = u16::from(v_in);
            let v_out = DataType::from(wire);
            assert_eq!(v_in, v_out);
        }

        #[test]
        fn round_trip_data_id(v_in in gen_data_id()) {
            let wire = u16::from(v_in).to_be_bytes();
            let v_out = DataId::from(u16::from_be_bytes(wire));
            assert_eq!(v_in, v_out);
        }
    }
}
