use crate::wire::WireError;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// This counter is incremented with every generated MTData2 message
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PacketCounter(pub u16);

impl fmt::Display for PacketCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod field {
    use crate::wire::Field;

    pub const PC: Field = 0..2;
}

impl PacketCounter {
    pub const WIRE_SIZE: usize = 2;

    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        if bytes.len() < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            let pc = BigEndian::read_u16(&bytes[field::PC]);
            Ok(PacketCounter(pc))
        }
    }
}
