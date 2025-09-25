use super::*;
use crate::{
    cartridge::Mirroring,
    ppu::{PPU, registers::MaskRegister, renderer::Renderer, sprite::Sprite},
};

fn build_ppu(mirroring: Mirroring) -> PPU {
    let memory: Memory = Memory {
        cpu_vram: [0x00; 2048],
        prg_rom: vec![0x00; PRG_ROM_PAGE_SIZE * 2],
        prg_ram: vec![0x00; PRG_RAM_PAGE_SIZE],
        chr_mem: vec![0x00; CHR_ROM_PAGE_SIZE],
        use_chr_ram: true,
    };

    let mut ppu: PPU = PPU::new(Rc::new(RefCell::new(memory)), mirroring);
    ppu.cycles = 9886;

    ppu
}

fn quick_setup() -> (PPU, Renderer) {
    (empty_ppu(Mirroring::Horizontal), Renderer::new())
}

fn quick_setup_with_rom() -> (PPU, Renderer) {
    (build_ppu(Mirroring::Horizontal), Renderer::new())
}

#[test]
fn test_evaluate_sprites() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.scanline = 10;
    ppu.oam_data[0] = 10;
    ppu.oam_data[4] = 10 - 7;
    ppu.oam_data[8] = 10 - 8;
    ppu.oam_data[12] = 11;
    renderer.eval_sprites(&mut ppu);
    assert_eq!(renderer.secondary_oam.len(), 2);
}

#[test]
fn test_sprite_overflow() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.scanline = 10;
    for i in 0..8 {
        ppu.oam_data[i * 4] = 0x0A;
    }
    renderer.eval_sprites(&mut ppu);
    assert_eq!(renderer.secondary_oam.len(), 8);
    assert_eq!(ppu.status.is_sprite_overflow(), false);

    ppu.oam_data[8 * 4] = 0x0A;
    renderer.eval_sprites(&mut ppu);
    assert_eq!(renderer.secondary_oam.len(), 8);
    assert_eq!(ppu.status.is_sprite_overflow(), true);
}

#[test]
fn test_load_sprites() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup_with_rom();

    for i in 0..256u16 {
        ppu.raw_write_to_data(i, i as u8);
    }

    renderer
        .secondary_oam
        .push(Sprite::new(0, &[0x05, 0x03, 0x01, 0x02]));
    ppu.scanline = 6;
    renderer.load_sprites(&mut ppu);

    assert_eq!(renderer.secondary_oam[0].x, renderer.primary_oam[0].x);
    assert_eq!(renderer.secondary_oam[0].y, renderer.primary_oam[0].y);
    assert_eq!(
        renderer.secondary_oam[0].status_flip_y,
        renderer.primary_oam[0].status_flip_y
    );
    assert_eq!(
        renderer.secondary_oam[0].status_flip_x,
        renderer.primary_oam[0].status_flip_x
    );
    assert_eq!(
        renderer.secondary_oam[0].status_behind_background,
        renderer.primary_oam[0].status_behind_background
    );
    assert_eq!(
        renderer.secondary_oam[0].status_palette,
        renderer.primary_oam[0].status_palette
    );
    assert_eq!(
        renderer.secondary_oam[0].tile_index,
        renderer.primary_oam[0].tile_index
    );
    assert_eq!(
        renderer.primary_oam[0].data_lo,
        renderer.primary_oam[0].tile_address(ppu.scanline, ppu.ctrl) as u8
    );
    assert_eq!(
        renderer.primary_oam[0].data_hi,
        renderer.primary_oam[0].tile_address(ppu.scanline, ppu.ctrl) as u8 + 8
    );
}

#[test]
fn test_render_background_pixel() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1110);
    renderer.background_shift.hi = 0b1010_0000_0000_0000;
    renderer.background_shift.lo = 0b1100_0000_0000_0000;
    renderer.attribute_shift.hi = 0b1010_0000;
    renderer.attribute_shift.lo = 0b1100_0000;
    ppu.scroll_fine_x = 0;
    assert_eq!(renderer.render_background_pixel(0x00, &mut ppu), 0x0F);

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1100);
    assert_eq!(renderer.render_background_pixel(0, &mut ppu), 0x00);
    assert_eq!(renderer.render_background_pixel(8, &mut ppu), 0x0F);

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_0110);
    assert_eq!(renderer.render_background_pixel(0, &mut ppu), 0x00);
}

#[test]
fn test_render_sprite_pixel() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    let mut sprite: Sprite = Sprite::new(0, &[0x00, 0x00, 0x00, 0x00]);
    sprite.data_lo = 0b0100_0000;
    sprite.data_hi = 0b0100_0000;
    renderer.primary_oam.push(sprite);

    // Bit 5 sets the "behind bg" flag
    let mut sprite: Sprite = Sprite::new(1, &[0x00, 0x00, 0x23, 0x00]);
    sprite.data_lo = 0b0000_0000;
    sprite.data_hi = 0b0001_0000;
    renderer.primary_oam.push(sprite);

    let mut sprite: Sprite = Sprite::new(2, &[0x00, 0x00, 0x00, 0x00]);
    sprite.data_lo = 0b0100_0000;
    sprite.data_hi = 0b0000_0000;
    renderer.primary_oam.push(sprite);

    ppu.mask = MaskRegister::from_bits_truncate(0b0000_1110); // Hide sprites
    assert_eq!(
        renderer.render_sprite_pixel(0, &mut ppu),
        (0x00, false, false)
    );

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1010); // Hide left 8px of sprites
    assert_eq!(
        renderer.render_sprite_pixel(0, &mut ppu),
        (0x00, false, false)
    );

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1110); // Show all sprites and bg
    assert_eq!(
        renderer.render_sprite_pixel(1, &mut ppu),
        (0x13, false, true)
    );
    assert_eq!(
        renderer.render_sprite_pixel(3, &mut ppu),
        (0x1E, true, false)
    );
}

#[test]
fn test_reload_shift() {
    let (_, mut renderer): (PPU, Renderer) = quick_setup();

    renderer.background_shift.lo = 0b1010_1010_1010_1010;
    renderer.background_shift.hi = 0b0101_0101_0101_0101;
    renderer.background_latch.lo = 0b0000_0001;
    renderer.background_latch.hi = 0b0000_0010;
    renderer.attribute_entry = 0x03;
    renderer.reload_shift_registers();

    assert_eq!(renderer.background_shift.lo, 0b1010_1010_0000_0001);
    assert_eq!(renderer.background_shift.hi, 0b0101_0101_0000_0010);
    assert_eq!(renderer.attribute_latch.lo, 0b0000_0001);
    assert_eq!(renderer.attribute_latch.hi, 0b0000_0001);
}

#[test]
fn test_shift() {
    let (_, mut renderer): (PPU, Renderer) = quick_setup();

    renderer.background_shift.lo = 0b1010_1010_1010_1010;
    renderer.background_shift.hi = 0b0101_0101_0101_0101;
    renderer.attribute_latch.lo = 0b0000_0000;
    renderer.attribute_latch.hi = 0b0000_0001;
    renderer.attribute_entry = 0x03;
    renderer.shift();

    assert_eq!(renderer.background_shift.lo, 0b0101_0101_0101_0100);
    assert_eq!(renderer.background_shift.hi, 0b1010_1010_1010_1010);
    assert_eq!(renderer.attribute_shift.lo, 0b0000_0000);
    assert_eq!(renderer.attribute_shift.hi, 0b0000_0001);
}

#[test]
fn test_render_pixel_transparent_sprite_front() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1110);
    renderer.background_shift.hi = 0b1111_0000_0000_0000;
    renderer.background_shift.lo = 0b1111_0000_0000_0000;
    assert_eq!(renderer.render_background_pixel(0, &mut ppu), 0x03);

    let mut sprite: Sprite = Sprite::new(0, &[0x00, 0x00, 0x00, 0x00]);
    sprite.data_lo = 0b0100_0000;
    sprite.data_hi = 0b0100_0000;
    renderer.primary_oam.push(sprite);

    assert_eq!(
        renderer.render_sprite_pixel(0, &mut ppu),
        (0x00, false, false)
    );
    assert_eq!(renderer.render_pixel(0, 0, &mut ppu), Some(0x03));
    assert_eq!(ppu.status.is_sprite_0_hit(), false);
}

#[test]
fn test_render_pixel_opaque_sprite_front() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1110);
    renderer.background_shift.hi = 0b1111_0000_0000_0000;
    renderer.background_shift.lo = 0b1111_0000_0000_0000;
    assert_eq!(renderer.render_background_pixel(0, &mut ppu), 0x03);

    let mut sprite: Sprite = Sprite::new(0, &[0x00, 0x00, 0x00, 0x00]);
    sprite.data_lo = 0b1000_0000;
    sprite.data_hi = 0b0000_0000;
    renderer.primary_oam.push(sprite);

    assert_eq!(
        renderer.render_sprite_pixel(0, &mut ppu),
        (0x11, false, true)
    );
    assert_eq!(renderer.render_pixel(0, 0, &mut ppu), Some(0x11));
    assert_eq!(ppu.status.is_sprite_0_hit(), true);
}

#[test]
fn test_render_pixel_opaque_sprite_behind() {
    let (mut ppu, mut renderer): (PPU, Renderer) = quick_setup();

    ppu.mask = MaskRegister::from_bits_truncate(0b0001_1110);
    renderer.background_shift.hi = 0b1111_0000_0000_0000;
    renderer.background_shift.lo = 0b1111_0000_0000_0000;
    assert_eq!(renderer.render_background_pixel(0, &mut ppu), 0x03);

    let mut sprite: Sprite = Sprite::new(0, &[0x00, 0x00, 0x20, 0x00]);
    sprite.data_lo = 0b1000_0000;
    sprite.data_hi = 0b0000_0000;
    renderer.primary_oam.push(sprite);

    assert_eq!(
        renderer.render_sprite_pixel(0, &mut ppu),
        (0b0001_0001, true, true)
    );
    assert_eq!(renderer.render_pixel(0, 0, &mut ppu), Some(0x03));
    assert_eq!(ppu.status.is_sprite_0_hit(), true);
}
