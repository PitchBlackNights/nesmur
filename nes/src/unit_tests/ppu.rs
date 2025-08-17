use super::*;
use crate::cartridge::Mirroring;
use crate::ppu::PPU;

#[test]
fn test_ppu_vram_writes() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ppu_addr(0x23);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x66);

    assert_eq!(ppu.vram[0x0305], 0x66);
}

#[test]
fn test_ppu_vram_reads() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ctrl(0);
    ppu.vram[0x0305] = 0x66;
    ppu.write_to_ppu_addr(0x23);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.addr.get(), 0x2306);
    assert_eq!(ppu.read_data(), 0x66);
}

#[test]
fn test_ppu_vram_reads_cross_page() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ctrl(0);
    ppu.vram[0x01ff] = 0x66;
    ppu.vram[0x0200] = 0x77;
    ppu.write_to_ppu_addr(0x21);
    ppu.write_to_ppu_addr(0xff);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x66);
    assert_eq!(ppu.read_data(), 0x77);
}

#[test]
fn test_ppu_vram_reads_step_32() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ctrl(0b100);
    ppu.vram[0x01ff] = 0x66;
    ppu.vram[0x01ff + 32] = 0x77;
    ppu.vram[0x01ff + 64] = 0x88;
    ppu.write_to_ppu_addr(0x21);
    ppu.write_to_ppu_addr(0xff);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x66);
    assert_eq!(ppu.read_data(), 0x77);
    assert_eq!(ppu.read_data(), 0x88);
}

// Horizontal: https://wiki.nesdev.com/w/index.php/Mirroring
//   [0x2000 A ] [0x2400 a ]
//   [0x2800 B ] [0x2C00 b ]
#[test]
fn test_vram_horizontal_mirror() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ppu_addr(0x24);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x66); // Write to a
    ppu.write_to_ppu_addr(0x28);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x77); // Write to B
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x66); // Read from A

    ppu.write_to_ppu_addr(0x2C);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x77); // Read from b
}

// Vertical: https://wiki.nesdev.com/w/index.php/Mirroring
//   [0x2000 A ] [0x2400 B ]
//   [0x2800 a ] [0x2C00 b ]
#[test]
fn test_vram_vertical_mirror() {
    let mut ppu: PPU = empty_ppu(Mirroring::Vertical);

    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x66); // Write to A
    ppu.write_to_ppu_addr(0x2C);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x77); // Write to b
    ppu.write_to_ppu_addr(0x28);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x66); // Read from a

    ppu.write_to_ppu_addr(0x24);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x77); // Read from B
}

#[test]
fn test_read_status_resets_latch() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);
    ppu.vram[0x0305] = 0x66;

    ppu.write_to_ppu_addr(0x21);
    ppu.write_to_ppu_addr(0x23);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_ne!(ppu.read_data(), 0x66);

    ppu.read_status();
    ppu.write_to_ppu_addr(0x23);
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into buffer

    assert_eq!(ppu.read_data(), 0x66);
}

#[test]
fn test_ppu_vram_mirroring() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_ctrl(0);
    ppu.vram[0x0305] = 0x66;
    ppu.write_to_ppu_addr(0x63); // 0x6305 -> 0x2305
    ppu.write_to_ppu_addr(0x05);
    ppu.read_data(); // Load into_buffer

    assert_eq!(ppu.read_data(), 0x66);
}

#[test]
fn test_read_status_resets_vblank() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.status.set_vblank_status(true);
    let status: u8 = ppu.read_status();

    assert_eq!(status >> 7, 1);
    assert_eq!(ppu.status.snapshot() >> 7, 0);
}

#[test]
fn test_oam_read_write() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    ppu.write_to_oam_addr(0x10);
    ppu.write_to_oam_data(0x66);
    ppu.write_to_oam_data(0x77);
    ppu.write_to_oam_addr(0x10);

    assert_eq!(ppu.read_oam_data(), 0x66);

    ppu.write_to_oam_addr(0x11);

    assert_eq!(ppu.read_oam_data(), 0x77);
}

#[test]
fn test_oam_dma() {
    let mut ppu: PPU = empty_ppu(Mirroring::Horizontal);

    let mut data: [u8; 256] = [0x66; 256];
    data[0] = 0x77;
    data[255] = 0x88;
    ppu.write_to_oam_addr(0x10);
    ppu.write_oam_dma(&data);
    ppu.write_to_oam_addr(0xf); //wrap around

    assert_eq!(ppu.read_oam_data(), 0x88);

    ppu.write_to_oam_addr(0x10);

    assert_eq!(ppu.read_oam_data(), 0x77);

    ppu.write_to_oam_addr(0x11);

    assert_eq!(ppu.read_oam_data(), 0x66);
}
