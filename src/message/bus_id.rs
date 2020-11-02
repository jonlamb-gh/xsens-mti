use static_assertions::assert_eq_size;

assert_eq_size!(BusId, u8);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BusId(pub u8);

impl BusId {
    /// Indicating "first device"
    pub const SELF: Self = BusId(1);

    pub const MASTER: Self = BusId(0xFF);

    pub const fn new(id: u8) -> Self {
        BusId(id)
    }
}

impl From<u8> for BusId {
    fn from(id: u8) -> Self {
        BusId(id)
    }
}

impl From<BusId> for u8 {
    fn from(id: BusId) -> Self {
        id.0
    }
}
