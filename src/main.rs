use nes::bus::Mem;
use nes::cpu::CPU;
use nesmur::cli_parser::Args;
use nesmur::prelude::*;
use nesmur::setup;
use nesmur::test_timing::test_region_timings;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    test_region_timings();
    sleep(Duration::from_secs(100));

    let mut cpu: CPU = CPU::new();

    cpu.mem_write(0x0000, 0xEA);
    cpu.mem_write(0x0001, 0xEB);
    cpu.mem_write(0x0003, 0xB1);

    cpu.step();
    cpu.step();
    cpu.step();
}
