use super::Mapper;
use crate::cartridge::Rom;
use crate::bus::Bus;

pub struct NROM {
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Mapper for NROM {
    fn new(rom: Rom) -> Self {
        let mut prg_ram: Vec<u8> = Vec::with_capacity(rom.prg_ram_size);
        prg_ram.resize(rom.prg_ram_size, 0x00);
        
        NROM {
            prg_rom: rom.prg_rom,
            prg_ram,
            chr_rom: rom.chr_rom,
        }
    }
    
    fn read(&mut self, bus: &mut Bus, addr: u16) -> u8 {
        0
    }
    
    fn write(&mut self, bus: &mut Bus, addr: u16, data: u8) {
        
    }
}