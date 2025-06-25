use crate::prelude::*;

// 0x0000-0x00FF ::  256  :: Zero Page
// 0x0100-0x01FF ::  256  :: Stack
// 0x0200-0xFFF9 :: 65018 :: User Memory
// 0xFFFA-0xFFFB ::   2   :: NMI Handler Address
// 0xFFFC-0xFFFD ::   2   :: Reset Handler Address
// 0xFFFE-0xFFFF ::   2   :: Interrupt Handler Address

const MEMORY_SIZE: usize = 0xFFFF + 1;

#[derive(Copy, Clone, Debug)]
pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let byte: u8 = self.data[address as usize];
        trace!("[Read] {:02x} from {:04x}", byte, address);
        byte
    }

    pub fn read_n_bytes(&self, base_address: u16, num_bytes: u16) -> Vec<u8> {
        let mut num_bytes: u16 = num_bytes - 1;
        if base_address as usize + num_bytes as usize > 0xFFFF {
            warn!("[Read] Attempted to read bytes outside memory. Truncating to {} bytes read", 0xFFFF - base_address);
            num_bytes = 0xFFFF - base_address;
        }

        let mut buffer: Vec<u8> = Vec::new();
        for address in base_address..base_address + num_bytes {
            buffer.push(self.data[address as usize]);
        }

        trace!("[Read] {} bytes from {:04x}", num_bytes, base_address);
        buffer
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.data[address as usize] = byte;
        trace!("[Write] {:02x} to {:04x}", byte, address);
    }

    pub fn write_n_bytes(&mut self, base_address: u16, bytes: Vec<u8>) {
        let mut bytes: Vec<u8> = bytes;
        if base_address as usize + bytes.len() > 0xFFFF {
            warn!("[Write] Attempted to write bytes outside memory. Truncating to {} bytes wrote", 0xFFFF - base_address);
            bytes.truncate(0xFFFF - base_address as usize);
        }

        for address in base_address..base_address + bytes.len() as u16 {
            self.data[address as usize] = bytes[address as usize - base_address as usize]
        }
        trace!("[Write] {} bytes to {:04x}", bytes.len(), base_address);
    }

    pub fn modify<F>(&mut self, address: u16, function: F)
    where
        F: Fn(u8) -> u8,
    {
        self.data[address as usize] = function(self.data[address as usize])
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
