#[allow(unused_imports)]
use nes::NES;
use nes::cartridge::ROM;

pub fn setup_nes(rom_path: &str) -> NES {
    let path: String = format!("tests/roms/{}", rom_path);
    let rom_bytes: Vec<u8> = std::fs::read(path).unwrap();
    let rom: ROM = ROM::new(&rom_bytes).unwrap();
    NES::new(rom)
}
