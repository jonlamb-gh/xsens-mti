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

## Protocol

See [MT_Low-Level_Documentation.pdf](https://www.xsens.com/hubfs/Downloads/Manuals/MT_Low-Level_Documentation.pdf).

## CA-MP2-MTi Cable Pinout

The part number of the Fischer connector is SS 102A059-130 Gunfire.

| MTi    | Description |
| :---   |        ---: |
| Red    |     4.5-34V |
| Black  |         GND |
| Yellow |    RS232 Tx |
| Grey   |    RS322 Rx |

## TODOs

do all the `grep -R 'TODO' src/` things
