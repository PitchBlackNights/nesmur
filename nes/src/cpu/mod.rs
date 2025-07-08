pub mod addr;
pub mod instr;
pub mod opcode;

use crate::bus::{Bus, Mem};
// use crate::cpu::addr::AddressingMode;
use crate::cpu::instr::execute_instruction;
use crate::cpu::opcode::OpCode;
// use crate::prelude::*;
use bitflags::bitflags;

bitflags! {
    /// Status Register (P) - http://wiki.nesdev.com/w/index.php/Status_flags
    /// ```plaintext
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +---- Carry
    ///  | |   | | | +------ Zero
    ///  | |   | | +-------- Interrupt Disable
    ///  | |   | +---------- Decimal Mode (not used on NES)
    ///  | |   +------------ Break
    ///  | +---------------- Overflow
    ///  +------------------ Negative
    /// ```
    #[derive(Clone, Debug)]
    pub struct Flags: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAK             = 0b0001_0000;
        const UNUSED            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIVE          = 0b1000_0000;
    }
}

const _STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

pub struct CPU {
    pub cycles: u64,
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status: Flags,
    pub bus: Bus,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }
    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

impl CPU {
    pub fn new() -> Self {
        // Hack to build OPCODES hashmap now instead of in `cpu::step()`
        let _ = &opcode::OPCODES.get(&0u8);

        CPU {
            cycles: 0,
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            stack_pointer: STACK_RESET,
            program_counter: 0x0000,
            status: Flags::from_bits_truncate(0b0010_0100),
            bus: Bus::new(),
        }
    }

    pub fn step(&mut self) {
        let code: u8 = self.mem_read(self.program_counter);
        self.program_counter += 1;
        let program_counter_state: u16 = self.program_counter;
        let opcode: &'static OpCode = opcode::decode_opcode(code);

        let mut operands: Vec<u8> = Vec::new();
        for i in 1..opcode.len {
            operands.push(self.mem_read(self.program_counter + i as u16 - 1));
        }
        let cycles: u64 = execute_instruction(self, opcode, operands);
        self.cycles += cycles;
        // TODO: Tick bus n cycles

        if program_counter_state == self.program_counter {
            self.program_counter += opcode.len as u16 - 1
        }
    }
}
