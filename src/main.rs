use nes::cartridge::Rom;
use nes::NES;
use nesmur::cli_parser::Args;
use nesmur::prelude::*;
use nesmur::setup;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    let rom_bytes: Vec<u8> = std::fs::read("nes/tests/roms/nestest.nes").unwrap();
    let rom: Rom = Rom::new(&rom_bytes).unwrap();
    let mut nes: NES = NES::new(rom);

    nes::bus::set_quiet_log(true);
    nes.cpu.reset();
    let mut instruction_cycle: u16 = 0;
    nes.cpu.run_with_callback(|cpu| {
        instruction_cycle += 1;
        if instruction_cycle == 8992 {
            cpu.running = false;
        }
    });

    info!("Stopping Emulator...");
}
