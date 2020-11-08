mod bus_id;
pub use bus_id::BusId;
mod frame;
pub use frame::{Frame, FrameError};
mod message_id;
pub use message_id::MessageId;
mod payload_length;
pub use payload_length::PayloadLength;

// TODO
// some high level Message enum for easy decoding
// decode trait has decode for easy on the stack allocation
//   and decode_into() for alt pre-allocated stuff,
//   maybe separate traits
// probably need some enc/dec error types
//   decode/decode_into on a msg with different id doesn't fit into the FrameError type
//   caller should check it for now

pub trait MessageExt {
    const MSG_ID: MessageId;
}

pub trait MessageEncode: MessageExt {
    // TODO - some sort of PayloadLength or size hint

    // caller sets
    //   preamble, bus_id, checksum
    //
    // impl sets
    //   msg_id, payload_length, payload
    //   if payload != 0, impl should call check_payload_len too
    fn encode<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) -> Result<(), FrameError>;
}

pub trait MessageDecode: MessageExt {
    fn decode<T: AsRef<[u8]>>(frame: &Frame<T>) -> Result<Self, FrameError>
    where
        Self: Sized;

    fn decode_into<T: AsRef<[u8]>>(&mut self, frame: &Frame<T>) -> Result<(), FrameError>;
}
