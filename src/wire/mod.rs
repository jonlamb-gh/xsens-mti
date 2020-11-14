pub mod output_config;
pub use output_config::*;
mod data_id;
pub use data_id::*;

pub(crate) type Field = ::core::ops::Range<usize>;
//pub(crate) type Rest = ::core::ops::RangeFrom<usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WireError {
    MissingBytes,
}
