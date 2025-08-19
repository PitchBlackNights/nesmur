use crate::tools::BitPlane;

//  F E D C B A 9 8 __ 7 6 5 4 3 2 1 0
//    | | | | | | |    +-+-+-+-+-+-+-+---- Lo Byte
//    | +-+-+-+-+-+----------------------- Hi Byte
//    | +-+-+-+-+-+----+-+-+-+-+-+-+-+---- Address
//    | | | | | | |    | | | +-+-+-+-+---- Coarse X
//    | | | | | +-+----+-+-+-------------- Coarse Y
//    | | | +-+--------------------------- Nametable
//    +-+-+------------------------------- Fine Y
#[derive(Clone, Copy)]
pub struct AddrRegister {
    value: BitPlane<u8>,
}

impl AddrRegister {
    pub fn new() -> Self {
        // https://www.nesdev.org/wiki/PPU_power_up_state
        AddrRegister {
            value: BitPlane::new(0x00, 0x00), // Hi byte first, Lo byte second
        }
    }

    fn set(&mut self, data: u16) {
        self.value.hi = (data >> 8) as u8;
        self.value.lo = (data & 0b0000_0000_1111_1111) as u8;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo: u8 = self.value.lo;
        self.value.lo = self.value.lo.wrapping_add(inc);
        if lo > self.value.lo {
            self.value.hi = self.value.hi.wrapping_add(1);
        }
    }

    pub fn get(&self) -> u16 {
        // Restricts to 0x0000-0x3FFF
        (self.get_all()) & 0b0011_1111_1111_1111
    }

    pub fn get_all(&self) -> u16 {
        ((self.value.hi as u16) << 8) | (self.value.lo as u16)
    }

    pub fn get_hi_byte(&self) -> u8 {
        self.value.hi
    }

    pub fn set_hi_byte(&mut self, value: u8) {
        self.value.hi = value
    }

    pub fn get_lo_byte(&self) -> u8 {
        self.value.lo
    }

    pub fn set_lo_byte(&mut self, value: u8) {
        self.value.lo = value
    }

    pub fn get_nametable(&self) -> u8 {
        ((self.get_all() & 0b0000_1100_0000_0000) >> 10) as u8
    }

    pub fn set_nametable(&mut self, value: u8) {
        self.set(
            (self.get_all() & 0b1111_0011_1111_1111)
                | (((value as u16) << 10) & 0b0000_1100_0000_0000),
        );
    }

    pub fn get_coarse_x(&self) -> u8 {
        (self.get_all() & 0b0000_0000_0001_1111) as u8
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.set((self.get_all() & 0b1111_1111_1110_0000) | (value as u16 & 0b0000_0000_0001_1111));
    }

    pub fn get_coarse_y(&self) -> u8 {
        ((self.get_all() & 0b0000_0011_1110_0000) >> 5) as u8
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.set(
            (self.get_all() & 0b1111_1100_0001_1111)
                | (((value as u16) << 5) & 0b0000_0011_1110_0000),
        );
    }

    pub fn get_fine_y(&self) -> u8 {
        ((self.get_all() & 0b0111_0000_0000_0000) >> 12) as u8
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.set(
            (self.get_all() & 0b1000_1111_1111_1111)
                | (((value as u16) << 12) & 0b0111_0000_0000_0000),
        );
    }

    pub fn get_nametable_addr(&self) -> u16 {
        // Removes Fine Y from address, since is used for inner-tile scrolling
        (self.get_all() & 0b0000_1111_1111_1111) | 0b0010_0000_0000_0000
    }

    pub fn get_attribute_addr(&self) -> u16 {
        let nametable: u16 = self.get_nametable() as u16;
        let coarse_y: u16 = self.get_coarse_y() as u16;
        let coarse_x: u16 = self.get_coarse_x() as u16;
        // First find the base address of the attribute table then offset
        // by the x and y tiles. Each Attribute table entry covers 4 tiles.
        (nametable << 10) | ((coarse_y / 4) << 3) | (coarse_x / 4) | 0b0010_0011_1100_0000
    }

    pub fn get_tile_offset(&self, tile_number: u8) -> u16 {
        ((tile_number as u16) << 4) | self.get_fine_y() as u16
    }

    pub fn copy_x(&mut self, other: AddrRegister) {
        self.set(
            (self.get_all() & 0b1111_1011_1110_0000) | (other.get_all() & 0b0000_0100_0001_1111),
        )
    }

    pub fn copy_y(&mut self, other: AddrRegister) {
        self.set(
            (self.get_all() & 0b1000_0100_0001_1111) | (other.get_all() & 0b0111_1011_1110_0000),
        )
    }

    pub fn scroll_x(&mut self) {
        if self.get_coarse_x() == 0x1F {
            self.set_coarse_x(0x00);
            self.set(self.get_all() ^ 0b0000_0100_0000_0000)
        } else {
            let coarse_x: u8 = self.get_coarse_x();
            self.set_coarse_x(coarse_x + 1);
        }
    }

    pub fn scroll_y(&mut self) {
        let fine_y: u8 = self.get_fine_y();

        if fine_y < 7 {
            self.set_fine_y(fine_y + 1);
        } else {
            self.set_fine_y(0x00);
            let coarse_y: u8 = self.get_coarse_y();

            if coarse_y == 29 {
                // The last row of tiles
                self.set_coarse_y(0x00);
                self.set(self.get_all() ^ 0b0000_1000_0000_0000);
            } else if coarse_y == 31 {
                // Values 29-31 are out-of-bounds but used by some games to implement a weird reverse scroll
                self.set_coarse_y(0);
            } else {
                self.set_coarse_y(coarse_y + 1);
            }
        }
    }
}
