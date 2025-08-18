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

pub type RcRef<T> = Rc<RefCell<T>>;
pub type BoxNESDevice = Box<dyn NESDevice>;
pub type BoxMapper = Box<dyn Mapper>;

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
    fn mapper(&self) -> Ref<BoxMapper> { self.mapper.borrow() }
    fn mapper_mut(&self) -> RefMut<BoxMapper> { self.mapper.borrow_mut() }
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
    fn device1(&self) -> Ref<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("NES tried to access \"Device 1\" before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow()
    }
    fn device1_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("NES tried to access \"Device 1\" before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow_mut()
    }
    fn device2(&self) -> Ref<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("NES tried to access \"Device 2\" before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow()
    }
    fn device2_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("NES tried to access \"Device 2\" before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow_mut()
    }
}

pub struct NES<'a> {
    pub memory: RcRef<Memory>,
    pub mapper: RcRef<BoxMapper>,
    pub device1: Option<RcRef<BoxNESDevice>>,
    pub device2: Option<RcRef<BoxNESDevice>>,
    pub cpu: CPU<'a>,
    pub bus: RcRef<Bus<'a>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub apu: RcRef<APU>,
    pub ppu: RcRef<PPU>,
    pub rom: RcRef<ROM>,
}

impl<'a> NES<'a> {
    pub fn new<'rcall, F>(rom: ROM, render_callback: F) -> Self
    where
        F: FnMut(RcRef<PPU>, &mut Option<RcRef<BoxNESDevice>>, &mut Option<RcRef<BoxNESDevice>>)
            + 'rcall + 'a,
    {
        let rom: RcRef<ROM> = Rc::new(RefCell::new(rom));
        let memory: RcRef<Memory> = Rc::new(RefCell::new(Memory::new(rom.borrow())));

        let apu: RcRef<APU> = Rc::new(RefCell::new(APU::new()));
        let ppu: RcRef<PPU> = Rc::new(RefCell::new({
            let rom: RcRef<ROM> = rom.clone();
            PPU::new(memory.clone(), rom.borrow().screen_mirroring)
        }));
        let mapper: RcRef<BoxMapper> = Rc::new(RefCell::new(mapper::init_mapper(
            rom.borrow(),
            memory.clone(),
        )));
        let bus: RcRef<Bus> = Rc::new(RefCell::new(Bus::new(
            memory.clone(),
            mapper.clone(),
            apu.clone(),
            ppu.clone(),
            render_callback,
        )));
        let cpu: CPU = CPU::new(bus.clone());

        mapper.borrow_mut().pass_ppu_ref(ppu.clone());

        NES {
            memory,
            mapper,
            device1: None,
            device2: None,
            cpu,
            bus,
            apu,
            ppu,
            rom,
        }
    }

    pub fn connect_input_device(&mut self, slot: u8, device: NESDeviceType) {
        assert!(slot >= 1 && slot <= 2);

        let device: RcRef<BoxNESDevice> = Rc::new(RefCell::new(input_device::new_device(device)));
        match slot {
            1 => self.device1 = Some(device),
            2 => self.device2 = Some(device),
            _ => panic!("This shouldn't happen!"),
        }

        self.bus_mut()
            .connect_input_device(slot, self.device1.clone().unwrap());
        self.mapper_mut()
            .connect_input_device(slot, self.device1.clone().unwrap());
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
