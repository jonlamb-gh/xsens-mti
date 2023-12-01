#![no_main]
#![deny(warnings, clippy::all)]

use libfuzzer_sys::{arbitrary, fuzz_target};
use xsens_mti::prelude::*;

mod test_support;

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
    let data_id = DataId::from(input.data_id.0);
    let data_type = data_id.data_type();
    let precision = data_id.precision();
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
            parse_any_precision_variant_from_be_slice!(EulerAngles, precision, bytes);
        }
        DataType::Acceleration => {
            parse_any_precision_variant_from_be_slice!(Acceleration, precision, bytes);
        }
        DataType::AltitudeEllipsoid => {
            parse_any_precision_variant_from_be_slice!(AltitudeEllipsoid, precision, bytes);
        }
        DataType::PositionEcef => {
            parse_float_precision_variant_from_be_slice!(PositionEcef, precision, bytes);
        }
        DataType::LatLon => {
            parse_any_precision_variant_from_be_slice!(LatLon, precision, bytes);
        }
        DataType::RateOfTurn => {
            parse_any_precision_variant_from_be_slice!(RateOfTurn, precision, bytes);
        }
        DataType::MagneticField => {
            parse_any_precision_variant_from_be_slice!(MagneticField, precision, bytes);
        }
        DataType::VelocityXYZ => {
            parse_any_precision_variant_from_be_slice!(VelocityXYZ, precision, bytes);
        }
        DataType::StatusByte => (), // TODO no type for this yet
        DataType::StatusWord => {
            let _ = StatusWord::from_be_slice(bytes);
        }
        DataType::Unknown(_) => (),
        _ => (),
    }
});
