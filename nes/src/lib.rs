#[allow(unused_imports)]
mod prelude {
    pub use crate::bus::Mem;
    pub use crate::ppu::NesPPU;
    pub use crate::tools;
    pub use crate::tools::NESAccess;
    pub use bitflags::bitflags;
    pub use log::{debug, error, info, trace, warn};
}
pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod ppu;
pub mod tools;
pub mod joypad;
#[cfg(test)]
mod unit_tests;

use crate::apu::APU;
use crate::bus::Bus;
use crate::cartridge::Rom;
use crate::cpu::CPU;
use crate::joypad::Joypad;
use crate::ppu::PPU;
use crate::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[rustfmt::skip]
impl<'a> NESAccess<'a> for NES<'a> {
    fn bus(&self) -> Ref<'_, Bus<'a>> { self.bus.borrow() }
    fn bus_mut(&self) -> RefMut<'_, Bus<'a>> { self.bus.borrow_mut() }
    fn apu(&self) -> Ref<APU> { self.apu.borrow() }
    fn apu_mut(&self) -> RefMut<APU> { self.apu.borrow_mut() }
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
    fn rom(&self) -> Ref<Rom> { self.rom.borrow() }
    fn rom_mut(&self) -> RefMut<Rom> { self.rom.borrow_mut() }
}

pub struct NES<'a> {
    pub cpu: CPU<'a>,
    pub bus: Rc<RefCell<Bus<'a>>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub apu: Rc<RefCell<APU>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub ppu: Rc<RefCell<PPU>>,
    pub rom: Rc<RefCell<Rom>>,
}

impl<'a> NES<'a> {
    pub fn new<'rcall, F>(rom: Rom, render_callback: F) -> Self
    where
        F: FnMut(Rc<RefCell<PPU>>, &mut Joypad) + 'rcall + 'a,
    {
        let rom: Rc<RefCell<Rom>> = Rc::new(RefCell::new(rom));
        let apu: Rc<RefCell<APU>> = Rc::new(RefCell::new(APU::new()));
        let ppu: Rc<RefCell<PPU>> = Rc::new(RefCell::new({
            let rom: Rc<RefCell<Rom>> = rom.clone();
            PPU::new(rom.borrow().chr_rom.clone(), rom.borrow().screen_mirroring)
        }));
        let bus: Rc<RefCell<Bus>> = Rc::new(RefCell::new(Bus::new(
            rom.clone(),
            apu.clone(),
            ppu.clone(),
            render_callback,
        )));
        let cpu: CPU = CPU::new(bus.clone());

        NES {
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
