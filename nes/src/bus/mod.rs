use crate::apu::APU;
use crate::cartridge::Rom;
use crate::ppu::PPU;
use crate::prelude::*;
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
const PRG_ROM: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub static mut QUIET_LOG: bool = false;
pub fn get_quiet_log() -> bool {
    unsafe { QUIET_LOG }
}
pub fn set_quiet_log(value: bool) {
    unsafe { QUIET_LOG = value }
}

#[rustfmt::skip]
impl NESAccess for Bus {
    fn rom(&self) -> Ref<Rom> { self.rom.borrow() }
    fn rom_mut(&self) -> RefMut<Rom> { self.rom.borrow_mut() }
}

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rc<RefCell<Rom>>,
}

impl Bus {
    pub fn new(rom: Rc<RefCell<Rom>>, _apu: Rc<RefCell<APU>>, _ppu: Rc<RefCell<PPU>>) -> Bus {
        Bus {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    pub fn memory(&self) -> Vec<u8> {
        let mut memory: Vec<u8> = vec![0u8; 0x10000];
        let cpu_vram: Vec<u8> = self.cpu_vram.to_vec();
        let prg_rom: &Vec<u8> = &self.rom().prg_rom;

        memory[0x0000..0x07FF+1].copy_from_slice(&cpu_vram);
        memory[0x0800..0x0FFF+1].copy_from_slice(&cpu_vram);
        memory[0x1000..0x17FF+1].copy_from_slice(&cpu_vram);
        memory[0x1800..0x1FFF+1].copy_from_slice(&cpu_vram);
        // OTHER MEMORY
        memory[0x8000..0xBFFF+1].copy_from_slice(&prg_rom);
        memory[0xC000..0xFFFF+1].copy_from_slice(&prg_rom);
        memory
    }

    #[rustfmt::skip]
    pub fn read(&self, addr: u16) -> u8 { self.__read(addr, false) }
    #[rustfmt::skip]
    pub fn write(&mut self, addr: u16, data: u8) { self.__write(addr, data, false); }
    #[rustfmt::skip]
    pub fn read_u16(&self, pos: u16) -> u16 { self.__read_u16(pos, false) }
    #[rustfmt::skip]
    pub fn write_u16(&mut self, pos: u16, data: u16) { self.__write_u16(pos, data, false); }
}

pub trait Mem {
    fn __read(&self, addr: u16, quiet: bool) -> u8;

    fn __write(&mut self, addr: u16, data: u8, quiet: bool);

    fn __read_u16(&self, pos: u16, quiet: bool) -> u16 {
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

impl Mem for Bus {
    fn __read(&self, addr: u16, quiet: bool) -> u8 {
        // Force on for now
        let quiet: bool = quiet || get_quiet_log() || true;
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                let byte: u8 = self.cpu_vram[mirror_down_addr as usize];
                (!quiet).then(|| trace!("[RAM] Read {:#04X} from {:#06X} ({:#06X})", byte, addr, mirror_down_addr));
                byte
            }
            PPU_REGISTERS..=PPU_REGISTERS_END => {
                let _addr_mirror_down: u16 = addr & 0b0010_0000_0000_0111;
                //todo!("PPU is not supported yet")
                0
            }
            PRG_ROM..=PRG_ROM_END => {
                let mut mirror_down_addr: u16 = addr - 0x8000;
                if self.rom().prg_rom.len() == 0x4000 && mirror_down_addr >= 0x4000 {
                    // Mirror the data if needed
                    mirror_down_addr %= 0x4000;
                }
                let byte: u8 = self.rom().prg_rom[mirror_down_addr as usize];
                (!quiet).then(|| trace!("[PRG-ROM] Read {:#04X} from {:#06X} ({:#06X})", byte, addr, mirror_down_addr));
                byte
            }

            _ => {
                warn!("Ignoring bus read at {:#06X}", addr);
                0
            }
        }
    }

    fn __write(&mut self, addr: u16, data: u8, quiet: bool) {
        // Force on for now
        let quiet: bool = quiet || get_quiet_log() || true;
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
                (!quiet).then(|| trace!("[RAM] Wrote {:#04X} to {:#06X} ({:#06X})", data, addr, mirror_down_addr));
            }
            PPU_REGISTERS..=PPU_REGISTERS_END => {
                let _mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                //todo!("PPU is not supported yet");
            }
            PRG_ROM..=PRG_ROM_END => panic!("Attempted to write to PRG-ROM: {:#06X}", addr),

            _ => warn!("Ignoring bus write at {:#06X}", addr),
        }
    }
}
