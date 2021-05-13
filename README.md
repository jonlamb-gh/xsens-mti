# xsens-mti

An unofficial and incomplete `no_std` Rust driver for Xsens MTi-series sensors.

## Example

```bash
cargo run --example frame-decode

BusId(0xFF), MsgId(0x36), Len(Standard(158))
  MTData2
    [0] DataId(0x1010, UtcTime, Float32, Enu), Len(12)
      Flags(0x07: ToW(true), WN(true), UTC(true)) 2021-05-13 12:05:37.4500000
    [1] DataId(0x1020, PacketCounter, Float32, Enu), Len(2)
      48834
    [2] DataId(0x1060, SampleTimeFine, Float32, Enu), Len(4)
      24455074
    [3] DataId(0x1070, SampleTimeCoarse, Float32, Enu), Len(4)
      2445
    [4] DataId(0x2034, EulerAngles, Float32, Ned), Len(12)
      Roll(-179.259), Pitch(-0.636), Yaw(-98.770)
    [5] DataId(0x4020, Acceleration, Float32, Enu), Len(12)
      X(-0.076), Y(0.141), Z(9.832)
    [6] DataId(0x5023, AltitudeEllipsoid, Float64, Enu), Len(8)
      603.645
    [7] DataId(0x5033, PositionEcef, Float64, Enu), Len(24)
      X(-1963779.710), Y(-3828825.125), Z(4692929.349)
    [8] DataId(0x5043, LatLon, Float64, Enu), Len(16)
      Lat(47.673), Lon(-117.153)
    [9] DataId(0x8020, RateOfTurn, Float32, Enu), Len(12)
      X(-0.006), Y(0.008), Z(0.005)
    [10] DataId(0xD010, VelocityXYZ, Float32, Enu), Len(12)
      X(-0.141), Y(-0.015), Z(0.032)
    [11] DataId(0xE020, StatusWord, Float32, Enu), Len(4)
      StatusWord(0x1800047: SelfTest(true), FilterValid(true), GNSSFix(true), ...)
```

## TODOs

check out other IMU crates for their patterns

* https://crates.io/crates/mpu9250
* https://crates.io/crates/bno055
* https://github.com/acburigo/ti154/blob/master/src/parser.rs
* https://github.com/cpsdqs/tinyframe-rs
  - https://github.com/cpsdqs/tinyframe-rs/blob/master/src/lib.rs

make a hal-friendly example with https://github.com/rust-embedded/linux-embedded-hal serial device

do all the `grep -R 'TODO' src/` things

diagram/table for the cable pinout
* mti cable red : Vin +5v
* mti cable black : GND
* mti cable yellow : RS232 Tx
* mti cable grey : RS232 Rx


DOCS BUG?:
page 13 on the proto spec in MT low level
IND ID 1 byte?
not in xsmessage.h nor is it in the viz?

protocol doc
https://www.xsens.com/hubfs/Downloads/Manuals/MT_Low-Level_Documentation.pdf

