use crate::message::{Frame, FrameError, PayloadLength};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    InsufficientBufferSize,
    FrameError(FrameError),
}

impl From<FrameError> for Error {
    fn from(e: FrameError) -> Self {
        Error::FrameError(e)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum State {
    Preamble,
    BusId,
    MsgId,
    Len,
    ExtLenMsb,
    ExtLenLsb,
    Payload,
    Checksum,
}

impl Default for State {
    fn default() -> Self {
        State::Preamble
    }
}

#[derive(Debug)]
pub struct Decoder<'a> {
    state: State,
    count: usize,
    invalid_count: usize,
    accumulated_checksum: u16,
    raw_payload_len: u16,
    expected_frame_size: usize,
    bytes_read: usize,
    buffer: &'a mut [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Result<Self, Error> {
        if buffer.len() < Frame::<&[u8]>::HEADER_SIZE + (PayloadLength::MAX_STD as usize) {
            Err(Error::InsufficientBufferSize)
        } else {
            Ok(Decoder {
                state: State::default(),
                count: 0,
                invalid_count: 0,
                accumulated_checksum: 0,
                raw_payload_len: 0,
                expected_frame_size: 0,
                bytes_read: 0,
                buffer,
            })
        }
    }

    pub fn reset(&mut self) {
        self.state = State::default();
        self.accumulated_checksum = 0;
        self.raw_payload_len = 0;
        self.expected_frame_size = 0;
        self.bytes_read = 0;
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn invalid_count(&self) -> usize {
        self.invalid_count
    }

    pub fn decode(&mut self, byte: u8) -> Result<Option<Frame<&[u8]>>, Error> {
        match self.state {
            State::Preamble => {
                if byte == Frame::<&[u8]>::PREAMBLE {
                    self.feed(byte)?;
                    // Checksum doesn't include preamble
                    self.accumulated_checksum = 0;
                    self.state = State::BusId;
                } else {
                    self.reset();
                }
            }
            State::BusId => {
                self.feed(byte)?;
                self.state = State::MsgId;
            }
            State::MsgId => {
                self.feed(byte)?;
                self.state = State::Len;
            }
            State::Len => {
                self.feed(byte)?;
                if byte == 0 {
                    // Message with no payload
                    self.raw_payload_len = 0;
                    self.expected_frame_size =
                        Frame::<&[u8]>::HEADER_SIZE + Frame::<&[u8]>::CHECKSUM_SIZE;
                    self.state = State::Checksum;
                } else if byte == Frame::<&[u8]>::STD_LEN_IS_EXT {
                    // Message with extended payload
                    self.state = State::ExtLenMsb;
                } else {
                    // Message with standard payload
                    self.raw_payload_len = byte as u16;
                    self.expected_frame_size = Frame::<&[u8]>::HEADER_SIZE
                        + Frame::<&[u8]>::CHECKSUM_SIZE
                        + (byte as usize);
                    self.state = State::Payload;
                }
            }
            State::ExtLenMsb => {
                self.feed(byte)?;
                self.raw_payload_len = byte as u16;
                self.state = State::ExtLenLsb;
            }
            State::ExtLenLsb => {
                self.feed(byte)?;
                // Msb stored in self.raw_payload_len in State::ExtLenMsb
                self.raw_payload_len = u16::from_be_bytes([self.raw_payload_len as u8, byte]);
                if self.raw_payload_len > PayloadLength::MAX_EXT {
                    self.reset();
                    self.inc_invalid_count();
                } else {
                    self.expected_frame_size = Frame::<&[u8]>::EXT_HEADER_SIZE
                        + Frame::<&[u8]>::CHECKSUM_SIZE
                        + (self.raw_payload_len as usize);
                }
                self.state = State::Payload;
            }
            State::Payload => {
                self.feed(byte)?;
                if self.bytes_read.saturating_add(1) >= self.expected_frame_size {
                    self.state = State::Checksum;
                }
            }
            State::Checksum => {
                self.feed(byte)?;
                let accumulated_checksum = self.accumulated_checksum;
                let bytes_read = self.bytes_read;
                self.reset();
                if accumulated_checksum.trailing_zeros() >= 8 {
                    self.inc_count();
                    // TODO - map err inc err counter, ok inc valid counter
                    let f = Frame::new(&self.buffer[..bytes_read])?;
                    return Ok(Some(f));
                } else {
                    self.inc_invalid_count();
                }
            }
        }
        Ok(None)
    }

    #[inline]
    fn feed(&mut self, byte: u8) -> Result<(), Error> {
        if self.bytes_read >= self.buffer.len() {
            Err(Error::InsufficientBufferSize)
        } else {
            self.accumulated_checksum = self.accumulated_checksum.wrapping_add(byte as u16);
            self.buffer[self.bytes_read] = byte;
            self.bytes_read = self.bytes_read.saturating_add(1);
            Ok(())
        }
    }

    #[inline]
    fn inc_count(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    #[inline]
    fn inc_invalid_count(&mut self) {
        self.invalid_count = self.invalid_count.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    static STD_MSG: [u8; 8] = [0xFA, 0xFF, 0x00, 0x03, 0x0A, 0x0B, 0x0C, 0xDD];

    #[test]
    fn basic_decoding() {
        let mut buffer = [0_u8; 512];
        let mut dec = Decoder::new(&mut buffer[..]).unwrap();

        for _ in 0..4 {
            for (idx, byte) in STD_MSG.iter().enumerate() {
                let maybe_frame = dec.decode(*byte).unwrap();
                if idx < (STD_MSG.len() - 1) {
                    assert_eq!(maybe_frame.is_some(), false);
                } else {
                    assert_eq!(maybe_frame.is_some(), true);
                }
            }
        }

        assert_eq!(dec.count, 4);
        assert_eq!(dec.invalid_count, 0);
    }
}
