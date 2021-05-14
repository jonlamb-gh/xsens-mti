use crate::wire::{DataId, WireDataId, WireError};
use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use core::mem;
use static_assertions::const_assert_eq;

// TODO
//
// AsMut setters for tests
// construction related tests
// tests for all the different types/sizes, data_as_T()
// make the data_as_u8/etc generic over T: primitive int | f32 | f64, see the num crate
// consider a helper method on the wire iterator to count the packets in buffer
//
// probably need some high-level structs/types for each of the MTData2 packet types
// simple ones can just be newtypes
//   like XDI_PacketCounter, it knows it needs to call data_as_u16
// some are more complex than just primitives, like XDI_UtcTime
// others depend on the DataId's Precision like f32/f64/fixed-int

const_assert_eq!(
    MTData2Packet::<&[u8]>::MIN_WIRE_SIZE,
    mem::size_of::<u16>() + mem::size_of::<u8>()
);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MTData2Packet<T: AsRef<[u8]>> {
    buffer: T,
}

impl<T: AsRef<[u8]>> fmt::Display for MTData2Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, Len({})", self.data_id(), self.data_length())
    }
}

mod field {
    use crate::wire::{Field, Rest};

    pub const DATA_ID: Field = 0..2;
    pub const LEN: usize = 2;
    pub const PAYLOAD: Rest = 3..;
}

impl<T: AsRef<[u8]>> MTData2Packet<T> {
    /// Min size, DataId, DataLen, 0 len payload
    pub const MIN_WIRE_SIZE: usize = WireDataId::<&[u8]>::WIRE_SIZE + mem::size_of::<u8>();

    pub fn new_unchecked(buffer: T) -> MTData2Packet<T> {
        MTData2Packet { buffer }
    }

    pub fn new(buffer: T) -> Result<MTData2Packet<T>, WireError> {
        let f = Self::new_unchecked(buffer);
        f.check_len()?;
        f.check_payload_length()?;
        Ok(f)
    }

    pub fn check_len(&self) -> Result<(), WireError> {
        let len = self.buffer.as_ref().len();
        if len < Self::MIN_WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            Ok(())
        }
    }

    pub fn check_payload_length(&self) -> Result<(), WireError> {
        let payload_len = self.data_length();
        let len = self.buffer.as_ref().len();
        if (len < Self::buffer_len(0)) || (len < Self::buffer_len(payload_len.into())) {
            Err(WireError::MissingBytes)
        } else {
            Ok(())
        }
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }

    #[inline]
    pub fn header_len() -> usize {
        field::PAYLOAD.start
    }

    #[inline]
    pub fn buffer_len(n_payload_bytes: usize) -> usize {
        Self::header_len() + n_payload_bytes
    }

    #[inline]
    pub fn data_id(&self) -> DataId {
        let data = self.buffer.as_ref();
        let value = BigEndian::read_u16(&data[field::DATA_ID]);
        DataId::from(value)
    }

    #[inline]
    pub fn data_length(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::LEN]
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> MTData2Packet<&'a T> {
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        let payload_len = self.data_length();
        let end = Self::header_len() + usize::from(payload_len);
        let data = self.buffer.as_ref();
        &data[field::PAYLOAD.start..end]
    }

    #[inline]
    pub fn data_as_u8(&self) -> Result<u8, WireError> {
        let data_len = usize::from(self.data_length());
        if data_len != mem::size_of::<u8>() {
            Err(WireError::MissingBytes)
        } else {
            let payload = self.payload();
            Ok(payload[0])
        }
    }

    #[inline]
    pub fn data_as_u16(&self) -> Result<u16, WireError> {
        let data_len = usize::from(self.data_length());
        if data_len != mem::size_of::<u16>() {
            Err(WireError::MissingBytes)
        } else {
            let payload = self.payload();
            Ok(BigEndian::read_u16(payload))
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for MTData2Packet<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MTData2PacketIter<'a> {
    cursor: usize,
    buffer: &'a [u8],
}

impl<'a> MTData2PacketIter<'a> {
    pub(crate) fn new(buffer: &'a [u8]) -> Self {
        MTData2PacketIter { cursor: 0, buffer }
    }
}

impl<'a> Iterator for MTData2PacketIter<'a> {
    type Item = Result<MTData2Packet<&'a [u8]>, WireError>;
    fn next(&mut self) -> Option<Self::Item> {
        let payload_bytes_exhausted = self.cursor >= self.buffer.len();
        if payload_bytes_exhausted {
            None
        } else {
            match MTData2Packet::new(&self.buffer[self.cursor..]) {
                Ok(p) => {
                    self.cursor += MTData2Packet::<&[u8]>::buffer_len(usize::from(p.data_length()));
                    Some(Ok(p))
                }
                Err(e) => {
                    self.cursor = self.buffer.len();
                    Some(Err(e))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::{CoordinateSystem, DataType, Precision};
    use pretty_assertions::assert_eq;

    // DataId: 0x1020 (PacketCounter)
    // DataLen: 2
    // Value: 0x0114
    static WIRE_BYTES: [u8; 5] = [0x10, 0x20, 0x02, 0x01, 0x14];

    #[rustfmt::skip]
    static SEQ_WIRE_BYTES: [u8; 57] = [
        // UtcTime
        0x10, 0x10, 0x0C, 0x24, 0x34, 0x30, 0x40, 0x07, 0xB2, 0x01, 0x01, 0x00, 0x00, 0x11, 0x00,
        // PacketCounter
        0x10, 0x20, 0x02, 0x01, 0x14,
        // SampleTimeFine
        0x10, 0x60, 0x04, 0x00, 0x02, 0xAF, 0xCA,
        // Quaternion
        0x20, 0x34, 0x0C, 0x43, 0x32, 0x09, 0x1E, 0xC0, 0x5A, 0xBC, 0xA1, 0x42, 0xAC, 0x7F, 0x61,
        // Acceleration
        0x40, 0x20, 0x0C, 0xBD, 0x9E, 0x50, 0xD6, 0x3E, 0x0A, 0x45, 0x4B, 0x41, 0x1D, 0x60, 0x76,
    ];

    #[test]
    fn buffer_len() {
        assert_eq!(
            MTData2Packet::<&[u8]>::buffer_len(0),
            MTData2Packet::<&[u8]>::MIN_WIRE_SIZE,
        );
        assert_eq!(
            MTData2Packet::<&[u8]>::MIN_WIRE_SIZE,
            MTData2Packet::<&[u8]>::header_len()
        );
    }

    #[test]
    fn deconstruct() {
        let w = MTData2Packet::new(&WIRE_BYTES[..]).unwrap();
        assert_eq!(w.data_id(), DataId::from_data_type(DataType::PacketCounter));
        assert_eq!(w.data_length(), 2);
        let data = w.payload();
        assert_eq!(data.len(), 2);
        assert_eq!(w.data_as_u16(), Ok(0x0114));
    }

    #[test]
    fn packet_iterator() {
        let mut iter = MTData2PacketIter::new(&SEQ_WIRE_BYTES[..]).into_iter();
        let item = iter.next().unwrap();
        let p = item.unwrap();
        assert_eq!(p.data_id().data_type(), DataType::UtcTime);
        assert_eq!(p.data_length(), 12);
        let item = iter.next().unwrap();
        let p = item.unwrap();
        assert_eq!(p.data_id().data_type(), DataType::PacketCounter);
        assert_eq!(p.data_length(), 2);
        let item = iter.next().unwrap();
        let p = item.unwrap();
        assert_eq!(p.data_id().data_type(), DataType::SampleTimeFine);
        assert_eq!(p.data_length(), 4);
        let item = iter.next().unwrap();
        let p = item.unwrap();
        assert_eq!(p.data_id().data_type(), DataType::EulerAngles);
        assert_eq!(p.data_id().precision(), Precision::Float32);
        assert_eq!(p.data_id().coordinate_system(), CoordinateSystem::Ned);
        assert_eq!(p.data_length(), 12);
        let item = iter.next().unwrap();
        let p = item.unwrap();
        assert_eq!(p.data_id().data_type(), DataType::Acceleration);
        assert_eq!(p.data_id().precision(), Precision::Float32);
        assert_eq!(p.data_id().coordinate_system(), CoordinateSystem::Enu);
        assert_eq!(p.data_length(), 12);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.cursor, SEQ_WIRE_BYTES.len());
    }
}
