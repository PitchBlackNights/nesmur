#[allow(unused_imports)]
use nes::NES;
use nes::cartridge::ROM;
use nes::ppu::PPU;
use nes::{BoxNESDevice, RcRef};

pub fn setup_nes(rom_path: &str) -> NES {
    let path: String = format!("tests/roms/{}", rom_path);
    let rom_bytes: Vec<u8> = std::fs::read(path).unwrap();
    let rom: ROM = ROM::new(&rom_bytes).unwrap();
    NES::new(
        rom,
        |_ppu: RcRef<PPU>,
         _device1: &mut Option<RcRef<BoxNESDevice>>,
         _device2: &mut Option<RcRef<BoxNESDevice>>| {},
    )
}
