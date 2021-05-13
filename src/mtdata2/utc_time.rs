use crate::wire::WireError;
use bitfield::bitfield;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

/// The timestamp expressed as the UTC time
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UtcTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub ns: u32,
    pub flags: UtcTimeStatusFlags,
}

bitfield! {
    /// NOTE: Time until UTC flag (0x04) goes to valid takes 12.5 minutes. This time is
    /// needed to correct for the clock bias of the receiver. It is advised to start
    /// synchronization using UTC only when the UTC flag is valid.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(transparent)]
    pub struct UtcTimeStatusFlags(u8);
    /// Valid time of week
    pub time_of_week_valid, set_time_of_week_valid : 0;
    /// Valid week number
    pub week_number_valid, set_week_number_valid : 1;
    /// Valid UTC
    pub utc_valid, set_utc_valid : 2;
}

impl fmt::Display for UtcTimeStatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Flags(0x{:02X}: ToW({}), WN({}), UTC({}))",
            self.0,
            self.time_of_week_valid(),
            self.week_number_valid(),
            self.utc_valid()
        )
    }
}

impl fmt::Display for UtcTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{}",
            self.flags,
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.ns
        )
    }
}

mod field {
    use crate::wire::Field;

    pub const NS: Field = 0..4;
    pub const YEAR: Field = 4..6;
    pub const MONTH: usize = 6;
    pub const DAY: usize = 7;
    pub const HOUR: usize = 8;
    pub const MINUTE: usize = 9;
    pub const SECOND: usize = 10;
    pub const FLAGS: usize = 11;
}

impl UtcTime {
    pub const WIRE_SIZE: usize = 12;

    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        if bytes.len() < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            let ns = BigEndian::read_u32(&bytes[field::NS]);
            let year = BigEndian::read_u16(&bytes[field::YEAR]);
            let month = bytes[field::MONTH];
            let day = bytes[field::DAY];
            let hour = bytes[field::HOUR];
            let minute = bytes[field::MINUTE];
            let second = bytes[field::SECOND];
            let flags = bytes[field::FLAGS];
            Ok(UtcTime {
                year,
                month,
                day,
                hour,
                minute,
                second,
                ns,
                flags: UtcTimeStatusFlags(flags),
            })
        }
    }
}
