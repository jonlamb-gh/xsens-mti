#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PayloadLength {
    Standard(u8),
    Extended(u16),
}

impl PayloadLength {
    pub const MAX_STD: u8 = 0xFE;
    pub const MAX_EXT: u16 = 0x0800;

    pub const fn new_standard(len: u8) -> Option<Self> {
        if len > Self::MAX_STD {
            None
        } else {
            Some(PayloadLength::Standard(len))
        }
    }

    pub const fn new_extended(len: u16) -> Option<Self> {
        if len > Self::MAX_EXT {
            None
        } else {
            Some(PayloadLength::Extended(len))
        }
    }

    pub const fn new(len: usize) -> Option<Self> {
        if len > Self::MAX_EXT as usize {
            None
        } else if len > Self::MAX_STD as usize {
            Some(PayloadLength::Extended(len as _))
        } else {
            Some(PayloadLength::Standard(len as _))
        }
    }

    pub fn get(&self) -> usize {
        match self {
            PayloadLength::Standard(l) => *l as usize,
            PayloadLength::Extended(l) => *l as usize,
        }
    }

    pub fn is_extended(&self) -> bool {
        matches!(self, PayloadLength::Extended(_))
    }
}
