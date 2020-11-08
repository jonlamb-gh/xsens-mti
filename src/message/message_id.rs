use static_assertions::assert_eq_size;

assert_eq_size!(MessageId, u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MessageId(pub u8);

impl MessageId {
    pub const fn new(id: u8) -> Self {
        MessageId(id)
    }
}

impl From<u8> for MessageId {
    fn from(id: u8) -> Self {
        MessageId(id)
    }
}

impl From<MessageId> for u8 {
    fn from(id: MessageId) -> Self {
        id.0
    }
}
