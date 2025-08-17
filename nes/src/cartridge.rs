use crate::prelude::*;

pub const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_ROM_PAGE_SIZE: usize = 16_384; // 16 KiB
pub const CHR_ROM_PAGE_SIZE: usize = 8_192; // 8 KiB

macro_rules! opt_debug {
    ($fmt:literal, $($arg:expr),+ $(,)?) => {{
        if true $( && $arg.is_some() )+ {
            log::log!(log::Level::Debug, $fmt, $($arg.as_ref().unwrap()),+);
        }
    }};
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mirroring {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum ROMRegion {
    None,
    NTSC,
    PAL,
    Dual,
}

pub struct Rom {
    pub ines_ver: u8,
    pub mapper: u16,
    pub submapper: u8,
    pub region: ROMRegion,
    pub prg_rom: Vec<u8>,
    pub prg_ram_size: usize,
    pub prg_nvram_size: usize,
    pub chr_rom: Vec<u8>,
    pub chr_ram_size: usize,
    pub chr_nvram_size: usize,
    pub screen_mirroring: Mirroring,
    pub uses_bat_mem: bool,
}

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Rom, String> {
        if raw[0..4] != NES_TAG {
            return Err("File is not in iNES file format".to_string());
        }
        let ines_ver: u8 = if (raw[7] >> 2) & 0b0000_0011 == 0 { 1 } else { 2 };
        
        let uses_bat_mem: bool = raw[6] & 0b0000_0010 != 0;
        let uses_trainer: bool = raw[6] & 0b0000_0100 != 0;
        let alt_nametable_layout: bool = raw[6] & 0b0000_1000 != 0;
        let vertical_mirroring: bool = raw[6] & 0b0000_0001 != 0;
        let screen_mirroring: Mirroring = match (alt_nametable_layout, vertical_mirroring) {
            (true, _) => return Err("This ROM uses the mapper specific \"Alternative Nametable Layout\" feature!".to_string()),
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };
        
        fn format_byte_size_option(bytes_opt: Option<usize>) -> Option<String> {
            match bytes_opt {
                Some(bytes) => Some(tools::format_byte_size(bytes)),
                None => None,
            }
        }
        
        if ines_ver == 1 {
            match raw[7] & 0b0000_0011 {
                0x0 => {},
                0x1 => return Err("The Nintendo VS. System is not supported!".to_string()),
                0x2 => return Err("The Nintendo Playchoice 10 is not supported!".to_string()),
                _ => return Err("ROM provides invalid console type (0x3) for iNES 1.0 format!".to_string()),
            };
        
            let mut mapper: u16 = ((raw[7] & 0b1111_0000) | (raw[6] >> 4)) as u16;
    
            let prg_rom_size: usize = raw[4] as usize * PRG_ROM_PAGE_SIZE;
            let prg_rom_start: usize = 16 + if uses_trainer { 512 } else { 0 };
            let prg_rom: Vec<u8> = raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec();
            
            let chr_rom_size: usize = raw[5] as usize * CHR_ROM_PAGE_SIZE;
            let chr_rom_start: usize = prg_rom_start + prg_rom_size;
            let chr_rom: Vec<u8> = raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec();
            let uses_chr_ram: bool = chr_rom_size == 0;
            
            // Optional ROM features
            let mut prg_ram_size: Option<usize> = None;
            let mut region: Option<ROMRegion> = None;
            
            // https://www.nesdev.org/wiki/INES#Flags_10
            if &raw[11..=15] != [0x00, 0x00, 0x00, 0x00, 0x00] {
                mapper &= 0b0000_1111;
            } else {
                prg_ram_size = if raw[10] & 0b0001_0000 != 0 { Some(raw[8] as usize * 8192) } else { None };
                
                let region_def_1: u8 = raw[9] & 0b0000_0001;
                let region_def_2: u8 = raw[10] & 0b0000_0011;
                region = match (region_def_1, region_def_2) {
                    (0, 0) => Some(ROMRegion::NTSC),
                    (1, 0) | (0, 2) | (1, 2) => Some(ROMRegion::PAL),
                    (_, 1) | (_, 3) => Some(ROMRegion::Dual),
                    (_, _) => panic!("This shouldn't happen!"),
                };
            }
            
            debug!("Loaded ROM Info:");
            debug!("  iNES Version: {}", ines_ver);
            debug!("  Mapper: {}", mapper);
            opt_debug!("  Region: {:?}", region);
            debug!("  PRG-ROM Size: {}", tools::format_byte_size(prg_rom_size));
            opt_debug!("  PRG-RAM Size: {}", format_byte_size_option(prg_ram_size));
            debug!("  CHR-ROM Size: {}", tools::format_byte_size(chr_rom_size));
            debug!("  CHR-RAM?: {}", uses_chr_ram);
            debug!("  Mirroring: {:?}", screen_mirroring);
            debug!("  Trainer?: {}", uses_trainer);
            debug!("  Battery Mem?: {}", uses_bat_mem);
            
            Ok(Rom {
                ines_ver,
                mapper,
                submapper: 0,
                region: region.unwrap_or(ROMRegion::None),
                prg_rom,
                prg_ram_size: prg_ram_size.unwrap_or(0),
                prg_nvram_size: 0,
                chr_rom,
                chr_ram_size: if uses_chr_ram { CHR_ROM_PAGE_SIZE } else { 0 },
                chr_nvram_size: 0,
                screen_mirroring,
                uses_bat_mem,
            })
        } else {
            // ines_ver == 2
            match raw[7] & 0b0000_0011 {
                0x0 => {},
                0x1 => return Err("The Nintendo VS. System is not supported!".to_string()),
                0x2 => return Err("The Nintendo Playchoice 10 is not supported!".to_string()),
                0x3 => {
                    match raw[13] & 0b0000_1111 {
                        0x0 => {},
                        0x1 => return Err("The Nintendo VS. System is not supported!".to_string()),
                        0x2 => return Err("The Nintendo Playchoice 10 is not supported!".to_string()),
                        0x3 => return Err("Famiclones with Decimal Mode support are not supported!".to_string()),
                        0x4 => return Err("The EPSM module and/or Plug-through cartridges are not supported!".to_string()),
                        0x5 => return Err("The V.R. Technology VT01 is not supported!".to_string()),
                        0x6 => return Err("The V.R. Technology VT02 is not supported!".to_string()),
                        0x7 => return Err("The V.R. Technology VT03 is not supported!".to_string()),
                        0x8 => return Err("The V.R. Technology VT09 is not supported!".to_string()),
                        0x9 => return Err("The V.R. Technology VT32 is not supported!".to_string()),
                        0xA => return Err("The V.R. Technology VT365 is not supported!".to_string()),
                        0xB => return Err("The UMC UM6578 is not supported!".to_string()),
                        0xC => return Err("The Famicom Network System is not supported!".to_string()),
                        _ => return Err(format!("ROM provides invalid extended console type ({:#3X}) for iNES 2.0 format!", raw[13] & 0b0000_1111)),
                    }
                },
                _ => panic!("This shouldn't happen!"),
            };
            if raw[14] & 0b0000_0011 != 0 {
                return Err("iNES 2.0 bundled \"Miscellaneous\" ROMs are not supported!".to_string());
            }
            if raw[15] & 0b0011_1111 != 0x01 {
                return Err("ROM uses unsupported input device(s)!".to_string());
            }
            
            let mapper: u16 = ((raw[8] as u16 & 0b0000_1111) << 8) | (raw[7] & 0b1111_0000) as u16 | (raw[6] >> 4) as u16;
            let submapper: u8 = raw[8] >> 4;
            
            let prg_rom_size: usize = match raw[9] & 0b0000_1111 {
                0x0..0xF => (((raw[9] as usize & 0b0000_1111) << 8) | raw[4] as usize) * PRG_ROM_PAGE_SIZE,
                0xF => {
                    let multiplier: u8 = raw[4] & 0b0000_0011;
                    let exponent: u8 = raw[4] >> 2;
                    2u32.saturating_pow(exponent as u32).saturating_mul(multiplier as u32 * 2 + 1) as usize
                }
                _ => panic!("This shouldn't happen!"),
            };
            let prg_rom_start: usize = 16 + if uses_trainer { 512 } else { 0 };
            let prg_rom: Vec<u8> = raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec();
            let prg_ram_size: Option<usize> = match raw[10] & 0b0000_1111 {
                0 => None,
                _ => Some(64usize << (raw[10] & 0b0000_1111) as usize),
            };
            let prg_nvram_size: Option<usize> = match raw[10] >> 4 {
                0 => None,
                _ => Some(64usize << (raw[10] >> 4) as usize),
            };
            
            let chr_rom_size: usize = match raw[9] >> 4 {
                0x0..0xF => (((raw[9] as usize >> 4) << 8) | raw[5] as usize) * PRG_ROM_PAGE_SIZE,
                0xF => {
                    let multiplier: u8 = raw[5] & 0b0000_0011;
                    let exponent: u8 = raw[5] >> 2;
                    2u32.saturating_pow(exponent as u32).saturating_mul(multiplier as u32 * 2 + 1) as usize
                }
                _ => panic!("This shouldn't happen!"),
            };
            let chr_rom_start: usize = prg_rom_start + prg_rom_size;
            let chr_rom: Vec<u8> = raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec();
            let chr_ram_size: Option<usize> = match raw[11] & 0b0000_1111 {
                0 => None,
                _ => Some(64usize << (raw[11] & 0b0000_1111) as usize),
            };
            let chr_nvram_size: Option<usize> = match raw[11] >> 4 {
                0 => None,
                _ => Some(64usize << (raw[11] >> 4) as usize),
            };
            
            let region: ROMRegion = match raw[12] & 0b0000_0011 {
                0 => ROMRegion::NTSC,
                1 => ROMRegion::PAL,
                2 => ROMRegion::Dual,
                _ => return Err("The \"Dendy\" famiclone is not supported!".to_string()),
            };
            
            debug!("Loaded ROM Info:");
            debug!("  iNES Version: {}", ines_ver);
            debug!("  Mapper: {}", mapper);
            debug!("  Submapper: {}", submapper);
            debug!("  Region: {:?}", region);
            debug!("  PRG-ROM Size: {}", tools::format_byte_size(prg_rom_size));
            opt_debug!("  PRG-RAM Size: {}", format_byte_size_option(prg_ram_size));
            opt_debug!("  PRG-NVRAM Size: {}", format_byte_size_option(prg_nvram_size));
            debug!("  CHR-ROM Size: {}", tools::format_byte_size(chr_rom_size));
            opt_debug!("  CHR-RAM Size: {}", format_byte_size_option(chr_ram_size));
            opt_debug!("  CHR-NVRAM Size: {}", format_byte_size_option(chr_nvram_size));
            debug!("  Mirroring: {:?}", screen_mirroring);
            debug!("  Trainer?: {}", uses_trainer);
            debug!("  Battery Mem?: {}", uses_bat_mem);
            
            Ok(Rom {
                ines_ver,
                mapper,
                submapper,
                region,
                prg_rom,
                prg_ram_size: prg_ram_size.unwrap_or(0),
                prg_nvram_size: prg_nvram_size.unwrap_or(0),
                chr_rom,
                chr_ram_size: chr_ram_size.unwrap_or(0),
                chr_nvram_size: chr_nvram_size.unwrap_or(0),
                screen_mirroring,
                uses_bat_mem,
            })
        }
    }
}
