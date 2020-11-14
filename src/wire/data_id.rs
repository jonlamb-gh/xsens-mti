// TODO
// add all the datatype ids
// consider doing [repr(transparent) DataId as a bitfield for FFI friendliness
// proptesting, roundtrip, unknowns, etc
// docs on page 31

use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::mem;
use static_assertions::const_assert_eq;

const_assert_eq!(WireDataId::<&[u8]>::WIRE_SIZE, mem::size_of::<u16>());

enum_with_unknown! {
    pub doc enum Precision(u8) {
        /// Single precision IEEE 32-bit floating point number
        Float32 = 0x0,
        /// Fixed point 12.20 32-bit number
        Fp1220 = 0x1,
        /// Fixed point 16.32 48-bit number
        Fp1632 = 0x2,
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
    pub doc enum CoordinateSystem(u8) {
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
        // TemperatureGroup = 0x08x0
        Temperature     = 0x0810,

        // TimestampGroup = 0x10x0
        UtcTime         = 0x1010,
        PacketCounter   = 0x1020,
        SampleTimeFine  = 0x1060,

        //OrientationGroup = 0x20xy
        Quaternion      = 0x2010,

        // AccelerationGroup = 0x40xy
        Acceleration    = 0x4020,
    }
}

impl DataType {
    const MASK: u16 = 0b1111_1000_1111_0000;
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
}

impl From<u16> for DataId {
    fn from(value: u16) -> Self {
        let precision = value & 0b11;
        let coordinate_system = value & 0b1100;
        let group_type = value & DataType::MASK;
        DataId {
            data_type: DataType::from(group_type),
            precision: Precision::from(precision as u8),
            coordinate_system: CoordinateSystem::from(coordinate_system as u8),
        }
    }
}

impl From<DataId> for u16 {
    fn from(value: DataId) -> Self {
        let group_type = DataType::MASK & u16::from(value.data_type);
        let precision = 0b11 & u8::from(value.precision) as u16;
        let coordinate_system = 0b1100 & u8::from(value.coordinate_system) as u16;
        group_type | coordinate_system | precision
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
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
}
