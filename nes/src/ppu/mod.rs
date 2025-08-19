pub mod palette;
pub mod registers;
pub mod renderer;
pub mod sprite;

use crate::cartridge::Mirroring;
use crate::memory::Memory;
use crate::ppu::renderer::Renderer;
use crate::prelude::*;
use crate::{RcRef, Region};
use registers::addr::AddrRegister;
use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::status::StatusRegister;
// use registers::scroll::ScrollRegister;

#[rustfmt::skip]
impl NESAccess<'_> for PPU {
    fn memory(&self) -> Ref<Memory> { self.memory.borrow() }
    fn memory_mut(&self) -> RefMut<Memory> { self.memory.borrow_mut() }
}

pub struct PPU {
    memory: RcRef<Memory>,

    pub vram: [u8; 2048],
    pub mirroring: Mirroring,
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub palette_table: [u8; 32],

    pub register_latch: bool,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    // pub scroll: ScrollRegister,
    pub vram_addr: AddrRegister,
    pub temp_addr: AddrRegister,
    pub scroll_fine_x: u8,
    internal_data_buf: u8,

    pub scanline: usize,
    pub odd_frame: bool,
    pub dot: usize,
    pub cycles: usize,
    pub nmi_interrupt: Option<u8>,
    pub region: Region,
}

impl PPU {
    pub fn new(memory: RcRef<Memory>, mirroring: Mirroring) -> Self {
        PPU {
            memory,
            vram: [0x00; 2048],
            mirroring,
            oam_addr: 0x00,
            oam_data: [0x00; 256],
            palette_table: [0x00; 32],

            register_latch: false,
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            // scroll: ScrollRegister::new(),
            vram_addr: AddrRegister::new(),
            temp_addr: AddrRegister::new(),
            scroll_fine_x: 0,
            internal_data_buf: 0,

            scanline: 0,
            odd_frame: false,
            dot: 0,
            cycles: 0,
            nmi_interrupt: None,
            region: Region::NTSC,
        }
    }

    pub fn reset(&mut self) {
        self.ctrl.reset();
        self.mask.reset();
        // self.scroll.reset();

        self.scanline = 0;
        self.odd_frame = false;
        self.dot = 0;
        self.cycles = 0;
        self.nmi_interrupt = None;
    }

    pub fn tick(&mut self, mut renderer: RefMut<Renderer>) -> bool {
        renderer.tick(self);
        self.step()

        // if self.dot >= 341 {
        //     if self.is_sprite_0_hit(self.dot) {
        //         self.status.set_sprite_zero_hit(true);
        //     }
        //     self.dot %= 341;
        //     self.scanline += 1;
        //
        //     if self.scanline >= 262 {
        //         self.scanline = 0;
        //         self.odd_frame = !self.odd_frame;
        //         self.nmi_interrupt = None;
        //         self.status.set_sprite_zero_hit(false);
        //         self.status.reset_vblank_status();
        //         return true;
        //     }
        // }
    }

    pub fn step(&mut self) -> bool {
        self.dot += 1;
        self.cycles += 1;

        if self.dot >= 341 {
            self.dot %= 341;
            self.scanline += 1;

            if self.scanline >= 262 {
                self.scanline = 0;
                self.odd_frame = !self.odd_frame;
                return true;
            }
        }
        false
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
        self.vram_addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }

    // fn is_sprite_0_hit(&self, cycles: usize) -> bool {
    //     let y: usize = self.oam_data[0] as usize;
    //     let x: usize = self.oam_data[3] as usize;
    //     (y == self.scanline) && (x <= cycles) && self.mask.show_sprites()
    // }

    pub fn write_to_ctrl(&mut self, value: u8) {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        if self.cycles >= 9886 {
            let before_nmi_status: bool = self.ctrl.generate_vblank_nmi();
            self.ctrl = ControlRegister::from_bits_truncate(value);
            if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
                self.nmi_interrupt = Some(1);
            }
            self.temp_addr.set_nametable(self.ctrl.get_nametable());
        }
    }

    pub fn write_to_mask(&mut self, value: u8) {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        if self.cycles >= 9886 {
            self.mask = MaskRegister::from_bits_truncate(value);
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let data: u8 = self.status.snapshot();
        self.status.reset_vblank_status();
        // self.vram_addr.reset_latch();
        // self.scroll.reset_latch();
        self.register_latch = false;
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
        // https://www.nesdev.org/wiki/PPU_power_up_state
        if self.cycles >= 9886 {
            // self.scroll.write(value);
            if self.register_latch {
                self.temp_addr.set_fine_y(value);
                self.temp_addr.set_coarse_y(value >> 3);
            } else {
                self.scroll_fine_x = value & 0b0000_0111;
                self.temp_addr.set_coarse_x(value >> 3);
            }
            self.register_latch = !self.register_latch;
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        if self.cycles >= 9886 {
            if self.register_latch {
                self.temp_addr.set_lo_byte(value);
                self.vram_addr = self.temp_addr;
            } else {
                self.temp_addr.set_hi_byte(value);
            }
            self.register_latch = !self.register_latch;
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr: u16 = self.vram_addr.get();
        self.raw_write_to_data(addr, value);
        self.increment_vram_addr();
    }

    pub fn raw_write_to_data(&mut self, addr: u16, value: u8) {
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
    }

    pub fn read_data(&mut self) -> u8 {
        let addr: u16 = self.vram_addr.get();

        self.increment_vram_addr();

        if addr <= 0x3EFF {
            let result: u8 = self.internal_data_buf;
            self.internal_data_buf = self.raw_read_data(addr);
            result
        } else {
            self.raw_read_data(addr)
        }
    }

    pub fn raw_read_data(&mut self, addr: u16) -> u8 {
        match addr {
            0..=0x1FFF => self.memory().chr_mem[addr as usize],
            0x2000..=0x2FFF => self.vram[self.mirror_vram_addr(addr) as usize],
            0x3000..=0x3EFF => {
                unimplemented!("PPU Addr {:#06X} shouldn't be used in reality", addr)
            }

            // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let addr_mirror: u16 = addr - 0x0010;
                self.palette_table[(addr_mirror - 0x3F00) as usize]
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
