pub mod frame;
pub mod palette;

use frame::Frame;
use nes::cartridge::Mirroring;
use nes::ppu::PPU;

fn bg_pallette(ppu: &PPU, attribute_table: &[u8], tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_idx: usize = tile_row / 4 * 8 + tile_column / 4;
    let attr_byte: u8 = attribute_table[attr_table_idx];

    let pallet_idx: u8 = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b0000_0011,
        (1, 0) => (attr_byte >> 2) & 0b0000_0011,
        (0, 1) => (attr_byte >> 4) & 0b0000_0011,
        (1, 1) => (attr_byte >> 6) & 0b0000_0011,
        (_, _) => panic!("This shouldn't happen!"),
    };

    let pallete_start: usize = 1 + (pallet_idx as usize) * 4;
    [
        ppu.palette_table[0],
        ppu.palette_table[pallete_start],
        ppu.palette_table[pallete_start + 1],
        ppu.palette_table[pallete_start + 2],
    ]
}

fn sprite_palette(ppu: &PPU, pallete_idx: u8) -> [u8; 4] {
    let start: usize = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}

struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Rect { x1, y1, x2, y2 }
    }
}

fn render_name_table(
    ppu: &PPU,
    frame: &mut Frame,
    name_table: &[u8],
    view_port: Rect,
    shift_x: isize,
    shift_y: isize,
) {
    let bank: u16 = ppu.ctrl.bknd_pattern_addr();

    let attribute_table: &[u8] = &name_table[0x03C0..0x0400];

    #[allow(clippy::needless_range_loop)]
    for i in 0..0x03C0 {
        let tile_column: usize = i % 32;
        let tile_row: usize = i / 32;
        let tile_idx: u16 = name_table[i] as u16;
        let tile: &[u8] =
            &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];
        let palette: [u8; 4] = bg_pallette(ppu, attribute_table, tile_column, tile_row);

        for y in 0..=7 {
            let mut upper: u8 = tile[y];
            let mut lower: u8 = tile[y + 8];

            for x in (0..=7).rev() {
                let value: u8 = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb: (u8, u8, u8) = match value {
                    0 => palette::SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => palette::SYSTEM_PALLETE[palette[1] as usize],
                    2 => palette::SYSTEM_PALLETE[palette[2] as usize],
                    3 => palette::SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("This shouldn't happen!"),
                };
                let pixel_x: usize = tile_column * 8 + x;
                let pixel_y: usize = tile_row * 8 + y;

                if pixel_x >= view_port.x1
                    && pixel_x < view_port.x2
                    && pixel_y >= view_port.y1
                    && pixel_y < view_port.y2
                {
                    frame.set_pixel(
                        (shift_x + pixel_x as isize) as usize,
                        (shift_y + pixel_y as isize) as usize,
                        rgb,
                    );
                }
            }
        }
    }
}

pub fn render(ppu: &PPU, frame: &mut Frame) {
    let scroll_x: usize = (ppu.scroll.scroll_x) as usize;
    let scroll_y: usize = (ppu.scroll.scroll_y) as usize;

    let (main_nametable, second_nametable): (&[u8], &[u8]) =
        match (&ppu.mirroring, ppu.ctrl.nametable_addr()) {
            (Mirroring::Vertical, 0x2000)
            | (Mirroring::Vertical, 0x2800)
            | (Mirroring::Horizontal, 0x2000)
            | (Mirroring::Horizontal, 0x2400) => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
            (Mirroring::Vertical, 0x2400)
            | (Mirroring::Vertical, 0x2C00)
            | (Mirroring::Horizontal, 0x2800)
            | (Mirroring::Horizontal, 0x2C00) => (&ppu.vram[0x400..0x800], &ppu.vram[0..0x400]),
            (_, _) => {
                panic!("Not supported mirroring type {:?}", ppu.mirroring);
            }
        };

    render_name_table(
        ppu,
        frame,
        main_nametable,
        Rect::new(scroll_x, scroll_y, 256, 240),
        -(scroll_x as isize),
        -(scroll_y as isize),
    );
    if scroll_x > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, scroll_x, 240),
            (256 - scroll_x) as isize,
            0,
        );
    } else if scroll_y > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, 256, scroll_y),
            0,
            (240 - scroll_y) as isize,
        );
    }

    for i in (0..ppu.oam_data.len()).step_by(4).rev() {
        let tile_idx: u16 = ppu.oam_data[i + 1] as u16;
        let tile_x: usize = ppu.oam_data[i + 3] as usize;
        let tile_y: usize = ppu.oam_data[i] as usize;

        let flip_vertical: bool = ppu.oam_data[i + 2] >> 7 & 1 == 1;
        let flip_horizontal: bool = ppu.oam_data[i + 2] >> 6 & 1 == 1;
        let pallette_idx: u8 = ppu.oam_data[i + 2] & 0b0000_0011;
        let sprite_palette: [u8; 4] = sprite_palette(ppu, pallette_idx);
        let bank: u16 = ppu.ctrl.sprt_pattern_addr();

        let tile: &[u8] =
            &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];

        for y in 0..=7 {
            let mut upper: u8 = tile[y];
            let mut lower: u8 = tile[y + 8];
            'next: for x in (0..=7).rev() {
                let value: u8 = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb: (u8, u8, u8) = match value {
                    0 => continue 'next, // skip coloring the pixel
                    1 => palette::SYSTEM_PALLETE[sprite_palette[1] as usize],
                    2 => palette::SYSTEM_PALLETE[sprite_palette[2] as usize],
                    3 => palette::SYSTEM_PALLETE[sprite_palette[3] as usize],
                    _ => panic!("This shouldn't happen!"),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => {
                        frame.set_pixel(tile_x + x, tile_y + y, rgb);
                    }
                    (true, false) => {
                        frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb);
                    }
                    (false, true) => {
                        frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb);
                    }
                    (true, true) => {
                        frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb);
                    }
                }
            }
        }
    }
}
