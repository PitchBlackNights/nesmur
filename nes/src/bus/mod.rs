use crate::apu::APU;
use crate::cpu::interrupt::{self, Interrupt};
use crate::input_device::NESDevice;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::prelude::*;

pub static mut QUIET_LOG: bool = false;
pub static mut PREV_QUIET_LOG: bool = false;
pub fn get_quiet_log() -> bool {
    unsafe { QUIET_LOG }
}
pub fn set_quiet_log(value: bool) {
    unsafe { QUIET_LOG = value }
}

#[macro_export]
macro_rules! bus_trace {
    ($($arg:tt)+) => ({
        (!$crate::bus::get_quiet_log()).then(|| {
            log::log!(log::Level::Trace, $($arg)+)
        });
    })
}

#[rustfmt::skip]
impl NESAccess<'_> for Bus<'_> {
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
    fn mapper(&self) -> Ref<Box<dyn Mapper>> { self.mapper.borrow() }
    fn mapper_mut(&self) -> RefMut<Box<dyn Mapper>> { self.mapper.borrow_mut() }
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
    fn device1(&self) -> Ref<Box<dyn NESDevice>> { self.device1.borrow() }
    fn device1_mut(&self) -> RefMut<Box<dyn NESDevice>> { self.device1.borrow_mut() }
}

pub struct Bus<'rcall> {
    pub cpu_cycles: usize,
    pub memory: Rc<RefCell<Memory>>,
    pub mapper: Rc<RefCell<Box<dyn Mapper>>>,
    pub ppu: Rc<RefCell<PPU>>,
    pub device1: Rc<RefCell<Box<dyn NESDevice>>>,
    #[allow(clippy::type_complexity)]
    render_callback:
        Box<dyn FnMut(Rc<RefCell<PPU>>, &mut Rc<RefCell<Box<dyn NESDevice>>>) + 'rcall>,
}

impl<'a> Bus<'a> {
    pub fn new<'rcall, F>(
        memory: Rc<RefCell<Memory>>,
        mapper: Rc<RefCell<Box<dyn Mapper>>>,
        _apu: Rc<RefCell<APU>>,
        ppu: Rc<RefCell<PPU>>,
        device1: Rc<RefCell<Box<dyn NESDevice>>>,
        render_callback: F,
    ) -> Bus<'rcall>
    where
        F: FnMut(Rc<RefCell<PPU>>, &mut Rc<RefCell<Box<dyn NESDevice>>>) + 'rcall,
    {
        Bus {
            cpu_cycles: 0,
            memory,
            mapper,
            ppu,
            device1,
            render_callback: Box::from(render_callback),
        }
    }

    pub fn tick(&mut self, cpu_cycles: usize) {
        self.cpu_cycles += cpu_cycles;

        let nmi_before: bool = self.ppu().nmi_interrupt.is_some();
        self.ppu_mut().tick(cpu_cycles * 3);
        let nmi_after: bool = self.ppu().nmi_interrupt.is_some();

        if !nmi_before && nmi_after {
            (self.render_callback)(self.ppu.clone(), &mut self.device1);
        }
    }

    pub fn poll_interrupts(&mut self) -> Option<Interrupt> {
        if self.ppu_mut().poll_nmi_interrupt().is_some() {
            return Some(interrupt::NMI);
        }
        None
    }

    // pub fn memory(&self) -> Vec<u8> {
    //     let mut memory: Vec<u8> = vec![0u8; 0x10000];
    //     let cpu_vram: &Vec<u8> = &self.cpu_vram.to_vec();
    //     let prg_rom: &Vec<u8> = &self.prg_rom;

    //     memory[0x0000..0x07FF + 1].copy_from_slice(cpu_vram);
    //     memory[0x0800..0x0FFF + 1].copy_from_slice(cpu_vram);
    //     memory[0x1000..0x17FF + 1].copy_from_slice(cpu_vram);
    //     memory[0x1800..0x1FFF + 1].copy_from_slice(cpu_vram);
    //     // OTHER MEMORY
    //     memory[0x8000..0xBFFF + 1].copy_from_slice(prg_rom);
    //     memory[0xC000..0xFFFF + 1].copy_from_slice(prg_rom);
    //     memory
    // }

    pub fn read_u16(&mut self, pos: u16) -> u16 {
        let lo: u16 = self.read(pos) as u16;
        let hi: u16 = self.read(pos.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_u16(&mut self, pos: u16, data: u16) {
        let hi: u8 = (data >> 8) as u8;
        let lo: u8 = (data & 0xff) as u8;
        self.write(pos, lo);
        self.write(pos.wrapping_add(1), hi);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.mapper_mut().read(addr)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mapper_mut().write(addr, data);
    }
}
