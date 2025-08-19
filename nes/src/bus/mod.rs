use crate::apu::APU;
use crate::cpu::interrupt::{self, Interrupt};
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::ppu::renderer::Renderer;
use crate::prelude::*;
use crate::{BoxMapper, BoxNESDevice, RcRef};

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
    fn mapper(&self) -> Ref<BoxMapper> { self.mapper.borrow() }
    fn mapper_mut(&self) -> RefMut<BoxMapper> { self.mapper.borrow_mut() }
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
    fn device1(&self) -> Ref<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("Bus tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow()
    }
    fn device1_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("Bus tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow_mut()
    }
    fn device2(&self) -> Ref<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("Bus tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow()
    }
    fn device2_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("Bus tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow_mut()
    }
    fn renderer(&self) -> Ref<Renderer> { self.renderer.borrow() }
    fn renderer_mut(&self) -> RefMut<Renderer> { self.renderer.borrow_mut() }
}

pub struct Bus<'rcall> {
    pub cpu_cycles: usize,
    pub memory: RcRef<Memory>,
    pub mapper: RcRef<BoxMapper>,
    pub renderer: RcRef<Renderer>,
    pub ppu: RcRef<PPU>,
    pub device1: Option<RcRef<BoxNESDevice>>,
    pub device2: Option<RcRef<BoxNESDevice>>,
    #[allow(clippy::type_complexity)]
    render_callback: Box<
        dyn FnMut(
                RcRef<Renderer>,
                &mut Option<RcRef<BoxNESDevice>>,
                &mut Option<RcRef<BoxNESDevice>>,
            ) + 'rcall,
    >,
}

impl<'a> Bus<'a> {
    pub fn new<'rcall, F>(
        memory: RcRef<Memory>,
        mapper: RcRef<BoxMapper>,
        renderer: RcRef<Renderer>,
        _apu: RcRef<APU>,
        ppu: RcRef<PPU>,
        render_callback: F,
    ) -> Bus<'rcall>
    where
        F: FnMut(
                RcRef<Renderer>,
                &mut Option<RcRef<BoxNESDevice>>,
                &mut Option<RcRef<BoxNESDevice>>,
            ) + 'rcall,
    {
        Bus {
            cpu_cycles: 0,
            memory,
            mapper,
            renderer,
            ppu,
            device1: None,
            device2: None,
            render_callback: Box::from(render_callback),
        }
    }

    pub fn tick(&mut self, cpu_cycles: usize) {
        self.cpu_cycles += cpu_cycles;

        for _ in 0..cpu_cycles * 3 {
            let nmi_before: bool = self.ppu().nmi_interrupt.is_some();
            self.ppu_mut().tick(self.renderer_mut());
            let nmi_after: bool = self.ppu().nmi_interrupt.is_some();

            if !nmi_before && nmi_after {
                (self.render_callback)(
                    self.renderer.clone(),
                    &mut self.device1.clone(),
                    &mut self.device2.clone(),
                );
            }
        }
    }

    pub fn poll_interrupts(&mut self) -> Option<Interrupt> {
        if self.ppu_mut().poll_nmi_interrupt().is_some() {
            return Some(interrupt::NMI);
        }
        None
    }

    pub fn connect_input_device(&mut self, slot: u8, device: RcRef<BoxNESDevice>) {
        assert!((1..=2).contains(&slot));
        match slot {
            1 => self.device1 = Some(device),
            2 => self.device2 = Some(device),
            _ => panic!("This shouldn't happen!"),
        };
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
        let lo: u8 = (data & 0xFF) as u8;
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
