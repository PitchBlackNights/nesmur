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
use crate::ppu::renderer::Renderer;
use crate::prelude::*;

pub type RcRef<T> = Rc<RefCell<T>>;
pub type BoxNESDevice = Box<dyn NESDevice>;
pub type BoxMapper = Box<dyn Mapper>;

pub const DO_BUS_TRACE: bool = false;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

#[rustfmt::skip]
impl NESAccess for NES {
    fn bus(&self) -> Ref<Bus> { self.bus.borrow() }
    fn bus_mut(&self) -> RefMut<Bus> { self.bus.borrow_mut() }
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
            panic!("NES tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow()
    }
    fn device1_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("NES tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow_mut()
    }
    fn device2(&self) -> Ref<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("NES tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow()
    }
    fn device2_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("NES tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow_mut()
    }
    fn renderer(&self) -> Ref<Renderer> { self.renderer.borrow() }
    fn renderer_mut(&self) -> RefMut<Renderer> { self.renderer.borrow_mut() }
}

pub struct NES {
    pub memory: Rc<RefCell<Memory>>,
    pub mapper: Rc<RefCell<BoxMapper>>,
    pub renderer: Rc<RefCell<Renderer>>,

    pub device1: Option<Rc<RefCell<BoxNESDevice>>>,
    pub device2: Option<Rc<RefCell<BoxNESDevice>>>,

    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    /// ***CURRENTLY UNIMPLEMENTED***
    pub apu: Rc<RefCell<APU>>,
    pub ppu: Rc<RefCell<PPU>>,
    pub rom: Rc<RefCell<ROM>>,
}

impl NES {
    pub fn new(rom: ROM) -> Self {
        let rom: RcRef<ROM> = Rc::new(RefCell::new(rom));
        let memory: RcRef<Memory> = Rc::new(RefCell::new(Memory::new(rom.borrow())));
        let renderer: RcRef<Renderer> = Rc::new(RefCell::new(Renderer::new()));

        let apu: RcRef<APU> = Rc::new(RefCell::new(APU::new()));
        let ppu: RcRef<PPU> = Rc::new(RefCell::new({
            let rom: RcRef<ROM> = rom.clone();
            PPU::new(memory.clone(), rom.borrow().screen_mirroring)
        }));
        let mapper: RcRef<BoxMapper> = Rc::new(RefCell::new(mapper::init_mapper(
            rom.borrow(),
            memory.clone(),
            ppu.clone(),
        )));
        let bus: RcRef<Bus> = Rc::new(RefCell::new(Bus::new(
            memory.clone(),
            mapper.clone(),
            renderer.clone(),
            apu.clone(),
            ppu.clone(),
        )));
        let cpu: CPU = CPU::new(bus.clone());

        NES {
            memory,
            mapper,
            renderer,

            device1: None,
            device2: None,

            cpu,
            bus,
            apu,
            ppu,
            rom,
        }
    }

    pub fn reset(&mut self) {
        info!("Resetting NES...");
        self.cpu.reset();
        self.ppu_mut().reset();
        self.renderer_mut().reset();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback(&mut self, mut callback: impl FnMut(&mut CPU)) {
        loop {
            self.cpu.pre_step();
            callback(&mut self.cpu);
            if self.cpu.running {
                self.cpu.step();
            } else {
                break;
            }
        }
    }

    pub fn step(&mut self, mut callback: impl FnMut(&mut CPU)) -> bool {
        self.cpu.pre_step();
        callback(&mut self.cpu);
        if self.cpu.running {
            self.cpu.step();
            true
        } else {
            false
        }
    }

    pub fn render_callback<F>(&self, callback: F)
    where
        F: FnMut(
                RcRef<Renderer>,
                &mut Option<RcRef<BoxNESDevice>>,
                &mut Option<RcRef<BoxNESDevice>>,
            ) + 'static,
    {
        let renderer: RcRef<Renderer> = self.renderer.clone();
        let device1: Option<RcRef<BoxNESDevice>> = self.device1.clone();
        let device2: Option<RcRef<BoxNESDevice>> = self.device2.clone();
        let render_callback: RcRef<F> = Rc::new(RefCell::new(callback));

        self.bus_mut().render_callback(move || {
            let mut device1_clone: Option<RcRef<BoxNESDevice>> = device1.clone();
            let mut device2_clone: Option<RcRef<BoxNESDevice>> = device2.clone();
            (render_callback.clone().borrow_mut())(
                renderer.clone(),
                &mut device1_clone,
                &mut device2_clone,
            );
        });
    }

    pub fn connect_input_device(&mut self, port: u8, device_type: NESDeviceType) {
        assert!((1..=2).contains(&port));

        let device: RcRef<BoxNESDevice> =
            Rc::new(RefCell::new(input_device::new_device(device_type)));
        match port {
            1 => self.device1 = Some(device),
            2 => self.device2 = Some(device),
            _ => panic!("This shouldn't happen!"),
        }

        self.mapper_mut()
            .connect_input_device(port, self.device1.clone().unwrap());
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum Region {
    NTSC,
    PAL,
}
