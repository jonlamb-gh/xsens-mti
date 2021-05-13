#![no_main]

use libfuzzer_sys::fuzz_target;
use xsens_mti::prelude::*;

fuzz_target!(|data: &[u8]| {
    let f = match Frame::new(data) {
        Ok(f) => f,
        Err(_) => return,
    };
    let _ = f.preamble();
    let _ = f.bus_id();
    let _ = f.message_id();
    let _ = f.payload_length();
    let _ = f.checksum();
    let _ = f.compute_checksum();
    let _ = f.payload();
});
