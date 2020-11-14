// TODO
// this impl could use a refactor
// explicit endianness here or in a wire-level impl
// add all the datatype ids
// modular_bitfield has a nice ascii diagram, do something like that
// use modular-bitfield once const constructors land
// docs on page 31
//
// proptesting

use bitfield::bitfield;
use core::mem;
use static_assertions::const_assert_eq;

const_assert_eq!(mem::size_of::<DataId>(), mem::size_of::<u16>());
const_assert_eq!(mem::align_of::<DataId>(), mem::align_of::<u16>());

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

impl Precision {
    fn to_wire(self) -> u8 {
        u8::from(self) & 0b0000_0011
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
        Ned = 0x1,
        /// North-West-Up coordinate system
        Nwu = 0x2,
    }
}

impl CoordinateSystem {
    fn to_wire(self) -> u8 {
        u8::from(self) & 0b0000_0011
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
    }
}

impl DataType {
    const MASK: u16 = 0b1111_1000_1111_0000;

    fn to_wire(self) -> u16 {
        u16::from(self) & Self::MASK
    }

    fn from_wire(w: u16) -> Self {
        Self::from(w & Self::MASK)
    }

    fn to_wire_typ(self) -> u16 {
        self.to_wire() >> 4
    }

    fn to_wire_group(self) -> u16 {
        self.to_wire() >> 11
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct DataId(u16);
    u16;
    format_precision, set_format_precision : 1, 0;
    format_coordinate_system, set_format_coordinate_system : 3, 2;
    typ, set_typ : 7, 4;
    reserved, set_reserved : 10, 8;
    group, set_group : 15, 11;
}

impl DataId {
    const fn new_unchecked(raw: u16) -> Self {
        DataId(raw)
    }

    pub fn new(
        data_type: DataType,
        precision: Precision,
        coordinate_system: CoordinateSystem,
    ) -> Self {
        let mut id = DataId::new_unchecked(0);
        id.set_format_precision(precision.to_wire() as _);
        id.set_format_coordinate_system(coordinate_system.to_wire() as _);
        id.set_typ(data_type.to_wire_typ());
        id.set_group(data_type.to_wire_group());
        id
    }

    pub fn precision(&self) -> Precision {
        (self.format_precision() as u8).into()
    }

    pub fn set_precision(&mut self, precision: Precision) {
        self.set_format_precision(precision.to_wire() as _);
    }

    pub fn coordinate_system(&self) -> CoordinateSystem {
        (self.format_coordinate_system() as u8).into()
    }

    pub fn set_coordinate_system(&mut self, coordinate_system: CoordinateSystem) {
        self.set_format_coordinate_system(coordinate_system.to_wire() as _);
    }

    pub fn data_type(&self) -> DataType {
        DataType::from_wire(self.0)
    }

    pub fn set_data_type(&mut self, data_type: DataType) {
        self.set_typ(data_type.to_wire_typ());
        self.set_group(data_type.to_wire_group());
    }
}

impl From<u16> for DataId {
    fn from(value: u16) -> Self {
        DataId::new_unchecked(value)
    }
}

impl From<DataId> for u16 {
    fn from(value: DataId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn round_trip() {
        let id = DataId::new(
            DataType::Quaternion,
            Precision::Fp1632,
            CoordinateSystem::Ned,
        );
        assert_eq!(id.data_type(), DataType::Quaternion);
        assert_eq!(id.precision(), Precision::Fp1632);
        assert_eq!(id.coordinate_system(), CoordinateSystem::Ned);
        assert_eq!(u16::from(id), 0x2016);
        let id_out = DataId::from(id.0);
        assert_eq!(id, id_out);
        let mut id_out = DataId::new_unchecked(0b1111_1000_1111_1111);
        id_out.set_data_type(DataType::Quaternion);
        id_out.set_precision(Precision::Fp1632);
        id_out.set_coordinate_system(CoordinateSystem::Ned);
        assert_eq!(id, id_out);
    }
}
