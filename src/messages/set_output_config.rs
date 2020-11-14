use crate::{
    message::{
        Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
    },
    wire::{DataId, OutputConfiguration},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetOutputConfiguration<T: AsRef<[OutputConfiguration]>> {
    settings: T,
}

impl<T: AsRef<[OutputConfiguration]>> MessageExt for SetOutputConfiguration<T> {
    const MSG_ID: MessageId = MessageId::new(0xC0);
}

impl<T: AsRef<[OutputConfiguration]>> SetOutputConfiguration<T> {
    pub const MAX_SETTINGS: usize = 32;
}
