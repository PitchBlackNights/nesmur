pub mod registers;

use crate::RcRef;
use crate::cartridge::Mirroring;
use crate::memory::Memory;
use crate::prelude::*;
use registers::addr::AddrRegister;
use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;

#[rustfmt::skip]
impl NESAccess<'_> for PPU {
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
}

pub struct PPU {
    memory: RcRef<Memory>,
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub scroll: ScrollRegister,
    pub addr: AddrRegister,
    pub vram: [u8; 2048],
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub palette_table: [u8; 32],
    internal_data_buf: u8,
    pub scanline: u16,
    pub cycles: usize,
    pub nmi_interrupt: Option<u8>,
}

impl PPU {
    pub fn new(memory: RcRef<Memory>, mirroring: Mirroring) -> Self {
        PPU {
            memory,
            mirroring,
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            oam_addr: 0,
            scroll: ScrollRegister::new(),
            addr: AddrRegister::new(),
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            internal_data_buf: 0,
            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
        }
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]
    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram: u16 = addr & 0b0010_1111_1111_1111; // mirror down 0x3000-0x3EFF to 0x2000 - 0x2EFF
        let vram_index: u16 = mirrored_vram - 0x2000; // to vram vector
        let name_table: u16 = vram_index / 0x0400;
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x0800,
            (Mirroring::Horizontal, 2) => vram_index - 0x0400,
            (Mirroring::Horizontal, 1) => vram_index - 0x0400,
            (Mirroring::Horizontal, 3) => vram_index - 0x0800,
            _ => vram_index,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn tick(&mut self, cycles: usize) -> bool {
        self.cycles += cycles;
        if self.cycles >= 341 {
            if self.is_sprite_0_hit(self.cycles) {
                self.status.set_sprite_zero_hit(true);
            }
            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status.set_vblank_status(true);
                self.status.set_sprite_zero_hit(false);
                if self.ctrl.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(1);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.nmi_interrupt = None;
                self.status.set_sprite_zero_hit(false);
                self.status.reset_vblank_status();
                return true;
            }
        }
        false
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }

    fn is_sprite_0_hit(&self, cycles: usize) -> bool {
        let y: usize = self.oam_data[0] as usize;
        let x: usize = self.oam_data[3] as usize;
        (y == self.scanline as usize) && (x <= cycles) && self.mask.show_sprites()
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status: bool = self.ctrl.generate_vblank_nmi();
        self.ctrl = ControlRegister::from_bits_truncate(value);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask = MaskRegister::from_bits_truncate(value);
    }

    pub fn read_status(&mut self) -> u8 {
        let data: u8 = self.status.snapshot();
        self.status.reset_vblank_status();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        data
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr: u16 = self.addr.get();
        match addr {
            0..=0x1FFF => {
                if self.memory().use_chr_ram {
                    self.memory_mut().chr_mem[addr as usize] = value;
                } else {
                    error!(
                        "[PPU] Attempted to write {:#04X} to CHR-ROM {:#06X}",
                        value, addr
                    );
                }
            }
            0x2000..=0x2FFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3EFF => {
                unimplemented!("PPU Addr {:#06X} shouldn't be used in reality", addr)
            }

            // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let add_mirror: u16 = addr - 0x0010;
                self.palette_table[(add_mirror - 0x3F00) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[(addr - 0x3F00) as usize] = value;
            }
            _ => panic!("Unexpected access to mirrored PPU space {:#06X}", addr),
        }
        self.increment_vram_addr();
    }

    pub fn read_data(&mut self) -> u8 {
        let addr: u16 = self.addr.get();

        self.increment_vram_addr();

        match addr {
            0..=0x1FFF => {
                let result: u8 = self.internal_data_buf;
                let chr_mem_data: u8 = self.memory().chr_mem[addr as usize];
                self.internal_data_buf = chr_mem_data;
                result
            }
            0x2000..=0x2FFF => {
                let result: u8 = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3EFF => {
                unimplemented!("PPU Addr {:#06X} shouldn't be used in reality", addr)
            }

            // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let add_mirror: u16 = addr - 0x0010;
                self.palette_table[(add_mirror - 0x3F00) as usize]
            }

            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3F00) as usize],
            _ => panic!("Unexpected access to mirrored PPU space {:#06X}", addr),
        }
    }

    pub fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam_data[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
    }
}
