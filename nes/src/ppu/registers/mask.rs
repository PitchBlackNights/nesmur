use crate::prelude::*;

bitflags! {
    /// ```plaintext
    /// 7  bit  0
    /// ---- ----
    /// BGRs bMmG
    /// ││││ ││││
    /// ││││ │││╘═ Greyscale
    /// ││││ ││╘══ Show background in leftmost 8 pixels of screen
    /// ││││ │╘═══ Show sprites in leftmost 8 pixels of screen
    /// ││││ ╘════ Show background
    /// │││╘══════ Show sprites
    /// ││╘═══════ Emphasize red
    /// │╘════════ Emphasize green
    /// ╘═════════ Emphasize blue
    /// ```
    pub struct MaskRegister: u8 {
        const GREYSCALE                = 0b0000_0001;
        const LEFTMOST_8PXL_BACKGROUND = 0b0000_0010;
        const LEFTMOST_8PXL_SPRITE     = 0b0000_0100;
        const SHOW_BACKGROUND          = 0b0000_1000;
        const SHOW_SPRITES             = 0b0001_0000;
        const EMPHASISE_RED            = 0b0010_0000;
        const EMPHASISE_GREEN          = 0b0100_0000;
        const EMPHASISE_BLUE           = 0b1000_0000;
    }
}

// ==== UNUSED ====
// pub enum Color {
//     Red,
//     Green,
//     Blue,
// }

impl MaskRegister {
    pub fn new() -> Self {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        MaskRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn reset(&mut self) {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        self.remove(MaskRegister::GREYSCALE);
        self.remove(MaskRegister::LEFTMOST_8PXL_BACKGROUND);
        self.remove(MaskRegister::LEFTMOST_8PXL_SPRITE);
        self.remove(MaskRegister::SHOW_BACKGROUND);
        self.remove(MaskRegister::SHOW_SPRITES);
        self.remove(MaskRegister::EMPHASISE_RED);
        self.remove(MaskRegister::EMPHASISE_GREEN);
        self.remove(MaskRegister::EMPHASISE_BLUE);
    }

    pub fn is_grayscale(&self) -> bool {
        self.contains(MaskRegister::GREYSCALE)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        self.contains(MaskRegister::LEFTMOST_8PXL_BACKGROUND)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        self.contains(MaskRegister::LEFTMOST_8PXL_SPRITE)
    }

    pub fn show_background(&self) -> bool {
        self.contains(MaskRegister::SHOW_BACKGROUND)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskRegister::SHOW_SPRITES)
    }

    // ==== UNUSED ====
    // pub fn emphasise(&self) -> Vec<Color> {
    //     let mut result: Vec<Color> = Vec::<Color>::new();
    //     if self.contains(MaskRegister::EMPHASISE_RED) {
    //         result.push(Color::Red);
    //     }
    //     if self.contains(MaskRegister::EMPHASISE_BLUE) {
    //         result.push(Color::Blue);
    //     }
    //     if self.contains(MaskRegister::EMPHASISE_GREEN) {
    //         result.push(Color::Green);
    //     }
    //     result
    // }

    pub fn rendering(&self) -> bool {
        self.show_sprites() || self.show_background()
    }

    pub fn rendering_background(&self, x: usize) -> bool {
        self.show_background() && (self.leftmost_8pxl_background() || x >= 8)
    }

    pub fn rendering_sprites(&self, x: usize) -> bool {
        self.show_sprites() && (self.leftmost_8pxl_sprite() || x >= 8)
    }
}
