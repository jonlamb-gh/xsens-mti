use crate::message::{
    Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToMeasurement;

impl MessageExt for GoToMeasurement {
    const MSG_ID: MessageId = MessageId::new(0x10);
}

impl MessageEncode for GoToMeasurement {
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToMeasurement {
    fn decode_new(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToMeasurement)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GoToMeasurementAck;

impl MessageExt for GoToMeasurementAck {
    const MSG_ID: MessageId = MessageId::new(0x11);
}

impl MessageEncode for GoToMeasurementAck {
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(PayloadLength::Standard(0));
        Ok(())
    }
}

impl MessageDecode<'_> for GoToMeasurementAck {
    fn decode_new(_frame: &Frame<&[u8]>) -> Result<Self, FrameError> {
        Ok(GoToMeasurementAck)
    }
}
