pub mod instruction;

use crate::bus::{Bus, Mem};
use crate::cpu::instruction::Instruction::*;
use crate::cpu::instruction::{AddressingMode, OpCode};
use crate::prelude::*;
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

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

pub struct CPU {
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
        let _ = &instruction::OPCODES.get(&0u8);

        CPU {
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            stack_pointer: STACK_RESET,
            program_counter: 0x0000,
            status: Flags::from_bits_truncate(0b0010_0100),
            bus: Bus::new(),
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos: u8 = self.mem_read(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.index_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos: u8 = self.mem_read(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.index_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base: u16 = self.mem_read_u16(self.program_counter);
                let addr: u16 = base.wrapping_add(self.index_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base: u16 = self.mem_read_u16(self.program_counter);
                let addr: u16 = base.wrapping_add(self.index_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base: u8 = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.index_x);
                let lo: u8 = self.mem_read(ptr as u16);
                let hi: u8 = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base: u8 = self.mem_read(self.program_counter);

                let lo: u8 = self.mem_read(base as u16);
                let hi: u8 = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base: u16 = (hi as u16) << 8 | (lo as u16);
                let deref: u16 = deref_base.wrapping_add(self.index_y as u16);
                deref
            }

            _ => {
                panic!("Addressing Mode {mode:?} is not supported!");
            }
        }
    }

    pub fn step(&mut self) {
        let code: u8 = self.mem_read(self.program_counter);
        self.program_counter += 1;
        let program_counter_state: u16 = self.program_counter;
        let opcode: &'static OpCode = instruction::decode_opcode(code);

        debug!("==== Executing Operation ====");
        debug!("  Byte: {:#04X},", opcode.byte);
        debug!("  Instruction: {:?},", opcode.instruction);
        debug!("  Mnemonic: \"{}\"", opcode.mnemonic);
        debug!("  Len: {}", opcode.len);
        debug!("  Mode: {:?}", opcode.mode);

        match opcode.instruction {
            LDA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            LDX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            LDY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            STA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            STX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            STY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TAX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TAY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TXA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TYA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TSX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TXS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            PHA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            PHP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            PLA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            PLP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            AND => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            EOR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ORA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BIT => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ADC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SBC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CMP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CPX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CPY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            INC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            INX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            INY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            DEC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            DEX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            DEY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ASL => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            LSR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ROL => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ROR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            JMP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            JSR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            RTS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BCC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BCS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BEQ => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BMI => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BNE => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BPL => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BVC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BVS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CLC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CLD => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CLI => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            CLV => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SEC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SED => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SEI => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            BRK => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            NOP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            RTI => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            NOP_ALT => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SLO => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            RLA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SRE => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            RRA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SAX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            LAX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            DCP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ISC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ANC => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ALR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            ARR => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            XAA => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            AXS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SBC_NOP => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            AHX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SHY => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            SHX => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            TAS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
            LAS => panic!(
                "CPU Operation '{:?}' is not implemented",
                opcode.instruction
            ),
        }

        if program_counter_state == self.program_counter {
            self.program_counter += opcode.len as u16 - 1
        }
    }
}
