mod mmc1;
mod nrom;

use crate::cartridge::ROM;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;
use crate::{BoxMapper, BoxNESDevice, RcRef};

pub trait Mapper {
    fn pass_ppu_ref(&mut self, ppu: RcRef<PPU>);
    fn connect_input_device(&mut self, slot: u8, device: RcRef<BoxNESDevice>);
    fn poll_interrupt(&self) -> bool {
        false
    }
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub fn init_mapper(rom: Ref<ROM>, memory: RcRef<Memory>) -> BoxMapper {
    match rom.mapper {
        0 => Box::new(nrom::NROM::new(memory)),
        _ => panic!(
            "Mapper {} ({}) is not supported!",
            rom.mapper, rom.submapper
        ),
    }
}
