use serial::prelude::*;
use std::io::{self, Read};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use xsens_mti::decoder::Decoder;

fn main() -> Result<(), io::Error> {
    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            println!("Shutting down");
        } else {
            println!("Force exit");
            process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");

    let mut read_buffer = vec![0_u8; 2048];
    let mut dec_buffer = vec![0_u8; 2048];
    let mut decoder = Decoder::new(&mut dec_buffer).unwrap();

    let mut port = serial::open("/dev/ttyUSB0")?;

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(5000))?;

    while running.load(Ordering::SeqCst) == 0 {
        let bytes_read = match port.read(&mut read_buffer) {
            Ok(cnt) => cnt,
            Err(e) => {
                if matches!(e.kind(), io::ErrorKind::Interrupted) {
                    0
                } else {
                    return Err(e);
                }
            }
        };
        for byte in read_buffer[..bytes_read].iter() {
            match decoder.decode(*byte) {
                Ok(maybe_frame) => match maybe_frame {
                    Some(f) => println!("{}", f),
                    None => (),
                },
                Err(e) => eprintln!("Decoder error {:?}", e),
            }
        }
    }

    println!("Count: {}", decoder.count());
    println!("Invalid count: {}", decoder.invalid_count());

    Ok(())
}
