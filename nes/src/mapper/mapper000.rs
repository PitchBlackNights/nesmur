use super::Mapper;
use crate::bus_trace;
use crate::memory::Memory;
use crate::memory::mem_map::*;
use crate::ppu::PPU;
use crate::prelude::*;
use crate::{BoxNESDevice, RcRef};

#[rustfmt::skip]
impl NESAccess<'_> for Mapper000 {
    fn ppu(&self) -> Ref<PPU> { self.ppu.borrow() }
    fn ppu_mut(&self) -> RefMut<PPU> { self.ppu.borrow_mut() }
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
    fn device1(&self) -> Ref<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("Mapper tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow()
    }
    fn device1_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device1.is_none() {
            panic!("Mapper tried to access `Device 1` before a reference was passed to it!");
        }
        self.device1.as_ref().unwrap().borrow_mut()
    }
    fn device2(&self) -> Ref<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("Mapper tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow()
    }
    fn device2_mut(&self) -> RefMut<BoxNESDevice> {
        if self.device2.is_none() {
            panic!("Mapper tried to access `Device 2` before a reference was passed to it!");
        }
        self.device2.as_ref().unwrap().borrow_mut()
    }
}

pub struct Mapper000 {
    ppu: RcRef<PPU>,
    memory: RcRef<Memory>,
    device1: Option<RcRef<BoxNESDevice>>,
    device2: Option<RcRef<BoxNESDevice>>,
}

impl Mapper000 {
    pub fn new(memory: RcRef<Memory>, ppu: RcRef<PPU>) -> Self {
        Mapper000 {
            ppu,
            memory,
            device1: None,
            device2: None,
        }
    }
}

impl Mapper for Mapper000 {
    fn connect_input_device(&mut self, slot: u8, device: RcRef<BoxNESDevice>) {
        assert!((1..=2).contains(&slot));
        match slot {
            1 => self.device1 = Some(device),
            2 => self.device2 = Some(device),
            _ => panic!("This shouldn't happen!"),
        };
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                let byte: u8 = self.memory().cpu_vram[mirror_down_addr as usize];
                bus_trace!(
                    "[RAM] Read {:#04X} from {:#06X} ({:#06X})",
                    byte,
                    addr,
                    mirror_down_addr
                );
                byte
            }

            MMIO_PPUCTRL | MMIO_PPUMASK | MMIO_OAMADDR | MMIO_PPUSCROLL | MMIO_PPUADDR
            | MMIO_OAMDMA => {
                error!(
                    "Attempted to read from write-only PPU address {:#06X}",
                    addr
                );
                0
            }
            MMIO_PPUSTATUS => {
                let byte: u8 = self.ppu_mut().read_status();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU Status)", byte, addr);
                byte
            }
            MMIO_OAMDATA => {
                let byte: u8 = self.ppu().read_oam_data();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU OAM Data)", byte, addr);
                byte
            }
            MMIO_PPUDATA => {
                let byte: u8 = self.ppu_mut().read_data();
                bus_trace!("[PPU] Read {:#04X} from {:#06X} (PPU Data)", byte, addr);
                byte
            }
            PPU_REGISTERS_MIRROR..=PPU_REGISTERS_END => {
                let mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                bus_trace!(
                    "[PPU] Mirroring down read at {:#06X} to {:#06X}",
                    addr,
                    mirror_down_addr
                );
                self.read(mirror_down_addr)
            }

            APU_REGISTERS..=APU_REGISTERS_END => {
                // warn!("[APU] Ignoring bus read at {:#06X}", addr);
                0
            }

            MMIO_JOY1 => {
                if self.device1.is_some() {
                    self.device1_mut().read()
                } else {
                    0x00
                }
            }
            MMIO_JOY2 => {
                if self.device2.is_some() {
                    self.device2_mut().read()
                } else {
                    0x00
                }
            }

            PRG_ROM..=PRG_ROM_END => {
                let mut mirror_down_addr: u16 = addr - 0x8000;
                if self.memory().prg_rom.len() == 0x4000 && mirror_down_addr >= 0x4000 {
                    // Mirror the data if needed
                    mirror_down_addr %= 0x4000;
                }
                let byte: u8 = self.memory().prg_rom[mirror_down_addr as usize];
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
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_END => {
                let mirror_down_addr: u16 = addr & 0b0000_0111_1111_1111;
                self.memory_mut().cpu_vram[mirror_down_addr as usize] = data;
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
            MMIO_PPUMASK => {
                self.ppu_mut().write_to_mask(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU Mask Register)",
                    data,
                    addr
                );
            }
            MMIO_PPUSTATUS => error!("Attempted to write {:#04X} to PPU status register", data),
            MMIO_OAMADDR => {
                self.ppu_mut().write_to_oam_addr(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU OAM Address)",
                    data,
                    addr
                );
            }
            MMIO_OAMDATA => {
                self.ppu_mut().write_to_oam_data(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU OAM Data)", data, addr);
            }
            MMIO_PPUSCROLL => {
                self.ppu_mut().write_to_scroll(data);
                bus_trace!(
                    "[PPU] Wrote {:#04X} to {:#06X} (PPU Scroll Register)",
                    data,
                    addr
                );
            }
            MMIO_PPUADDR => {
                self.ppu_mut().write_to_ppu_addr(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU Address)", data, addr);
            }
            MMIO_PPUDATA => {
                self.ppu_mut().write_to_data(data);
                bus_trace!("[PPU] Wrote {:#04X} to {:#06X} (PPU Data)", data, addr);
            }
            PPU_REGISTERS_MIRROR..=PPU_REGISTERS_END => {
                let mirror_down_addr: u16 = addr & 0b0010_0000_0000_0111;
                bus_trace!(
                    "[PPU] Mirroring down write at {:#06X} to {:#06X}",
                    addr,
                    mirror_down_addr
                );
                self.write(mirror_down_addr, data);
            }

            APU_REGISTERS..=MMIO_DMC_LEN | APU_REGISTERS_END => {
                // warn!("[APU] Ignoring bus write at {:#06X}", addr);
            }

            // https://wiki.nesdev.com/w/index.php/PPU_programmer_reference#OAM_DMA_.28.244014.29_.3E_write
            MMIO_OAMDMA => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.read(hi + i);
                }

                self.ppu_mut().write_oam_dma(&buffer);

                // TODO: Handle this eventually
                // let add_cycles: u16 = if bus.cpu_cycles % 2 == 1 { 514 } else { 513 };
                // bus.tick(add_cycles); // TODO: This will cause weird effects as PPU will have 513/514 * 3 ticks
            }

            MMIO_JOY1 => {
                if self.device1.is_some() {
                    self.device1_mut().write(data);
                }
                if self.device2.is_some() {
                    self.device2_mut().write(data);
                }
            }
            MMIO_JOY2 => {
                // warn!("[JOY-2] Ignoring bus write {:#04X} at {:#06X}", data, addr);
            }

            PRG_ROM..=PRG_ROM_END => {
                error!("Attempted to write {:#04X} to PRG-ROM {:#06X}", data, addr)
            }

            _ => warn!("Ignoring bus write at {:#06X}", addr),
        }
    }
}
