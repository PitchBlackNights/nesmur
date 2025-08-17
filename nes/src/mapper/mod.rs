mod mmc1;
mod nrom;

use crate::cartridge::ROM;
use crate::input_device::NESDevice;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;

pub trait Mapper {
    fn pass_ppu_ref(&mut self, ppu: Rc<RefCell<PPU>>);
    fn poll_interrupt(&self) -> bool {
        false
    }
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub fn init_mapper(
    rom: Ref<ROM>,
    memory: Rc<RefCell<Memory>>,
    device1: Rc<RefCell<Box<dyn NESDevice>>>,
) -> Box<dyn Mapper> {
    match rom.mapper {
        0 => Box::new(nrom::NROM::new(memory, device1)),
        _ => panic!(
            "Mapper {} ({}) is not supported!",
            rom.mapper, rom.submapper
        ),
    }
}
