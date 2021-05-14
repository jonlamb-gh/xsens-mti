mod bus_id;
mod frame;
mod message_id;
mod payload_length;

pub use bus_id::BusId;
pub use frame::{Frame, FrameError};
pub use message_id::MessageId;
pub use payload_length::PayloadLength;

// TODO - this stuff needs a refactor

pub trait MessageExt {
    const MSG_ID: MessageId;

    fn message_id(&self) -> MessageId {
        Self::MSG_ID
    }
}

pub trait MessageEncode: MessageExt {
    // TODO - some sort of PayloadLength or size hint
    //
    // caller sets
    //   preamble, bus_id, checksum
    //
    // impl sets
    //   msg_id, payload_length, payload
    //   if payload != 0, impl should call check_payload_len too
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        frame.set_message_id(Self::MSG_ID);
        self.encode_frame(frame)
    }

    fn encode_frame(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError>;
}

pub trait MessageDecode<'buf>: MessageExt {
    // TODO - better method naming
    // caller checks msg id
    fn decode(frame: &Frame<&'buf [u8]>) -> Result<Self, FrameError>
    where
        Self: Sized,
    {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Self::decode_frame(frame)
    }

    fn decode_frame(frame: &Frame<&'buf [u8]>) -> Result<Self, FrameError>
    where
        Self: Sized;
}
