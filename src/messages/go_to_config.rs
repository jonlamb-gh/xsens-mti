use crate::message::{
    Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
};

/// Switch the active state of the device from Measurement State to Config State.
/// This message can also be used in Config State to confirm that Config State is
/// currently the active state.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfig;

impl MessageExt for GoToConfig {
    const MSG_ID: MessageId = MessageId::new(0x30);
}

impl MessageEncode for GoToConfig {
    fn encode_frame(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToConfig {
    fn decode_frame(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToConfig)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToConfigAck;

impl MessageExt for GoToConfigAck {
    const MSG_ID: MessageId = MessageId::new(0x31);
}

impl MessageEncode for GoToConfigAck {
    fn encode_frame(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToConfigAck {
    fn decode_frame(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToConfigAck)
    }
}
