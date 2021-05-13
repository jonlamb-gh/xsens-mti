pub mod data_id;
pub mod mtdata2_packet;
pub mod output_config;

pub use data_id::*;
pub use mtdata2_packet::*;
pub use output_config::*;

pub(crate) type Field = ::core::ops::Range<usize>;
pub(crate) type Rest = ::core::ops::RangeFrom<usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WireError {
    MissingBytes,
}
