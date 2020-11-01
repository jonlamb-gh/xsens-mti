// TODO
// - warnings/etc
// - fixup the pub/private mod, prelude

#![no_std]
//#![deny(warnings, clippy::all)]

pub mod decoder;
pub mod message;

pub use message::{BusId, MessageId, PayloadLength};
