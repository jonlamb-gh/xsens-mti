use crate::message::{Frame, FrameError, MessageEncode, MessageExt, MessageId, PayloadLength};

/// Switch the active state of the device from Measurement State to Config State.
/// This message can also be used in Config State to confirm that Config State is
/// currently the active state.
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
