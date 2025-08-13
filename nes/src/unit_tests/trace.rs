use super::*;
use crate::bus::Bus;
use std::cell::RefMut;

#[test]
fn test_format_trace() {
    let mut nes: NES = setup_nes();

    let mut bus: RefMut<'_, Bus> = nes.bus_mut();
    // LDX #$01
    // DEX
    // DEY
    // BRK
    bus.write(0x0064, 0xA2);
    bus.write(0x0065, 0x01);
    bus.write(0x0066, 0xCA);
    bus.write(0x0067, 0x88);
    bus.write(0x0068, 0x00);
    drop(bus);

    nes.cpu.program_counter = 0x0064;
    nes.cpu.accumulator = 1;
    nes.cpu.index_x = 2;
    nes.cpu.index_y = 3;

    let mut result: Vec<String> = vec![];
    nes.cpu.run_with_callback(|cpu| {
        result.push(tools::trace(cpu));
    });

    assert_eq!(
        "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
        result[0]
    );
    assert_eq!(
        "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
        result[1]
    );
    assert_eq!(
        "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
        result[2]
    );
}

#[test]
fn test_format_mem_access() {
    let mut nes: NES = setup_nes();

    let mut bus: RefMut<'_, Bus> = nes.bus_mut();
    // ORA ($33), Y
    bus.write(0x0064, 0x11);
    bus.write(0x0065, 0x33);
    // Data
    bus.write(0x0033, 0x00);
    bus.write(0x0034, 0x04);
    bus.write(0x0400, 0xAA);
    drop(bus);

    nes.cpu.program_counter = 0x0064;
    nes.cpu.index_y = 0x00;

    let mut result: Vec<String> = vec![];
    nes.cpu.run_with_callback(|cpu| {
        result.push(tools::trace(cpu));
    });

    assert_eq!(
        "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
        result[0]
    );
}
