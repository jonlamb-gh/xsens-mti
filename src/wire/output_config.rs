use crate::wire::DataId;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputFrequency(pub u16);

impl OutputFrequency {
    pub const MAX: Self = OutputFrequency(0xFFFF);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputConfiguration {
    pub data_id: DataId,
    pub output_frequency: u16,
}
