#![no_main]

use libfuzzer_sys::{arbitrary, fuzz_target};
use xsens_mti::prelude::*;

#[derive(arbitrary::Arbitrary, Debug)]
struct ArbDataId(u16);

#[derive(arbitrary::Arbitrary, Debug)]
struct Input {
    data_id: ArbDataId,
    data: Vec<u8>,
}

fuzz_target!(|input: Input| {
    if input.data.len() > Frame::<&[u8]>::MAX_FRAME_SIZE {
        return;
    }
    let data_type = DataId::from(input.data_id.0).data_type();
    let bytes = &input.data[..];
    match data_type {
        DataType::Temperature => (), // TODO no type for this yet
        DataType::UtcTime => {
            let _ = UtcTime::from_be_slice(bytes);
        }
        DataType::PacketCounter => {
            let _ = PacketCounter::from_be_slice(bytes);
        }
        DataType::SampleTimeFine => {
            let _ = SampleTimeFine::from_be_slice(bytes);
        }
        DataType::SampleTimeCoarse => {
            let _ = SampleTimeCoarse::from_be_slice(bytes);
        }
        DataType::Quaternion => (), // TODO no type for this yet
        DataType::EulerAngles => {
            let _ = EulerAngles::from_be_slice(bytes);
        }
        DataType::Acceleration => {
            let _ = Acceleration::from_be_slice(bytes);
        }
        DataType::AltitudeEllipsoid => {
            let _ = AltitudeEllipsoid::from_be_slice(bytes);
        }
        DataType::PositionEcef => {
            let _ = PositionEcef::from_be_slice(bytes);
        }
        DataType::LatLon => {
            let _ = LatLon::from_be_slice(bytes);
        }
        DataType::RateOfTurn => {
            let _ = RateOfTurn::from_be_slice(bytes);
        }
        DataType::VelocityXYZ => {
            let _ = VelocityXYZ::from_be_slice(bytes);
        }
        DataType::StatusByte => (), // TODO no type for this yet
        DataType::StatusWord => {
            let _ = StatusWord::from_be_slice(bytes);
        }
        DataType::Unknown(_) => (),
    }
});
