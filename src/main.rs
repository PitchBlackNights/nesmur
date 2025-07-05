use nes::bus::Mem;
use nesmur::cli_parser::Args;
use nes::cpu::CPU;
use nesmur::prelude::*;
use nesmur::setup;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    let mut cpu: CPU = CPU::new();

    cpu.mem_write(0x0000, 0xEA);
    cpu.mem_write(0x0001, 0xEB);
    cpu.mem_write(0x0003, 0xB1);

    cpu.step();
    cpu.step();
    cpu.step();
}
