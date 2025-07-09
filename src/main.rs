use nes::cartridge::Rom;
use nes::NES;
use nesmur::cli_parser::Args;
use nesmur::prelude::*;
use nesmur::setup;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");
    info!("Test {:?}", _args);

    let rom_bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom: Rom = Rom::new(&rom_bytes).unwrap();

    let mut nes: NES = NES::new(rom);
    nes.reset();
    nes.cpu.program_counter = 0x8000;
    for _ in 0..10 {
        nes.cpu.step();
    }
}
