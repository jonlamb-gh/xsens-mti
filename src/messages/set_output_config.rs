use crate::{
    message::{
        Frame, FrameError, MessageDecode, MessageEncode, MessageExt, MessageId, PayloadLength,
    },
    wire::{OutputConfiguration, WireOutputConfiguration, WireOutputConfigurationIterator},
};
use core::cmp;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TooManyOutputConifgurations;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetOutputConfiguration<T: AsRef<[OutputConfiguration]>> {
    settings: T,
}

impl<T: AsRef<[OutputConfiguration]>> MessageExt for SetOutputConfiguration<T> {
    const MSG_ID: MessageId = MessageId::new(0xC0);
}

impl<T: AsRef<[OutputConfiguration]>> SetOutputConfiguration<T> {
    pub const MAX_SETTINGS: usize = 32;

    pub fn new_unchecked(settings: T) -> Self {
        SetOutputConfiguration { settings }
    }

    pub fn new(settings: T) -> Result<Self, TooManyOutputConifgurations> {
        let m = Self::new_unchecked(settings);
        m.check_len()?;
        Ok(m)
    }

    pub fn check_len(&self) -> Result<(), TooManyOutputConifgurations> {
        let len = self.settings.as_ref().len();
        if len > Self::MAX_SETTINGS {
            Err(TooManyOutputConifgurations)
        } else {
            Ok(())
        }
    }
}

impl<C: AsRef<[OutputConfiguration]>> MessageEncode for SetOutputConfiguration<C> {
    fn encode(&self, frame: &mut Frame<&mut [u8]>) -> Result<(), FrameError> {
        let settings = self.settings.as_ref();
        let config_size = WireOutputConfiguration::<&[u8]>::WIRE_SIZE;
        let num_settings = cmp::min(Self::MAX_SETTINGS, settings.len());
        let min_size = config_size * num_settings;
        frame.set_message_id(Self::MSG_ID);
        frame.set_payload_length(
            PayloadLength::new(min_size).ok_or(FrameError::InvalidPayloadLength)?,
        );
        frame.check_payload_length()?;
        for (src, dst) in settings
            .iter()
            .zip(frame.payload_mut()?.chunks_exact_mut(config_size))
        {
            let mut c = WireOutputConfiguration::new_unchecked(dst);
            c.set_output_configuration(*src);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetOutputConfigurationAck<'a>(pub WireOutputConfigurationIterator<'a>);

impl<'a> MessageExt for SetOutputConfigurationAck<'a> {
    const MSG_ID: MessageId = MessageId::new(0xC1);
}

impl<'a> MessageDecode<'a> for SetOutputConfigurationAck<'a> {
    fn decode_new(frame: &Frame<&'a [u8]>) -> Result<Self, FrameError> {
        let payload = frame.payload()?;
        Ok(SetOutputConfigurationAck(
            WireOutputConfigurationIterator::new(payload),
        ))
    }
}
