#[allow(unused_imports)]
mod prelude {
    pub use bitflags::bitflags;
    pub use nes::bus::Mem;
    pub use nes::ppu::NesPPU;
    pub use nes::tools;
    pub use nes::tools::NESAccess;
    // pub use log::{debug, error, info, trace, warn};
}
use nes::NES;
use nes::cartridge::Rom;
// use prelude::*;

pub fn setup_nes(rom_path: &str) -> NES {
    let path: String = format!("tests/roms/{}", rom_path);
    let rom_bytes: Vec<u8> = std::fs::read(path).unwrap();
    let rom: Rom = Rom::new(&rom_bytes).unwrap();
    NES::new(rom, |_ppu, _joypad1| {})
}
