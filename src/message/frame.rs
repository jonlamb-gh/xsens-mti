//! MT communication protocol framing
//!
//! All binary data communication is done in big-endian format

use crate::message::{BusId, MessageId, PayloadLength};
use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use core::mem;
use static_assertions::const_assert_eq;

const_assert_eq!(PayloadLength::MAX_STD, 254);
const_assert_eq!(PayloadLength::MAX_EXT, 2048);
const_assert_eq!(Frame::<&[u8]>::PREAMBLE_SIZE, mem::size_of::<u8>());
const_assert_eq!(Frame::<&[u8]>::HEADER_SIZE, mem::size_of::<u32>());
const_assert_eq!(Frame::<&[u8]>::EXT_LEN_SIZE, mem::size_of::<u16>());
const_assert_eq!(Frame::<&[u8]>::CHECKSUM_SIZE, mem::size_of::<u8>());

// TODO
// - generate_checksum helper
// - extended message tests
// - add a total frame size getter
// - proptest round trip
// - consider removing the protocol check in payload length making it infallible
//
// TODO impl Display
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FrameError {
    MissingHeader,
    MissingChecksum,
    InvalidPreamble,
    InvalidPayloadLength,
    IncompletePayload,
    InvalidChecksum,
}

#[derive(Debug, Clone)]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

impl<T: AsRef<[u8]>> fmt::Display for Frame<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusId(0x{:X}), MsgId(0x{:X}), Len({:?})",
            self.bus_id().0,
            self.message_id().0,
            self.payload_length().map_err(|_| fmt::Error)?
        )
    }
}

mod field {
    use crate::wire::{Field, Rest};

    // Header fields
    pub const PREAMBLE: usize = 0;
    pub const BUS_ID: usize = 1;
    pub const MSG_ID: usize = 2;
    pub const LEN: usize = 3;

    // Optional extended length
    pub const EXT_LEN: Field = 4..6;

    pub const PAYLOAD: Rest = 4..;
    pub const PAYLOAD_EXT: Rest = 6..;
    // Followed by 1 byte checksum
}

impl<T: AsRef<[u8]>> Frame<T> {
    pub const PREAMBLE: u8 = 0xFA;
    pub const STD_LEN_IS_EXT: u8 = 0xFF;

    // Does not include checksum byte
    pub const HEADER_SIZE: usize = 4;
    pub const EXT_HEADER_SIZE: usize = Self::HEADER_SIZE + Self::EXT_LEN_SIZE;
    pub const PREAMBLE_SIZE: usize = 1;
    pub const EXT_LEN_SIZE: usize = 2;
    pub const CHECKSUM_SIZE: usize = 1;
    pub const MAX_FRAME_SIZE: usize = Self::HEADER_SIZE
        + Self::EXT_LEN_SIZE
        + Self::CHECKSUM_SIZE
        + (PayloadLength::MAX_EXT as usize);

    pub fn new_unchecked(buffer: T) -> Frame<T> {
        Frame { buffer }
    }

    pub fn new(buffer: T) -> Result<Frame<T>, FrameError> {
        let f = Self::new_unchecked(buffer);
        f.check_len()?;
        f.check_preamble()?;
        f.check_payload_length()?;
        f.check_checksum()?;
        Ok(f)
    }

    pub fn check_len(&self) -> Result<(), FrameError> {
        let len = self.buffer.as_ref().len();
        if len < field::PAYLOAD.start {
            Err(FrameError::MissingHeader)
        } else if len < (field::PAYLOAD.start + Self::CHECKSUM_SIZE) {
            Err(FrameError::MissingChecksum)
        } else {
            Ok(())
        }
    }

    pub fn check_preamble(&self) -> Result<(), FrameError> {
        if self.preamble() != Frame::<&[u8]>::PREAMBLE {
            Err(FrameError::InvalidPreamble)
        } else {
            Ok(())
        }
    }

    pub fn check_payload_length(&self) -> Result<(), FrameError> {
        let payload_len = self.payload_length()?;
        let len = self.buffer.as_ref().len();
        if (len < Self::buffer_len(0))
            || (len < (payload_len.header_size() + payload_len.get() + Self::CHECKSUM_SIZE))
        {
            Err(FrameError::IncompletePayload)
        } else {
            Ok(())
        }
    }

    pub fn check_checksum(&self) -> Result<(), FrameError> {
        let computed = self.compute_checksum()?;
        if computed != 0 {
            Err(FrameError::InvalidChecksum)
        } else {
            Ok(())
        }
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }

    /// Return the length of a message header.
    ///
    /// Does not include extended length, payload or checksum byte.
    #[inline]
    pub fn header_len() -> usize {
        field::PAYLOAD.start
    }

    /// Return the length of a buffer required to hold a message
    /// with a payload length of `n_payload_bytes` + `CHECKSUM_SIZE`.
    #[inline]
    pub fn buffer_len(n_payload_bytes: usize) -> usize {
        let header_size = if n_payload_bytes > (PayloadLength::MAX_STD as usize) {
            field::PAYLOAD_EXT.start
        } else {
            field::PAYLOAD.start
        };
        header_size + n_payload_bytes + Self::CHECKSUM_SIZE
    }

    #[inline]
    pub fn preamble(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::PREAMBLE]
    }

    #[inline]
    pub fn bus_id(&self) -> BusId {
        let data = self.buffer.as_ref();
        BusId(data[field::BUS_ID])
    }

    #[inline]
    pub fn message_id(&self) -> MessageId {
        let data = self.buffer.as_ref();
        MessageId(data[field::MSG_ID])
    }

    #[inline]
    pub fn payload_length(&self) -> Result<PayloadLength, FrameError> {
        let data = self.buffer.as_ref();
        let std_len = data[field::LEN];
        if std_len == Self::STD_LEN_IS_EXT {
            if data.len() < Self::EXT_HEADER_SIZE {
                Err(FrameError::InvalidPayloadLength)
            } else {
                let ext_len = BigEndian::read_u16(&data[field::EXT_LEN]);
                if ext_len > PayloadLength::MAX_EXT {
                    Err(FrameError::InvalidPayloadLength)
                } else {
                    Ok(PayloadLength::Extended(ext_len))
                }
            }
        } else {
            Ok(PayloadLength::Standard(std_len))
        }
    }

    #[inline]
    pub fn checksum(&self) -> Result<u8, FrameError> {
        let payload_len = self.payload_length()?;
        let offset = payload_len.header_size() + payload_len.get();
        let data = self.buffer.as_ref();
        debug_assert!(data.len() >= offset);
        Ok(data[offset])
    }

    /// Sum of all bytes excluding the preamble
    #[inline]
    pub fn compute_checksum(&self) -> Result<u8, FrameError> {
        let payload_len = self.payload_length()?;
        let size = payload_len.header_size() + payload_len.get() + Self::CHECKSUM_SIZE;
        let data = self.buffer.as_ref();
        let mut sum = 0_u16;
        for b in data[Self::PREAMBLE_SIZE..size].iter() {
            sum = sum.wrapping_add(*b as u16);
        }
        Ok((sum & 0xFF) as u8)
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Frame<&'a T> {
    /// Return a pointer to the payload.
    ///
    /// Does not include the checksum byte.
    #[inline]
    pub fn payload(&self) -> Result<&'a [u8], FrameError> {
        let payload_len = self.payload_length()?;
        let end = payload_len.header_size() + payload_len.get();
        let data = self.buffer.as_ref();
        Ok(&data[field::PAYLOAD.start..end])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    #[inline]
    pub fn set_preamble(&mut self) {
        let data = self.buffer.as_mut();
        data[field::PREAMBLE] = Self::PREAMBLE;
    }

    #[inline]
    pub fn set_bus_id(&mut self, value: BusId) {
        let data = self.buffer.as_mut();
        data[field::BUS_ID] = value.into();
    }

    #[inline]
    pub fn set_message_id(&mut self, value: MessageId) {
        let data = self.buffer.as_mut();
        data[field::MSG_ID] = value.into();
    }

    #[inline]
    pub fn set_payload_length(&mut self, value: PayloadLength) {
        let data = self.buffer.as_mut();
        match value {
            PayloadLength::Standard(l) => data[field::LEN] = l,
            PayloadLength::Extended(l) => {
                data[field::LEN] = Self::STD_LEN_IS_EXT;
                BigEndian::write_u16(&mut data[field::EXT_LEN], l);
            }
        }
    }

    /// Return a mutable pointer to the payload.
    ///
    /// Does not include the checksum byte.
    #[inline]
    pub fn payload_mut(&mut self) -> Result<&mut [u8], FrameError> {
        let payload_len = self.payload_length()?;
        let end = payload_len.header_size() + payload_len.get();
        let data = self.buffer.as_mut();
        Ok(&mut data[field::PAYLOAD.start..end])
    }

    #[inline]
    pub fn set_checksum(&mut self, value: u8) -> Result<(), FrameError> {
        let payload_len = self.payload_length()?;
        let offset = payload_len.header_size() + payload_len.get();
        let data = self.buffer.as_mut();
        debug_assert!(data.len() >= offset);
        data[offset] = value;
        Ok(())
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Frame<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

impl PayloadLength {
    fn header_size(&self) -> usize {
        if self.is_extended() {
            Frame::<&[u8]>::HEADER_SIZE + Frame::<&[u8]>::EXT_LEN_SIZE
        } else {
            Frame::<&[u8]>::HEADER_SIZE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    static STD_MSG: [u8; 8] = [0xFA, 0xFF, 0x00, 0x03, 0x0A, 0x0B, 0x0C, 0xDD];
    static STD_MSG_PAYLOAD: [u8; 3] = [0x0A, 0x0B, 0x0C];

    #[test]
    fn header_len() {
        assert_eq!(Frame::<&[u8]>::header_len(), 4);
        let n_payload_bytes = 12;
        assert_eq!(
            Frame::<&[u8]>::buffer_len(n_payload_bytes),
            4 + 12 + Frame::<&[u8]>::CHECKSUM_SIZE
        );
        let n_payload_bytes = 1024;
        assert_eq!(
            Frame::<&[u8]>::buffer_len(n_payload_bytes),
            4 + 2 + 1024 + Frame::<&[u8]>::CHECKSUM_SIZE
        );
    }

    #[test]
    fn construct_std() {
        let mut bytes = [0xFF; 8];
        let mut f = Frame::new_unchecked(&mut bytes[..]);
        assert_eq!(f.check_len(), Ok(()));
        f.set_preamble();
        f.set_bus_id(BusId::MASTER);
        f.set_message_id(MessageId(0));
        f.set_payload_length(PayloadLength::new_standard(3).unwrap());
        assert_eq!(f.check_payload_length(), Ok(()));
        f.payload_mut()
            .unwrap()
            .copy_from_slice(&STD_MSG_PAYLOAD[..]);
        f.set_checksum(0xDD).unwrap();
        assert_eq!(f.check_preamble(), Ok(()));
        assert_eq!(f.check_payload_length(), Ok(()));
        assert_eq!(f.check_checksum(), Ok(()));
        assert_eq!(&f.into_inner()[..], &STD_MSG[..]);
    }

    #[test]
    fn deconstruct_std() {
        let f = Frame::new(&STD_MSG[..]).unwrap();
        assert_eq!(f.preamble(), Frame::<&[u8]>::PREAMBLE);
        assert_eq!(f.bus_id(), BusId::MASTER);
        assert_eq!(f.message_id(), MessageId(0));
        assert_eq!(
            f.payload_length(),
            Ok(PayloadLength::new_standard(3).unwrap())
        );
        assert_eq!(f.payload(), Ok(&STD_MSG_PAYLOAD[..]));
        assert_eq!(f.checksum(), Ok(0xDD));
    }

    #[test]
    fn missing_header() {
        let bytes = [0xFF; 4 - 1];
        assert_eq!(bytes.len(), Frame::<&[u8]>::header_len() - 1);
        let r = Frame::new(&bytes[..]);
        assert_eq!(r.unwrap_err(), FrameError::MissingHeader);
    }

    #[test]
    fn missing_checksum() {
        let bytes = [0xFA, 0xFF, 0x00, 0x00];
        let f = Frame::new(&bytes[..]);
        assert_eq!(f.unwrap_err(), FrameError::MissingChecksum);
    }

    #[test]
    fn invalid_preamble() {
        let bytes = [0xF0, 0xFF, 0x00, 0x00, 0x01];
        let f = Frame::new(&bytes[..]);
        assert_eq!(f.unwrap_err(), FrameError::InvalidPreamble);
    }

    #[test]
    fn invalid_payload_length() {
        let mut bytes = [0xFF; 4096];
        bytes[0] = 0xFA;
        bytes[1] = 0xFF;
        bytes[2] = 0x00;
        bytes[3] = Frame::<&[u8]>::STD_LEN_IS_EXT;
        BigEndian::write_u16(&mut bytes[4..6], PayloadLength::MAX_EXT + 1);
        let f = Frame::new(&bytes[..]);
        assert_eq!(f.unwrap_err(), FrameError::InvalidPayloadLength);
    }

    #[test]
    fn incomplete_payload() {
        let bytes = [0xFA, 0xFF, 0x00, 0xFE, 0x01];
        let f = Frame::new(&bytes[..]);
        assert_eq!(f.unwrap_err(), FrameError::IncompletePayload);
    }

    #[test]
    fn invalid_checksum() {
        let bytes = [0xFA, 0xFF, 0x00, 0x00, 0x01 + 1];
        let f = Frame::new(&bytes[..]);
        assert_eq!(f.unwrap_err(), FrameError::InvalidChecksum);
    }
}
