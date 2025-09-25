use super::registers::ControlRegister;
use crate::tools::nth_bit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpriteTileIndex(pub u8);

impl SpriteTileIndex {
    pub fn base(&self) -> u16 {
        0x1000 * (self.0 % 2) as u16
    }

    pub fn large_offset(&self) -> u16 {
        ((self.0 & 0b1111_1110) as u16) << 4
    }

    pub fn small_offset(&self) -> u16 {
        (self.0 as u16) << 4
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub status_flip_y: bool,
    pub status_flip_x: bool,
    pub status_behind_background: bool,
    pub status_palette: u8,
    pub tile_index: SpriteTileIndex,
    pub data_lo: u8,
    pub data_hi: u8,
    pub oam_index: usize,
}

impl Sprite {
    pub fn new(oam_index: usize, bytes: &[u8]) -> Self {
        Sprite {
            x: bytes[3],
            y: bytes[0],
            status_flip_y: bytes[2] & 0b1000_0000 != 0,
            status_flip_x: bytes[2] & 0b0100_0000 != 0,
            status_behind_background: bytes[2] & 0b0010_0000 != 0,
            status_palette: bytes[2] & 0b0000_0011,
            tile_index: SpriteTileIndex(bytes[1]),
            data_lo: 0,
            data_hi: 0,
            oam_index,
        }
    }

    pub fn tile_address(&self, scanline: usize, control: ControlRegister) -> u16 {
        let tile_address: u16 = if control.sprite_size() == 16 {
            self.tile_index.base() + self.tile_index.large_offset()
        } else {
            control.sprite_pattern_addr() + self.tile_index.small_offset()
        };

        let mut y_offset: u16 = (scanline as u16 - self.y as u16) % control.sprite_size() as u16;
        if self.status_flip_y {
            y_offset = control.sprite_size() as u16 - 1 - y_offset;
        }

        tile_address + y_offset + if y_offset < 8 { 0 } else { 8 }
    }

    pub fn color_index(&self, x: usize) -> u8 {
        let mut sprite_x: u16 = x.wrapping_sub(self.x as usize) as u16;

        if sprite_x < 8 {
            if self.status_flip_x {
                sprite_x = 7 - sprite_x;
            }
            nth_bit(self.data_hi, 7 - sprite_x) << 1 | nth_bit(self.data_lo, 7 - sprite_x)
        } else {
            0
        }
    }
}
