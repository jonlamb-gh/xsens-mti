use crate::wire::{DataId, WireDataId, WireError};
use byteorder::{BigEndian, ByteOrder};
use core::mem;
use static_assertions::const_assert_eq;

const_assert_eq!(
    WireOutputConfiguration::<&[u8]>::WIRE_SIZE,
    mem::size_of::<u32>()
);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputFrequency(pub u16);

impl OutputFrequency {
    /// 0 or 0xFFFF means max frequency
    pub const MAX: Self = OutputFrequency(0xFFFF);
}

impl From<u16> for OutputFrequency {
    fn from(value: u16) -> Self {
        OutputFrequency(value)
    }
}

impl From<OutputFrequency> for u16 {
    fn from(value: OutputFrequency) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputConfiguration {
    pub data_id: DataId,
    pub output_frequency: OutputFrequency,
}

impl OutputConfiguration {
    pub fn new(data_id: DataId, output_frequency: OutputFrequency) -> Self {
        OutputConfiguration {
            data_id,
            output_frequency,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WireOutputConfiguration<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use super::WireDataId;
    use crate::wire::Field;

    pub const DATA_ID: Field = 0..WireDataId::<&[u8]>::WIRE_SIZE;
    pub const FREQ: Field = 2..4;
}

impl<T: AsRef<[u8]>> WireOutputConfiguration<T> {
    pub const WIRE_SIZE: usize = WireDataId::<&[u8]>::WIRE_SIZE + mem::size_of::<u16>();

    pub fn new_unchecked(buffer: T) -> WireOutputConfiguration<T> {
        WireOutputConfiguration { buffer }
    }

    pub fn new(buffer: T) -> Result<WireOutputConfiguration<T>, WireError> {
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

    #[inline]
    pub fn output_frequency(&self) -> OutputFrequency {
        let data = self.buffer.as_ref();
        BigEndian::read_u16(&data[field::FREQ]).into()
    }

    #[inline]
    pub fn output_configuration(&self) -> OutputConfiguration {
        OutputConfiguration {
            data_id: self.data_id(),
            output_frequency: self.output_frequency(),
        }
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> WireOutputConfiguration<T> {
    #[inline]
    pub fn set_data_id(&mut self, value: DataId) {
        let data = self.buffer.as_mut();
        BigEndian::write_u16(&mut data[field::DATA_ID], u16::from(value));
    }

    #[inline]
    pub fn set_output_frequency(&mut self, value: OutputFrequency) {
        let data = self.buffer.as_mut();
        BigEndian::write_u16(&mut data[field::FREQ], value.into());
    }

    #[inline]
    pub fn set_output_configuration(&mut self, value: OutputConfiguration) {
        self.set_data_id(value.data_id);
        self.set_output_frequency(value.output_frequency);
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for WireOutputConfiguration<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WireOutputConfigurationIterator<'a> {
    buffer: &'a [u8],
}

impl<'a> WireOutputConfigurationIterator<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        WireOutputConfigurationIterator { buffer }
    }

    pub fn iter(&self) -> impl Iterator<Item = OutputConfiguration> + 'a {
        self.buffer
            .chunks_exact(WireOutputConfiguration::<&[u8]>::WIRE_SIZE)
            // unchecked ok, only an error if not WIRE_SIZE bytes in length
            .map(|chunk| WireOutputConfiguration::new_unchecked(chunk).output_configuration())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::data_id::{CoordinateSystem, DataType, Precision};
    use pretty_assertions::assert_eq;

    static WIRE_BYTES: [u8; 4] = [0x40, 0x20, 0x01, 0x90];

    #[rustfmt::skip]
    static SEQ_BYTES: [u8; 16] = [
        0x10, 0x20, 0xFF, 0xFF,
        0x10, 0x60, 0xFF, 0xFF,
        0x20, 0x10, 0x00, 0x64,
        0x40, 0x20, 0x01, 0x90,
    ];

    #[test]
    fn buffer_len() {
        assert_eq!(
            WireOutputConfiguration::<&[u8]>::buffer_len(),
            mem::size_of::<u32>()
        );
        assert_eq!(
            WireOutputConfiguration::<&[u8]>::WIRE_SIZE,
            mem::size_of::<u32>()
        );
    }

    #[test]
    fn construct() {
        let mut bytes = [0xFF; 4];
        let mut w = WireOutputConfiguration::new_unchecked(&mut bytes[..]);
        assert_eq!(w.check_len(), Ok(()));
        w.set_output_configuration(OutputConfiguration::new(
            DataId::new(
                DataType::Acceleration,
                Precision::Float32,
                CoordinateSystem::Enu,
            ),
            OutputFrequency(400),
        ));
        assert_eq!(&w.into_inner()[..], &WIRE_BYTES[..]);
    }

    #[test]
    fn deconstruct() {
        let w = WireOutputConfiguration::new(&WIRE_BYTES[..]).unwrap();
        assert_eq!(
            w.output_configuration(),
            OutputConfiguration::new(
                DataId::new(
                    DataType::Acceleration,
                    Precision::Float32,
                    CoordinateSystem::Enu,
                ),
                OutputFrequency(400),
            )
        );
    }

    #[test]
    fn missing_bytes() {
        let bytes = [0xFF; 4 - 1];
        assert_eq!(
            bytes.len(),
            WireOutputConfiguration::<&[u8]>::buffer_len() - 1
        );
        let w = WireOutputConfiguration::new(&bytes[..]);
        assert_eq!(w.unwrap_err(), WireError::MissingBytes);
    }

    #[test]
    fn seq_iter() {
        let expected = [
            OutputConfiguration::new(
                DataId::from_data_type(DataType::PacketCounter),
                OutputFrequency::MAX,
            ),
            OutputConfiguration::new(
                DataId::from_data_type(DataType::SampleTimeFine),
                OutputFrequency::MAX,
            ),
            OutputConfiguration::new(
                DataId::from_data_type(DataType::Quaternion),
                OutputFrequency(100),
            ),
            OutputConfiguration::new(
                DataId::from_data_type(DataType::Acceleration),
                OutputFrequency(400),
            ),
        ];
        let s = WireOutputConfigurationIterator::new(&SEQ_BYTES[..]);
        for (a, b) in s.iter().zip(expected.iter()) {
            assert_eq!(a, *b);
        }
    }
}
