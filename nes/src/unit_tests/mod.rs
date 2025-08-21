pub mod bus;
pub mod cpu;
pub mod joypad;
pub mod ppu;
pub mod trace;

use crate::NES;
use crate::cartridge::{
    CHR_ROM_PAGE_SIZE, Mirroring, PRG_RAM_PAGE_SIZE, PRG_ROM_PAGE_SIZE, ROM, ROMRegion,
};
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;

// struct TestRom {
//     header: Vec<u8>,
//     trainer: Option<Vec<u8>>,
//     prg_rom: Vec<u8>,
//     chr_rom: Vec<u8>,
// }

// fn create_rom(rom: TestRom) -> Vec<u8> {
//     let mut result: Vec<u8> = Vec::with_capacity(
//         rom.header.len()
//             + rom.trainer.as_ref().map_or(0, |t| t.len())
//             + rom.prg_rom.len()
//             + rom.chr_rom.len(),
//     );

//     result.extend(&rom.header);
//     if let Some(t) = rom.trainer {
//         result.extend(t);
//     }
//     result.extend(&rom.prg_rom);
//     result.extend(&rom.chr_rom);

//     result
// }

fn test_rom(mut prg_rom: Vec<u8>) -> ROM {
    prg_rom.resize(2 * PRG_ROM_PAGE_SIZE, 0x00);
    prg_rom[0x7FFC] = 0x00;
    prg_rom[0x7FFD] = 0x80;

    // let test_rom: Vec<u8> = create_rom(TestRom {
    //     header: vec![
    //         0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00,
    //     ],
    //     trainer: None,
    //     pgp_rom: prg_rom,
    //     chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    // });

    // ROM::new(&test_rom).unwrap()

    ROM {
        ines_ver: 1,
        mapper: 0,
        submapper: 0,
        region: ROMRegion::NTSC,
        prg_rom,
        prg_ram_size: 0,
        chr_rom: vec![0x00; 1 * CHR_ROM_PAGE_SIZE],
        chr_ram_size: 0,
        screen_mirroring: Mirroring::Horizontal,
        uses_bat_mem: false,
    }
}

fn setup_nes_with_rom(data: Vec<u8>) -> NES {
    let rom: ROM = test_rom(data);
    NES::new(rom)
}

fn setup_nes() -> NES {
    setup_nes_with_rom(vec![])
}

fn empty_ppu(mirroring: Mirroring) -> PPU {
    let memory: Memory = Memory {
        cpu_vram: [0x00; 2048],
        prg_rom: vec![],
        prg_ram: vec![],
        chr_mem: vec![0x00; CHR_ROM_PAGE_SIZE],
        use_chr_ram: false,
    };

    let mut ppu: PPU = PPU::new(Rc::new(RefCell::new(memory)), mirroring);
    ppu.cycles = 9886;

    ppu
}
