mod mapper000;
mod mapper001;

use crate::cartridge::ROM;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;
use crate::{BoxMapper, BoxNESDevice, RcRef};

pub trait Mapper {
    fn connect_input_device(&mut self, slot: u8, device: RcRef<BoxNESDevice>);
    fn poll_interrupt(&self) -> bool {
        false
    }
    fn signal_scanline(&mut self) {}
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub fn init_mapper(rom: Ref<ROM>, memory: RcRef<Memory>, ppu: RcRef<PPU>) -> BoxMapper {
    match rom.mapper {
        0 => Box::new(mapper000::Mapper000::new(memory, ppu)),
        _ => panic!(
            "Mapper {} ({}) is not supported!",
            rom.mapper, rom.submapper
        ),
    }
}
