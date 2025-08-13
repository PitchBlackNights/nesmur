use super::*;
use crate::cartridge::Mirroring;

#[test]
fn test_rom_creation() {
    let test_rom: Vec<u8> = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });
    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::Vertical);
}

#[test]
fn test_with_trainer() {
    let test_rom: Vec<u8> = create_rom(TestRom {
        header: vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            0x02,
            0x01,
            0x31 | 0b0000_0100,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ],
        trainer: Some(vec![0; 512]),
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });

    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::Vertical);
}

#[test]
fn test_nes2_is_not_supported() {
    let test_rom: Vec<u8> = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x31, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        trainer: None,
        pgp_rom: vec![1; 1 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });
    let rom: Result<Rom, String> = Rom::new(&test_rom);

    match rom {
        Result::Ok(_) => assert!(false, "should not load rom"),
        Result::Err(str) => assert_eq!(str, "NES2.0 format is not supported"),
    }
}
