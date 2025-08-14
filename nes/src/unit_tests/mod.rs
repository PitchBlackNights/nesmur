pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod ppu;
pub mod trace;

use crate::NES;
use crate::cartridge::{CHR_ROM_PAGE_SIZE, PRG_ROM_PAGE_SIZE, Rom};
use crate::prelude::*;

struct TestRom {
    header: Vec<u8>,
    trainer: Option<Vec<u8>>,
    pgp_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, |t| t.len())
            + rom.pgp_rom.len()
            + rom.chr_rom.len(),
    );

    result.extend(&rom.header);
    if let Some(t) = rom.trainer {
        result.extend(t);
    }
    result.extend(&rom.pgp_rom);
    result.extend(&rom.chr_rom);

    result
}

fn test_rom(program: Vec<u8>) -> Rom {
    let mut pgp_rom_contents: Vec<u8> = program;
    pgp_rom_contents.resize(2 * PRG_ROM_PAGE_SIZE, 0);

    let test_rom: Vec<u8> = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        trainer: None,
        pgp_rom: pgp_rom_contents,
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });

    Rom::new(&test_rom).unwrap()
}

fn setup_nes_with_rom<'a>(data: Vec<u8>) -> NES<'a> {
    let rom: Rom = test_rom(data);
    NES::new(rom, |_| {})
}

fn setup_nes<'a>() -> NES<'a> {
    setup_nes_with_rom(vec![])
}
