use crate::message::{
    Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfig;

impl MessageExt for GoToConfig {
    const MSG_ID: MessageId = MessageId::new(0x30);
}

impl MessageEncode for GoToConfig {
    fn encode<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode for GoToConfig {
    fn decode<T: AsRef<[u8]>>(frame: &Frame<T>) -> Result<Self, FrameError>
    where
        Self: Sized,
    {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Ok(GoToConfig)
    }

    fn decode_into<T: AsRef<[u8]>>(&mut self, frame: &Frame<T>) -> Result<(), FrameError> {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfigAck;

impl MessageExt for GoToConfigAck {
    const MSG_ID: MessageId = MessageId::new(0x31);
}

impl MessageEncode for GoToConfigAck {
    fn encode<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode for GoToConfigAck {
    fn decode<T: AsRef<[u8]>>(frame: &Frame<T>) -> Result<Self, FrameError>
    where
        Self: Sized,
    {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Ok(GoToConfigAck)
    }

    fn decode_into<T: AsRef<[u8]>>(&mut self, frame: &Frame<T>) -> Result<(), FrameError> {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Ok(())
    }
}
