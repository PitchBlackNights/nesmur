use crate::prelude::*;

bitflags! {
    /// ```plaintext
    /// 7  bit  0
    /// ---- ----
    /// VPHB SINN
    /// ││││ ││││
    /// ││││ ││╘╧═ Base nametable address
    /// ││││ ││    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    /// ││││ │╘═══ VRAM address increment per CPU read/write of PPUDATA
    /// ││││ │     (0: add 1, going across; 1: add 32, going down)
    /// ││││ ╘════ Sprite pattern table address for 8x8 sprites
    /// ││││       (0: $0000; 1: $1000; ignored in 8x16 mode)
    /// │││╘══════ Background pattern table address (0: $0000; 1: $1000)
    /// ││╘═══════ Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    /// │╘════════ PPU master/slave select
    /// │          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    /// ╘═════════ Generate an NMI at the start of the
    ///            vertical blanking interval (0: off; 1: on)
    /// ```
    #[derive(Clone, Copy)]
    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b0000_0001;
        const NAMETABLE2              = 0b0000_0010;
        const VRAM_ADD_INCREMENT      = 0b0000_0100;
        const SPRITE_PATTERN_ADDR     = 0b0000_1000;
        const BACKROUND_PATTERN_ADDR  = 0b0001_0000;
        const SPRITE_SIZE             = 0b0010_0000;
        const MASTER_SLAVE_SELECT     = 0b0100_0000;
        const GENERATE_NMI            = 0b1000_0000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        ControlRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn reset(&mut self) {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        self.remove(ControlRegister::NAMETABLE1);
        self.remove(ControlRegister::NAMETABLE2);
        self.remove(ControlRegister::VRAM_ADD_INCREMENT);
        self.remove(ControlRegister::SPRITE_PATTERN_ADDR);
        self.remove(ControlRegister::BACKROUND_PATTERN_ADDR);
        self.remove(ControlRegister::SPRITE_SIZE);
        self.remove(ControlRegister::MASTER_SLAVE_SELECT);
        self.remove(ControlRegister::GENERATE_NMI);
    }

    pub fn get_nametable(&self) -> u8 {
        self.bits() & 0b0000_0011
    }

    pub fn nametable_addr(&self) -> u16 {
        match self.get_nametable() {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("not possible"),
        }
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    pub fn background_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::BACKROUND_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !self.contains(ControlRegister::SPRITE_SIZE) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if !self.contains(ControlRegister::MASTER_SLAVE_SELECT) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }
}
