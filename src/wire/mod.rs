pub mod data_id;
pub mod mtdata2_packet;
pub mod output_config;

pub use data_id::*;
pub use mtdata2_packet::*;
pub use output_config::*;

pub(crate) type Field = ::core::ops::Range<usize>;
pub(crate) type Rest = ::core::ops::RangeFrom<usize>;

pub(crate) mod field32 {
    use super::Field;

    pub const F0: Field = 0..4;
    pub const F1: Field = 4..8;
    pub const F2: Field = 8..12;
}

pub(crate) mod field64 {
    use super::Field;

    pub const F0: Field = 0..8;
    pub const F1: Field = 8..16;
    pub const F2: Field = 16..24;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, err_derive::Error)]
pub enum WireError {
    #[error(display = "Missing bytes")]
    MissingBytes,
}
