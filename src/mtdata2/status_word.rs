use crate::wire::WireError;
use bitfield::bitfield;
use byteorder::{BigEndian, ByteOrder};
use core::fmt;

bitfield! {
    /// Contains the 32bit status word
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(transparent)]
    pub struct StatusWord(u32);
    /// This flag indicates if the MT passed the self-test according to
    /// eMTS. For an up-to-date result of the self-test, use the command
    /// (RunSelftest). This flag is inactive (0) for the MTi 600-series.
    pub self_test, set_self_test : 0;
    /// This flag indicates if input into the orientation filter is reliable and
    /// or complete. If for example the measurement range of internal
    /// sensors is exceeded, orientation output cannot be reliably
    /// estimated and the filter flag will drop to 0. For GNSS/INS
    /// devices, the filter flag will also becom invalid if the GPS status
    /// remains invalid for an extended period
    pub filter_valid, set_filter_valid : 1;
    /// This flag indicates if the GNSS unit has a proper fix. The flag is
    /// only available in GNSS/INS units.
    pub gnss_fix, set_gnss_fix : 2;

    // TODO all the other bits on page 53
}

impl fmt::Display for StatusWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StatusWord(0x{:04X}: SelfTest({}), FilterValid({}), GNSSFix({}), ...)",
            self.0,
            self.self_test(),
            self.filter_valid(),
            self.gnss_fix()
        )
    }
}

mod field {
    use crate::wire::{field32, Field};

    pub const SW: Field = field32::F0;
}

impl StatusWord {
    pub const WIRE_SIZE: usize = 4;

    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        if bytes.len() < Self::WIRE_SIZE {
            Err(WireError::MissingBytes)
        } else {
            let sw = BigEndian::read_u32(&bytes[field::SW]);
            Ok(StatusWord(sw))
        }
    }
}
