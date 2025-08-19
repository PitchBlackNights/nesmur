use std::hash::{Hash, Hasher};

use crate::ppu::sprite::Sprite;
use crate::ppu::{PPU, palette};
use crate::tools::{BitPlane, nth_bit};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Hash for RGB {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
        self.2.hash(state);
    }
}

pub struct Renderer {
    pub background_latch: BitPlane<u8>,
    pub background_shift: BitPlane<u16>,
    pub attribute_latch: BitPlane<u8>,
    pub attribute_shift: BitPlane<u8>,
    pub scratch_address: u16,
    pub nametable_entry: u8,
    pub attribute_entry: u8,
    pub primary_oam: Vec<Sprite>,
    pub secondary_oam: Vec<Sprite>,
    pub pixels: Vec<RGB>,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer: Renderer = Renderer {
            background_latch: BitPlane::new(0x00, 0x00),
            background_shift: BitPlane::new(0x0000, 0x0000),
            attribute_latch: BitPlane::new(0x00, 0x00),
            attribute_shift: BitPlane::new(0x00, 0x00),
            scratch_address: 0x0000,
            nametable_entry: 0x00,
            attribute_entry: 0x00,
            primary_oam: Vec::with_capacity(8),
            secondary_oam: Vec::with_capacity(8),
            pixels: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT),
        };
        renderer.reset();
        renderer
    }

    pub fn reset(&mut self) {
        self.primary_oam.clear();
        self.secondary_oam.clear();
        self.clear_pixels();
    }

    pub fn clear_pixels(&mut self) {
        self.pixels = vec![RGB(0, 0, 0); self.pixels.capacity()];
    }

    pub fn tick(&mut self, ppu: &mut PPU) {
        match (ppu.scanline, ppu.dot) {
            (0..=239, _) => {
                self.tick_sprites(false, ppu);
                self.tick_pixel(ppu);
                self.tick_background(false, ppu);
            }
            (261, _) => {
                self.tick_sprites(true, ppu);
                self.tick_pixel(ppu);
                self.tick_background(true, ppu);
            }
            (241, 1) => {
                ppu.status.set_vblank_status(true);
                // self.status.set_sprite_zero_hit(false);
                if ppu.ctrl.generate_vblank_nmi() {
                    ppu.nmi_interrupt = Some(1);
                }
            }
            (_, _) => {}
        };
    }

    pub fn tick_sprites(&mut self, pre: bool, ppu: &mut PPU) {
        match ppu.dot {
            1 => {
                self.secondary_oam.clear();
                if pre {
                    ppu.status.set_sprite_overflow(false);
                    ppu.status.set_sprite_zero_hit(false);
                }
            }
            257 => self.eval_sprites(ppu),
            321 => self.load_sprites(ppu),
            _ => {}
        }
    }

    pub fn tick_pixel(&mut self, ppu: &mut PPU) {
        match ppu.dot {
            2..=257 | 322..=337 => {
                let x: usize = ppu.dot - 2;
                let y: usize = ppu.scanline;
                if let Some(color_index) = self.render_pixel(x, y, ppu) {
                    self.set_pixel(x, y, color_index, ppu);
                }
                self.shift();
            }
            _ => {}
        }
    }

    pub fn tick_background(&mut self, pre: bool, ppu: &mut PPU) {
        match ppu.dot {
            2..=255 | 322..=337 => match ppu.dot % 8 {
                1 => {
                    self.scratch_address = ppu.vram_addr.get_nametable_addr();
                    self.reload_shift_registers();
                }
                2 => {
                    self.nametable_entry = ppu.raw_read_data(self.scratch_address);
                }
                3 => {
                    self.scratch_address = ppu.vram_addr.get_attribute_addr();
                }
                4 => {
                    self.attribute_entry = ppu.raw_read_data(self.scratch_address);
                    if ppu.vram_addr.get_coarse_y() & 2 != 0 {
                        self.attribute_entry >>= 4;
                    }
                    if ppu.vram_addr.get_coarse_x() & 2 != 0 {
                        self.attribute_entry >>= 2;
                    }
                }
                5 => {
                    self.scratch_address = ppu.ctrl.background_pattern_addr()
                        + ppu.vram_addr.get_tile_offset(self.nametable_entry);
                }
                6 => {
                    self.background_latch.lo = ppu.raw_read_data(self.scratch_address);
                }
                7 => {
                    self.scratch_address += 8;
                }
                0 => {
                    self.background_latch.hi = ppu.raw_read_data(self.scratch_address);

                    if ppu.mask.rendering() {
                        ppu.vram_addr.scroll_x();
                    }
                }
                _ => panic!("This shouldn't happen!"),
            },
            256 => {
                self.background_latch.hi = ppu.raw_read_data(self.scratch_address);
                if ppu.mask.rendering() {
                    ppu.vram_addr.scroll_y();
                }
            }
            257 => {
                self.reload_shift_registers();
                if ppu.mask.rendering() {
                    ppu.vram_addr.copy_x(ppu.temp_addr);
                }
            }
            280..=304 => {
                if pre && ppu.mask.rendering() {
                    ppu.vram_addr.copy_y(ppu.temp_addr);
                }
            }
            1 => {
                self.scratch_address = ppu.vram_addr.get_nametable_addr();
                if pre {
                    ppu.status.set_vblank_status(false);
                }
            }
            321 | 339 => {
                self.scratch_address = ppu.vram_addr.get_nametable_addr();
            }
            338 => {
                self.nametable_entry = ppu.raw_read_data(self.scratch_address);
            }
            340 => {
                self.nametable_entry = ppu.raw_read_data(self.scratch_address);
                if pre && ppu.mask.rendering() && ppu.odd_frame {
                    ppu.dot += 1;
                }
            }

            _ => (),
        }
    }

    pub fn render_pixel(&mut self, x: usize, y: usize, ppu: &mut PPU) -> Option<u8> {
        if y < 240 && x < 256 {
            let background_color: u8 = self.render_background_pixel(x, ppu);
            let (sprite_color, sprite_behind, possible_zero_hit): (u8, bool, bool) =
                self.render_sprite_pixel(x, ppu);

            if possible_zero_hit && background_color != 0 {
                ppu.status.set_sprite_zero_hit(true);
            }

            let colors: [u8; 2] = if sprite_behind {
                [background_color, sprite_color]
            } else {
                [sprite_color, background_color]
            };

            Some(if colors[0] == 0 { colors[1] } else { colors[0] })
        } else {
            None
        }
    }

    pub fn render_background_pixel(&self, x: usize, ppu: &mut PPU) -> u8 {
        if !ppu.mask.rendering_background(x) {
            return 0;
        }

        let mut background_color: u8 = nth_bit(self.background_shift.hi, 15 - ppu.scroll_fine_x)
            << 1
            | nth_bit(self.background_shift.lo, 15 - ppu.scroll_fine_x);
        if background_color != 0 {
            background_color |= (nth_bit(self.attribute_shift.hi, 7 - ppu.scroll_fine_x) << 1
                | nth_bit(self.attribute_shift.lo, 7 - ppu.scroll_fine_x))
                << 2;
        }
        background_color
    }

    pub fn render_sprite_pixel(&mut self, x: usize, ppu: &mut PPU) -> (u8, bool, bool) {
        if !ppu.mask.rendering_sprites(x) {
            return (0, false, false);
        };

        let mut color: u8 = 0;
        let mut behind: bool = false;
        let mut possible_zero_hit: bool = false;

        for sprite in self.primary_oam.iter().rev() {
            let sprite_color_index: u8 = sprite.color_index(x);

            if sprite_color_index != 0 {
                if sprite.oam_index == 0 && x != 255 {
                    possible_zero_hit = true;
                }
                color = 0b1_00_00 | sprite.status_palette << 2 | sprite_color_index;
                behind = sprite.status_behind_background;
            }
        }

        (color, behind, possible_zero_hit)
    }

    pub fn eval_sprites(&mut self, ppu: &mut PPU) {
        self.secondary_oam.clear();
        for i in 0..64 {
            let address: usize = i * 4;
            let sprite: Sprite = Sprite::new(i, &ppu.oam_data[address..address + 4]);

            // There's a subtle NES detail at play here. We're loading sprites for the NEXT scanline,
            // but we're comparing `sprite.y` to the CURRENT scanline. This is because `sprite.y` values
            // are always offset by 1. So to draw a sprite on scanline 1, you set its Y to 0.
            if ppu.scanline >= sprite.y as usize
                && ppu.scanline < sprite.y as usize + ppu.ctrl.sprite_size() as usize
            {
                if self.secondary_oam.len() == 8 {
                    ppu.status.set_sprite_overflow(true);
                    break;
                }
                self.secondary_oam.push(sprite);
            }
        }
    }

    pub fn load_sprites(&mut self, ppu: &mut PPU) {
        let mut sprites: Vec<Sprite> = self.secondary_oam.clone();
        for sprite in sprites.iter_mut() {
            let tile_address: u16 = sprite.tile_address(ppu.scanline, ppu.ctrl);
            sprite.data_lo = ppu.raw_read_data(tile_address);
            sprite.data_hi = ppu.raw_read_data(tile_address + 8);
        }
        self.primary_oam = sprites;
    }

    pub fn reload_shift_registers(&mut self) {
        self.background_shift.lo =
            (self.background_shift.lo & 0xFF00) | self.background_latch.lo as u16;
        self.background_shift.hi =
            (self.background_shift.hi & 0xFF00) | self.background_latch.hi as u16;
        self.attribute_latch.lo = self.attribute_entry & 0b0000_0001;
        self.attribute_latch.hi = (self.attribute_entry & 0b0000_0010) >> 1;
    }

    pub fn shift(&mut self) {
        self.background_shift.lo <<= 1;
        self.background_shift.hi <<= 1;
        self.attribute_shift.lo = self.attribute_shift.lo << 1 | self.attribute_latch.lo;
        self.attribute_shift.hi = self.attribute_shift.hi << 1 | self.attribute_latch.hi;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color_index: u8, ppu: &mut PPU) {
        let pixel_index: usize = y * 256 + x;
        let palette_offset: u16 = if ppu.mask.rendering() {
            color_index as u16
        } else {
            0
        };
        let rgb_index: usize = ppu.raw_read_data(0x3F00 + palette_offset) as usize;
        self.pixels[pixel_index] = palette::NTSC[rgb_index];
    }
}
