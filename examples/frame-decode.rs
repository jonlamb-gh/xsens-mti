use embedded_hal::serial::Read;
use linux_embedded_hal::{nb::block, Serial};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use xsens_mti::decoder::Decoder;

// TODO - needs some work
//
// stty -F /dev/ttyUSB0 115200
fn main() -> Result<(), io::Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut buffer = vec![0_u8; 2048];
    let mut decoder = Decoder::new(&mut buffer).unwrap();

    let mut serial = Serial::open(Path::new("/dev/ttyUSB0"))?;

    while running.load(Ordering::SeqCst) {
        let byte = block!(serial.read())?;
        match decoder.decode(byte) {
            Ok(maybe_frame) => match maybe_frame {
                Some(f) => println!("{}", f),
                None => (),
            },
            Err(e) => eprintln!("Decoder error {:?}", e),
        }
    }

    println!("Count: {}", decoder.count());
    println!("Invalid count: {}", decoder.invalid_count());

    Ok(())
}
