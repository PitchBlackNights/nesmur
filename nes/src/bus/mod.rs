use crate::apu::APU;
use crate::cartridge::Rom;
use crate::ppu::PPU;
use crate::prelude::*;
use crate::joypad::Joypad;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | PPU Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|
const RAM: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;
const APU_REGISTERS: u16 = 0x4000;
const APU_REGISTERS_END: u16 = 0x4015;
const JOYPAD_1: u16 = 0x4016;
const JOYPAD_2: u16 = 0x4017;
const PRG_ROM: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub static mut QUIET_LOG: bool = false;
pub fn get_quiet_log() -> bool {
    unsafe { QUIET_LOG }
}
pub fn set_quiet_log(value: bool) {
    unsafe { QUIET_LOG = value }
}
pub static mut PREV_QUIET_LOG: bool = false;
pub fn get_prev_quiet_log() -> bool {
    unsafe { PREV_QUIET_LOG }
}
pub fn set_prev_quiet_log(value: bool) {
    unsafe { PREV_QUIET_LOG = value }
}

#[rustfmt::skip]
impl NESAccess<'_> for Bus<'_> {
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
}

pub struct Bus<'rcall> {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: Rc<RefCell<PPU>>,
    cycles: usize,
    render_callback: Box<dyn FnMut(Rc<RefCell<PPU>>, &mut Joypad) + 'rcall>,
    joypad1: Joypad,
}

impl<'a> Bus<'a> {
    pub fn new<'rcall, F>(
        rom: Rc<RefCell<Rom>>,
        _apu: Rc<RefCell<APU>>,
        ppu: Rc<RefCell<PPU>>,
        render_callback: F,
    ) -> Bus<'rcall>
    where
        F: FnMut(Rc<RefCell<PPU>>, &mut Joypad) + 'rcall,
    {
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.borrow().prg_rom.clone(),
            ppu,
            cycles: 0,
            render_callback: Box::from(render_callback),
            joypad1: Joypad::new(),
        }
    }

    pub fn tick(&mut self, cycles: usize) {
        self.cycles += cycles;
        let new_frame: bool = self.ppu_mut().tick(cycles * 3);
        if new_frame {
            (self.render_callback)(self.ppu.clone(), &mut self.joypad1);
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu_mut().poll_nmi_interrupt()
    }

    pub fn memory(&self) -> Vec<u8> {
        let mut memory: Vec<u8> = vec![0u8; 0x10000];
        let cpu_vram: &Vec<u8> = &self.cpu_vram.to_vec();
        let prg_rom: &Vec<u8> = &self.prg_rom;

        memory[0x0000..0x07FF + 1].copy_from_slice(cpu_vram);
        memory[0x0800..0x0FFF + 1].copy_from_slice(cpu_vram);
        memory[0x1000..0x17FF + 1].copy_from_slice(cpu_vram);
        memory[0x1800..0x1FFF + 1].copy_from_slice(cpu_vram);
        // OTHER MEMORY
        memory[0x8000..0xBFFF + 1].copy_from_slice(prg_rom);
        memory[0xC000..0xFFFF + 1].copy_from_slice(prg_rom);
        memory
    }

    #[rustfmt::skip]
    pub fn read(&mut self, addr: u16) -> u8 { self.__read(addr, false) }
    #[rustfmt::skip]
    pub fn write(&mut self, addr: u16, data: u8) { self.__write(addr, data, false); }
    #[rustfmt::skip]
    pub fn read_u16(&mut self, pos: u16) -> u16 { self.__read_u16(pos, false) }
    #[rustfmt::skip]
    pub fn write_u16(&mut self, pos: u16, data: u16) { self.__write_u16(pos, data, false); }
}

pub trait Mem {
    fn __read(&mut self, addr: u16, quiet: bool) -> u8;

    fn __write(&mut self, addr: u16, data: u8, quiet: bool);

    fn __read_u16(&mut self, pos: u16, quiet: bool) -> u16 {
        let lo: u16 = self.__read(pos, quiet) as u16;
        let hi: u16 = self.__read(pos.wrapping_add(1), quiet) as u16;
        (hi << 8) | lo
    }

    fn __write_u16(&mut self, pos: u16, data: u16, quiet: bool) {
        let hi: u8 = (data >> 8) as u8;
        let lo: u8 = (data & 0xff) as u8;
        self.__write(pos, lo, quiet);
        self.__write(pos.wrapping_add(1), hi, quiet);
    }
}

macro_rules! bus_trace {
    ($($arg:tt)+) => ({
        (!get_quiet_log()).then(|| {
            log::log!(log::Level::Trace, $($arg)+)
        });
    })
}

macro_rules! bus_logging {
    ($arg:ident) => {
        let quiet_log = get_quiet_log();
        set_prev_quiet_log(quiet_log);
        set_quiet_log($arg | quiet_log);
    };
    () => {
        set_quiet_log(get_prev_quiet_log());
    };
}

impl Mem for Bus<'_> {
    fn __read(&mut self, addr: u16, quiet: bool) -> u8 {
        bus_logging!(quiet);

        let data: u8 = match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                let byte: u8 = self.cpu_vram[mirror_down_addr as usize];
                bus_trace!(
                    "[RAM] Read {:#04X} from {:#06X} ({:#06X})",
                    byte,
                    addr,
                    mirror_down_addr
                );
                byte
            }

            PPU_REGISTERS | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!(
                    "Attempted to read from write-only PPU address: {:#06X}",
                    addr
                );
            }
            0x2002 => {
                let byte: u8 = self.ppu_mut().read_status();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU Status)", byte, addr);
                byte
            }
            0x2004 => {
                let byte: u8 = self.ppu().read_oam_data();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU OAM Data)", byte, addr);
                byte
            }
            0x2007 => {
                let byte: u8 = self.ppu_mut().read_data();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU Data)", byte, addr);
                byte
            }
            0x2008..=PPU_REGISTERS_END => {
                let mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                bus_trace!(
                    "[PPU] Mirroring down read at {:#06X} to {:#06X}",
                    addr,
                    mirror_down_addr
                );
                self.__read(mirror_down_addr, quiet)
            }

            APU_REGISTERS..=APU_REGISTERS_END => {
                // warn!("[APU] Ignoring bus read at {:#06X}", addr);
                0
            }

            JOYPAD_1 => {
                self.joypad1.read()
            }
            JOYPAD_2 => {
                // warn!("[JOY-2] Ignoring bus read at {:#06X}", addr);
                0
            }

            PRG_ROM..=PRG_ROM_END => {
                let mut mirror_down_addr: u16 = addr - 0x8000;
                if self.prg_rom.len() == 0x4000 && mirror_down_addr >= 0x4000 {
                    // Mirror the data if needed
                    mirror_down_addr %= 0x4000;
                }
                let byte: u8 = self.prg_rom[mirror_down_addr as usize];
                bus_trace!(
                    "[PRG-ROM] Read {:#04X} from {:#06X} ({:#06X})",
                    byte,
                    addr,
                    mirror_down_addr
                );
                byte
            }

            _ => {
                warn!("Ignoring bus read at {:#06X}", addr);
                0
            }
        };

        bus_logging!();
        data
    }

    fn __write(&mut self, addr: u16, data: u8, quiet: bool) {
        bus_logging!(quiet);

        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
                bus_trace!(
                    "[RAM] Wrote {:#04X} to {:#06X} ({:#06X})",
                    data,
                    addr,
                    mirror_down_addr
                );
            }

            PPU_REGISTERS => {
                self.ppu_mut().write_to_ctrl(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU Control Register)",
                    data,
                    addr
                );
            }
            0x2001 => {
                self.ppu_mut().write_to_mask(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU Mask Register)",
                    data,
                    addr
                );
            }
            0x2002 => panic!("Attempted to write to PPU status register"),
            0x2003 => {
                self.ppu_mut().write_to_oam_addr(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU OAM Address)",
                    data,
                    addr
                );
            }
            0x2004 => {
                self.ppu_mut().write_to_oam_data(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU OAM Data)", data, addr);
            }
            0x2005 => {
                self.ppu_mut().write_to_scroll(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU Scroll Register)",
                    data,
                    addr
                );
            }
            0x2006 => {
                self.ppu_mut().write_to_ppu_addr(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU Address)", data, addr);
            }
            0x2007 => {
                self.ppu_mut().write_to_data(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU Data)", data, addr);
            }
            0x2008..=PPU_REGISTERS_END => {
                let mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                bus_trace!(
                    "[PPU] Mirroring down write at {:#06X} to {:#06X}",
                    addr,
                    mirror_down_addr
                );
                self.__write(mirror_down_addr, data, quiet);
            }

            APU_REGISTERS..=0x4013 | APU_REGISTERS_END => {
                // warn!("[APU] Ignoring bus write at {:#06X}", addr);
            }

            // https://wiki.nesdev.com/w/index.php/PPU_programmer_reference#OAM_DMA_.28.244014.29_.3E_write
            0x4014 => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.__read(hi + i, quiet);
                }

                self.ppu_mut().write_oam_dma(&buffer);

                // TODO: Handle this eventually
                // let add_cycles: u16 = if self.cycles % 2 == 1 { 514 } else { 513 };
                // self.tick(add_cycles); // TODO: This will cause weird effects as PPU will have 513/514 * 3 ticks
            }

            JOYPAD_1 => {
                self.joypad1.write(data);
            }
            JOYPAD_2 => {
                // warn!("[JOY-2] Ignoring bus write at {:#06X}", addr);
            }

            PRG_ROM..=PRG_ROM_END => panic!("Attempted to write to PRG-ROM: {:#06X}", addr),

            _ => warn!("Ignoring bus write at {:#06X}", addr),
        }

        bus_logging!();
    }
}
