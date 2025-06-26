use crate::prelude::*;

pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_vram: [0; 2048],
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo: u16 = self.mem_read(pos) as u16;
        let hi: u16 = self.mem_read(pos + 1) as u16;
        (hi << 8) | lo
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi: u8 = (data >> 8) as u8;
        let lo: u8 = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                let byte: u8 = self.cpu_vram[mirror_down_addr as usize];
                trace!("[MEM] Read {:#02X} from {:#04X}", byte, mirror_down_addr);
                byte
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                todo!("PPU is not supported yet")
            }
            _ => {
                warn!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
                trace!("[MEM] Wrote {:#02X} to {:#04X}", data, mirror_down_addr);
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                todo!("PPU is not supported yet");
            }
            _ => {
                warn!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
