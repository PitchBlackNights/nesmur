use crate::{
    BoxMapper, RcRef,
    apu::APU,
    cpu::interrupt::{self, Interrupt},
    memory::Memory,
    ppu::{PPU, renderer::Renderer},
    prelude::*,
};

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
        (!$crate::bus::get_quiet_log() && $crate::DO_BUS_TRACE).then(|| {
            log::log!(log::Level::Trace, $($arg)+)
        });
    })
}

#[rustfmt::skip]
impl NESAccess for Bus {
    fn ppu(&self) -> Ref<'_, PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<'_, PPU> { self.ppu.borrow_mut() }
    fn mapper(&self) -> Ref<'_, BoxMapper> { self.mapper.borrow() }
    fn mapper_mut(&self) -> RefMut<'_, BoxMapper> { self.mapper.borrow_mut() }
    fn memory(&self) -> Ref<'_, Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<'_, Memory> { self.memory.borrow_mut() }
    fn renderer(&self) -> Ref<'_, Renderer> { self.renderer.borrow() }
    fn renderer_mut(&self) -> RefMut<'_, Renderer> { self.renderer.borrow_mut() }
}

pub struct Bus {
    pub cpu_cycles: usize,
    pub memory: RcRef<Memory>,
    pub mapper: RcRef<BoxMapper>,
    pub renderer: RcRef<Renderer>,
    pub ppu: RcRef<PPU>,
    #[allow(clippy::type_complexity)]
    render_callback: Box<dyn FnMut()>,
}

impl Bus {
    pub fn new(
        memory: RcRef<Memory>,
        mapper: RcRef<BoxMapper>,
        renderer: RcRef<Renderer>,
        _apu: RcRef<APU>,
        ppu: RcRef<PPU>,
    ) -> Bus {
        Bus {
            cpu_cycles: 0,
            memory,
            mapper,
            renderer,
            ppu,
            render_callback: Box::from(|| {}),
        }
    }

    pub fn render_callback<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.render_callback = Box::new(callback);
    }

    pub fn tick(&mut self, cpu_cycles: usize) {
        self.cpu_cycles += cpu_cycles;

        for _ in 0..cpu_cycles * 3 {
            let nmi_before: bool = self.ppu().nmi_interrupt.is_some();
            self.ppu_mut().tick(self.renderer_mut());
            let nmi_after: bool = self.ppu().nmi_interrupt.is_some();

            if !nmi_before && nmi_after {
                (self.render_callback)();
            }
        }
    }

    pub fn poll_interrupts(&mut self) -> Option<Interrupt> {
        if self.ppu_mut().poll_nmi_interrupt().is_some() {
            return Some(interrupt::NMI);
        }
        None
    }

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
