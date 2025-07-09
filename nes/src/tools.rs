use crate::apu::APU;
use crate::bus::Bus;
use crate::cartridge::Rom;
use crate::ppu::PPU;
use std::cell::{Ref, RefMut};

#[rustfmt::skip]
pub trait NESAccess {
    fn bus(&self) -> Ref<Bus> { panic!("Access to `Bus` is prohibited") }
    fn bus_mut(&self) -> RefMut<Bus> { panic!("Access to `Bus` is prohibited") }
    fn apu(&self) -> Ref<APU> { panic!("Access to `APU` is prohibited") }
    fn apu_mut(&self) -> RefMut<APU> { panic!("Access to `APU` is prohibited") }
    fn ppu(&self) -> Ref<PPU> { panic!("Access to `PPU` is prohibited") }
    fn ppu_mut(&self) -> RefMut<PPU> { panic!("Access to `PPU` is prohibited") }
    fn rom(&self) -> Ref<Rom> { panic!("Access to `Rom` is prohibited") }
    fn rom_mut(&self) -> RefMut<Rom> { panic!("Access to `Rom` is prohibited") }
}

pub fn bytes_to_u16(bytes: &[u8; 2]) -> u16 {
    ((bytes[1] as u16) << 8) | (bytes[0] as u16)
}

pub fn vec_to_u16(bytes: &Vec<u8>) -> u16 {
    let bytes: [u8; 2] = bytes.to_owned().try_into().unwrap();
    ((bytes[1] as u16) << 8) | (bytes[0] as u16)
}

pub fn page_cross(addr1: u16, addr2: u16) -> bool {
    addr1 & 0xFF00 != addr2 & 0xFF00
}
