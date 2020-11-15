use crate::message::{
    Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfig;

impl MessageExt for GoToConfig {
    const MSG_ID: MessageId = MessageId::new(0x30);
}

impl MessageEncode for GoToConfig {
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToConfig {
    fn decode_new(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToConfig)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfigAck;

impl MessageExt for GoToConfigAck {
    const MSG_ID: MessageId = MessageId::new(0x31);
}

impl MessageEncode for GoToConfigAck {
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToConfigAck {
    fn decode_new(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToConfigAck)
    }
}
