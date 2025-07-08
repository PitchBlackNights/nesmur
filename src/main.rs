use nes::NES;
use nes::cartridge::Rom;
use nesmur::cli_parser::Args;
use nesmur::prelude::*;
use nesmur::setup;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    let rom_bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom: Rom = Rom::new(&rom_bytes).unwrap();

    let mut nes: NES = NES::new(rom);
    nes.reset();
}
