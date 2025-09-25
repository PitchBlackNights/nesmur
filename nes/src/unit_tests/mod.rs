pub mod bus;
pub mod cpu;
pub mod joypad;
pub mod ppu;
pub mod trace;

use crate::{
    NES,
    cartridge::{
        CHR_ROM_PAGE_SIZE, Mirroring, PRG_RAM_PAGE_SIZE, PRG_ROM_PAGE_SIZE, ROM, ROMRegion,
    },
    memory::Memory,
    ppu::PPU,
    prelude::*,
};

fn test_rom(mut prg_rom: Vec<u8>) -> ROM {
    prg_rom.resize(2 * PRG_ROM_PAGE_SIZE, 0x00);
    prg_rom[0x7FFC] = 0x00;
    prg_rom[0x7FFD] = 0x80;

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
