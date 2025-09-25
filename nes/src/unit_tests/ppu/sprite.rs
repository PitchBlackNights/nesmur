use crate::ppu::{
    registers::ControlRegister,
    sprite::{Sprite, SpriteTileIndex},
};

#[test]
fn test_tile_base() {
    assert_eq!(SpriteTileIndex(0b0000_0000).base(), 0);
    assert_eq!(SpriteTileIndex(0b0000_0001).base(), 0x1000);
}

#[test]
fn test_sprite_tile_offset() {
    assert_eq!(SpriteTileIndex(0b0000_0101).large_offset(), 4 * 16);
    assert_eq!(SpriteTileIndex(0b0000_0101).small_offset(), 5 * 16);
}

#[test]
fn test_tile_address_small_no_flip() {
    let sprite: Sprite = Sprite::new(0, &[5, 7, 0, 0]);
    assert_eq!(
        sprite.tile_address(5, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (5 - 5)
    );
    assert_eq!(
        sprite.tile_address(8, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (8 - 5)
    );
    assert_eq!(
        sprite.tile_address(12, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (12 - 5)
    );
    assert_eq!(
        sprite.tile_address(13, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (5 - 5)
    );
}

#[test]
fn test_tile_address_small_flip_y() {
    let sprite: Sprite = Sprite::new(0, &[5, 7, 0b1000_0000, 0]);
    assert_eq!(
        sprite.tile_address(5, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (12 - 5)
    );
    assert_eq!(
        sprite.tile_address(8, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (9 - 5)
    );
    assert_eq!(
        sprite.tile_address(12, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (5 - 5)
    );
    assert_eq!(
        sprite.tile_address(13, ControlRegister::from_bits_truncate(0b0000_0000)),
        0 + (7 * 16) + (12 - 5)
    );
}

#[test]
fn test_tile_address_large_no_flip() {
    let sprite: Sprite = Sprite::new(0, &[5, 7, 0, 0]);
    let c: ControlRegister = ControlRegister::from_bits_truncate(0b0010_0000);
    assert_eq!(sprite.tile_address(5, c), 0x1000 + (6 * 16) + (5 - 5));
    assert_eq!(sprite.tile_address(8, c), 0x1000 + (6 * 16) + (8 - 5));
    assert_eq!(sprite.tile_address(12, c), 0x1000 + (6 * 16) + (12 - 5));
    assert_eq!(sprite.tile_address(13, c), 0x1000 + (6 * 16) + 8 + (13 - 5));
    assert_eq!(sprite.tile_address(16, c), 0x1000 + (6 * 16) + 8 + (16 - 5));
    assert_eq!(sprite.tile_address(19, c), 0x1000 + (6 * 16) + 8 + (19 - 5));
    assert_eq!(sprite.tile_address(21, c), 0x1000 + (6 * 16) + (5 - 5));
}

#[test]
fn test_tile_address_large_flip_y() {
    let sprite: Sprite = Sprite::new(0, &[5, 7, 0b1000_0000, 0]);
    let c: ControlRegister = ControlRegister::from_bits_truncate(0b0010_0000);
    assert_eq!(sprite.tile_address(5, c), 0x1000 + (6 * 16) + 8 + (20 - 5));
    assert_eq!(sprite.tile_address(6, c), 0x1000 + (6 * 16) + 8 + (19 - 5));
    assert_eq!(sprite.tile_address(20, c), 0x1000 + (6 * 16) + 0 + (5 - 5));
}

#[test]
fn test_color_index() {
    let mut sprite: Sprite = Sprite::new(0, &[0, 0, 0, 4]);
    sprite.data_lo = 0b1000_0010;
    sprite.data_hi = 0b0100_0010;
    assert_eq!(sprite.color_index(4 + 6), 3);
    assert_eq!(sprite.color_index(4 + 0), 1);
    assert_eq!(sprite.color_index(4 + 1), 2);
}

#[test]
fn test_color_index_flip_x() {
    let mut sprite: Sprite = Sprite::new(0, &[0, 0, 0b0100_0000, 4]);
    sprite.data_lo = 0b1000_0010;
    sprite.data_hi = 0b0100_0010;
    assert_eq!(sprite.color_index(4 + 7 - 6), 3);
    assert_eq!(sprite.color_index(4 + 7 - 0), 1);
    assert_eq!(sprite.color_index(4 + 7 - 1), 2);
}
