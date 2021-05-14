#![no_main]

use libfuzzer_sys::fuzz_target;
use xsens_mti::prelude::*;

fuzz_target!(|data: &[u8]| {
    let mut dec_buffer = vec![0_u8; Frame::<&[u8]>::MAX_FRAME_SIZE];
    let mut decoder = Decoder::new(&mut dec_buffer).unwrap();
    for b in data.iter() {
        let _ = decoder.decode(*b);
    }
});
