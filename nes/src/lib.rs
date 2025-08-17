#[allow(unused_imports)]
pub mod prelude {
    pub use crate::mapper::Mapper;
    pub use crate::tools;
    pub use crate::tools::NESAccess;
    pub use bitflags::bitflags;
    pub use log::{debug, error, info, trace, warn};
    pub use std::cell::{Ref, RefCell, RefMut};
    pub use std::rc::Rc;
}
pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod input_device;
pub mod mapper;
pub mod memory;
pub mod ppu;
pub mod tools;
#[cfg(test)]
mod unit_tests;

use crate::apu::APU;
use crate::bus::Bus;
use crate::cartridge::ROM;
use crate::cpu::CPU;
use crate::input_device::{NESDevice, NESDeviceType};
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;

#[rustfmt::skip]
impl<'a> NESAccess<'a> for NES<'a> {
    fn bus(&self) -> Ref<Bus<'a>> { self.bus.borrow() }
    fn bus_mut(&self) -> RefMut<Bus<'a>> { self.bus.borrow_mut() }
    fn apu(&self) -> Ref<APU> { self.apu.borrow() }
    fn apu_mut(&self) -> RefMut<APU> { self.apu.borrow_mut() }
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
    fn rom(&self) -> Ref<ROM> { self.rom.borrow() }
    fn rom_mut(&self) -> RefMut<ROM> { self.rom.borrow_mut() }
    fn mapper(&self) -> Ref<Box<dyn Mapper>> { self.mapper.borrow() }
    fn mapper_mut(&self) -> RefMut<Box<dyn Mapper>> { self.mapper.borrow_mut() }
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
}

pub struct NES<'a> {
    pub memory: Rc<RefCell<Memory>>,
    pub mapper: Rc<RefCell<Box<dyn Mapper>>>,
    pub device1: Rc<RefCell<Box<dyn NESDevice>>>,
    pub cpu: CPU<'a>,
    pub bus: Rc<RefCell<Bus<'a>>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub apu: Rc<RefCell<APU>>,
    pub ppu: Rc<RefCell<PPU>>,
    pub rom: Rc<RefCell<ROM>>,
}

impl<'a> NES<'a> {
    pub fn new<'rcall, F>(rom: ROM, render_callback: F) -> Self
    where
        F: FnMut(Rc<RefCell<PPU>>, &mut Rc<RefCell<Box<dyn NESDevice>>>) + 'rcall + 'a,
    {
        let rom: Rc<RefCell<ROM>> = Rc::new(RefCell::new(rom));
        let memory: Rc<RefCell<Memory>> = Rc::new(RefCell::new(Memory::new(rom.borrow())));
        let device1: Rc<RefCell<Box<dyn NESDevice>>> = Rc::new(RefCell::new(
            input_device::new_device(NESDeviceType::Joypad),
        ));

        let apu: Rc<RefCell<APU>> = Rc::new(RefCell::new(APU::new()));
        let ppu: Rc<RefCell<PPU>> = Rc::new(RefCell::new({
            let rom: Rc<RefCell<ROM>> = rom.clone();
            PPU::new(memory.clone(), rom.borrow().screen_mirroring)
        }));
        let mapper: Rc<RefCell<Box<dyn Mapper>>> = Rc::new(RefCell::new(mapper::init_mapper(
            rom.borrow(),
            memory.clone(),
            device1.clone(),
        )));
        let bus: Rc<RefCell<Bus>> = Rc::new(RefCell::new(Bus::new(
            memory.clone(),
            mapper.clone(),
            apu.clone(),
            ppu.clone(),
            device1.clone(),
            render_callback,
        )));
        let cpu: CPU = CPU::new(bus.clone());

        mapper.borrow_mut().pass_ppu_ref(ppu.clone());

        NES {
            memory,
            mapper,
            device1,
            cpu,
            bus,
            apu,
            ppu,
            rom,
        }
    }

    pub fn reset(&mut self) {
        info!("Resetting CPU...");
        self.cpu.reset();
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum Region {
    NTSC,
    PAL,
}
