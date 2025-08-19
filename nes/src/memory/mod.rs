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
        let prg_ram: Vec<u8> = vec![0x00; rom.prg_ram_size];
        let (chr_mem, use_chr_ram): (Vec<u8>, bool) = match !rom.chr_rom.is_empty() {
            true => (rom.chr_rom.clone(), false),
            false => (vec![0x00; rom.chr_ram_size], true),
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
