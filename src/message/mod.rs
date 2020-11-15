mod bus_id;
mod frame;
mod message_id;
mod payload_length;

pub use bus_id::BusId;
pub use frame::{Frame, FrameError};
pub use message_id::MessageId;
pub use payload_length::PayloadLength;

pub trait MessageExt {
    const MSG_ID: MessageId;

    fn message_id(&self) -> MessageId {
        Self::MSG_ID
    }
}

pub trait MessageEncode: MessageExt {
    // TODO - some sort of PayloadLength or size hint

    // caller sets
    //   preamble, bus_id, checksum
    //
    // impl sets
    //   msg_id, payload_length, payload
    //   if payload != 0, impl should call check_payload_len too
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError>;
}

pub trait MessageDecode<'buf>: MessageExt {
    // TODO - better method naming
    // caller checks msg id
    fn decode(frame: &Frame<&'buf [u8]>) -> Result<Self, FrameError>
    where
        Self: Sized,
    {
        debug_assert_eq!(frame.message_id(), Self::MSG_ID);
        Self::decode_new(frame)
    }

    fn decode_new(frame: &Frame<&'buf [u8]>) -> Result<Self, FrameError>
    where
        Self: Sized;

    //fn decode_into<T: AsRef<[u8]>>(&mut self, frame: &Frame<T>) -> Result<(), FrameError>;
}
