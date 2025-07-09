#[allow(unused_imports)]
mod prelude {
    pub use crate::bus::Mem;
    pub use crate::tools::NESAccess;
    pub use log::{debug, error, info, trace, warn};
}
pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod ppu;
pub mod tools;
#[cfg(test)]
mod unit_tests;

use crate::apu::APU;
use crate::bus::Bus;
use crate::cartridge::Rom;
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[rustfmt::skip]
impl NESAccess for NES {
    fn bus(&self) -> Ref<Bus> { self.bus.borrow() }
    fn bus_mut(&self) -> RefMut<Bus> { self.bus.borrow_mut() }
    fn apu(&self) -> Ref<APU> { self.apu.borrow() }
    fn apu_mut(&self) -> RefMut<APU> { self.apu.borrow_mut() }
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
    fn rom(&self) -> Ref<Rom> { self.rom.borrow() }
    fn rom_mut(&self) -> RefMut<Rom> { self.rom.borrow_mut() }
}

pub struct NES {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub apu: Rc<RefCell<APU>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub ppu: Rc<RefCell<PPU>>,
    pub rom: Rc<RefCell<Rom>>,
}

impl NES {
    pub fn new(rom: Rom) -> Self {
        let apu: Rc<RefCell<APU>> = Rc::new(RefCell::new(APU::new()));
        let ppu: Rc<RefCell<PPU>> = Rc::new(RefCell::new(PPU::new()));
        let rom: Rc<RefCell<Rom>> = Rc::new(RefCell::new(rom));
        let bus: Rc<RefCell<Bus>> = Rc::new(RefCell::new(Bus::new(
            rom.clone(),
            apu.clone(),
            ppu.clone(),
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

    pub fn debug_cpu() {
        
    }

    pub fn reset(&mut self) {
        info!("Resetting CPU...");
        self.cpu.reset();
    }
}
