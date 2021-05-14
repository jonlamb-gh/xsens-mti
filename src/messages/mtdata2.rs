//
// TODO hold a ref to the frame data
// impl some iterator over the packets
// some wire type for MTData2Packet, iterator yields them?
// [<data-id>, <packet>]
//
// wire type has a wire type iter, each item can be converted to the real type
//
// WireDataId/DataId should have a wire_packet_length method of sorts, known at compile time,
// checked at runtime?
//
// mtdata2 on page 46

use crate::{
    message::{Frame, FrameError, MessageDecode, MessageExt, MessageId},
    wire::{MTData2Packet, MTData2PacketIter, WireError},
};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MTData2<'a>(pub MTData2PacketIter<'a>);

impl<'a> MessageExt for MTData2<'a> {
    const MSG_ID: MessageId = MessageId::new(0x36);
}

impl<'a> MessageDecode<'a> for MTData2<'a> {
    fn decode_frame(frame: &Frame<&'a [u8]>) -> Result<Self, FrameError> {
        let payload = frame.payload()?;
        Ok(MTData2(MTData2PacketIter::new(payload)))
    }
}

impl<'a> IntoIterator for MTData2<'a> {
    type Item = Result<MTData2Packet<&'a [u8]>, WireError>;
    type IntoIter = MTData2PacketIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}
