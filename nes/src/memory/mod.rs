pub mod mem_map;

use crate::cartridge::ROM;
use std::cell::Ref;

pub struct Memory {
    pub cpu_vram: [u8; 2048],
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_mem: Vec<u8>,
    pub use_chr_ram: bool,
}

impl Memory {
    pub fn new(rom: Ref<ROM>) -> Self {
        let mut prg_ram: Vec<u8> = Vec::with_capacity(rom.prg_ram_size);
        prg_ram.resize(rom.prg_ram_size, 0x00);

        let (chr_mem, use_chr_ram): (Vec<u8>, bool) = match rom.chr_rom.len() != 0 {
            true => (rom.chr_rom.clone(), false),
            false => {
                let mut chr_ram: Vec<u8> = Vec::with_capacity(rom.chr_ram_size);
                chr_ram.resize(rom.chr_ram_size, 0x00);
                (chr_ram, true)
            }
        };

        Memory {
            cpu_vram: [0x00; 2048],
            prg_rom: rom.prg_rom.clone(),
            prg_ram,
            chr_mem,
            use_chr_ram,
        }
    }
}
