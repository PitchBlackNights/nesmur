//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | PPU Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|
pub const PRG_ROM_END: u16 = 0xFFFF;
pub const PRG_ROM_BANK_HI_END: u16 = 0xFFFF;
pub const PRG_ROM_BANK_HI: u16 = 0xC000;
pub const PRG_ROM_BANK_LO_END: u16 = 0xBFFF;
pub const PRG_ROM_BANK_LO: u16 = 0x8000;
pub const PRG_ROM: u16 = 0x8000;

pub const SRAM_END: u16 = 0x7FFF;
pub const SRAM: u16 = 0x6000;

pub const EXPANSION_ROM_END: u16 = 0x5FFF;
pub const EXPANSION_ROM: u16 = 0x4020;

pub const MMIO_JOY2: u16 = 0x4017;
pub const MMIO_JOY1: u16 = 0x4016;
pub const APU_REGISTERS_END: u16 = 0x4015;
pub const MMIO_SND_CHN: u16 = 0x4015;
pub const MMIO_OAMDMA: u16 = 0x4014;
pub const MMIO_DMC_LEN: u16 = 0x4013;
pub const MMIO_DMC_START: u16 = 0x4012;
pub const MMIO_DMC_RAW: u16 = 0x4011;
pub const MMIO_DMC_FREQ: u16 = 0x4010;
pub const MMIO_NOISE_HI: u16 = 0x400F;
pub const MMIO_NOISE_LO: u16 = 0x400E;
pub const MMIO_NOISE_VOL: u16 = 0x400C;
pub const MMIO_TRI_HI: u16 = 0x400B;
pub const MMIO_TRI_LO: u16 = 0x400A;
pub const MMIO_TRI_LINEAR: u16 = 0x4008;
pub const MMIO_SQ2_HI: u16 = 0x4007;
pub const MMIO_SQ2_LO: u16 = 0x4006;
pub const MMIO_SQ2_SWEEP: u16 = 0x4005;
pub const MMIO_SQ2_VOL: u16 = 0x4004;
pub const MMIO_SQ1_HI: u16 = 0x4003;
pub const MMIO_SQ1_LO: u16 = 0x4002;
pub const MMIO_SQ1_SWEEP: u16 = 0x4001;
pub const MMIO_SQ1_VOL: u16 = 0x4000;
pub const APU_REGISTERS: u16 = 0x4000;

pub const PPU_REGISTERS_END: u16 = 0x3FFF;
pub const PPU_REGISTERS_MIRROR: u16 = 0x2008;
pub const MMIO_PPUDATA: u16 = 0x2007;
pub const MMIO_PPUADDR: u16 = 0x2006;
pub const MMIO_PPUSCROLL: u16 = 0x2005;
pub const MMIO_OAMDATA: u16 = 0x2004;
pub const MMIO_OAMADDR: u16 = 0x2003;
pub const MMIO_PPUSTATUS: u16 = 0x2002;
pub const MMIO_PPUMASK: u16 = 0x2001;
pub const MMIO_PPUCTRL: u16 = 0x2000;
pub const PPU_REGISTERS: u16 = 0x2000;

pub const RAM_END: u16 = 0x1FFF;
pub const RAM: u16 = 0x0000;
