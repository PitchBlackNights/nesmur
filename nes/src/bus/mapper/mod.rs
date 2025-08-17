mod nrom;
mod mmc1;

use crate::cartridge::Rom;
use crate::bus::Bus;

pub trait Mapper {
    fn new(rom: Rom) -> Self;
    fn read(&mut self, bus: &mut Bus, addr: u16) -> u8;
    fn write(&mut self, bus: &mut Bus, addr: u16, data: u8);
}

pub fn init_mapper(rom: Rom) -> Box<impl Mapper> {
    match rom.mapper {
        0 => Box::new(nrom::NROM::new(rom)),
        _ => panic!("Mapper {} ({}) is not supported!", rom.mapper, rom.submapper),
    }
}