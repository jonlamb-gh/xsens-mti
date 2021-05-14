// TODO
// - warnings/etc
// - fixup the pub/private mod, prelude
// - add mixed-endian tests/etc
// - add some address sanitizers to the testing, probably use the fuzz testing framework
// - reorganize the submodules/types

#![no_std]
#![deny(warnings, clippy::all)]

#[macro_use]
mod macros;

pub mod decoder;
pub mod message;
pub mod messages;
pub mod mtdata2;
pub mod prelude;
pub mod wire;
