use crate::bus::{Bus, Mem};
use crate::cpu::instruction::{self, Instruction, AddressingMode};
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

                let ptr: u8 = (base as u8).wrapping_add(self.index_x);
                let lo: u8 = self.mem_read(ptr as u16);
                let hi: u8 = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base: u8 = self.mem_read(self.program_counter);

                let lo: u8 = self.mem_read(base as u16);
                let hi: u8 = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base: u16 = (hi as u16) << 8 | (lo as u16);
                let deref: u16 = deref_base.wrapping_add(self.index_y as u16);
                deref
            }

            _ => {
                panic!("Addressing Mode {:?} is not supported!", mode);
            }
        }
    }

    pub fn step(&mut self) {
        let opcode: u8 = self.mem_read(self.program_counter);
        self.program_counter += 1;
        let program_counter_state: u16 = self.program_counter;
        let (instruct, addr_mode, opcode_len) = instruction::decode_opcode(opcode);

        match instruct {
            Instruction::LDA => todo!(),
            Instruction::LDX => todo!(),
            Instruction::LDY => todo!(),
            Instruction::STA => todo!(),
            Instruction::STX => todo!(),
            Instruction::STY => todo!(),
            Instruction::TAX => todo!(),
            Instruction::TAY => todo!(),
            Instruction::TXA => todo!(),
            Instruction::TYA => todo!(),
            Instruction::TSX => todo!(),
            Instruction::TXS => todo!(),
            Instruction::PHA => todo!(),
            Instruction::PHP => todo!(),
            Instruction::PLA => todo!(),
            Instruction::PLP => todo!(),
            Instruction::AND => todo!(),
            Instruction::EOR => todo!(),
            Instruction::ORA => todo!(),
            Instruction::BIT => todo!(),
            Instruction::ADC => todo!(),
            Instruction::SBC => todo!(),
            Instruction::CMP => todo!(),
            Instruction::CPX => todo!(),
            Instruction::CPY => todo!(),
            Instruction::INC => todo!(),
            Instruction::INX => todo!(),
            Instruction::INY => todo!(),
            Instruction::DEC => todo!(),
            Instruction::DEX => todo!(),
            Instruction::DEY => todo!(),
            Instruction::ASL => todo!(),
            Instruction::LSR => todo!(),
            Instruction::ROL => todo!(),
            Instruction::ROR => todo!(),
            Instruction::JMP => todo!(),
            Instruction::JSR => todo!(),
            Instruction::RTS => todo!(),
            Instruction::BCC => todo!(),
            Instruction::BCS => todo!(),
            Instruction::BEQ => todo!(),
            Instruction::BMI => todo!(),
            Instruction::BNE => todo!(),
            Instruction::BPL => todo!(),
            Instruction::BVC => todo!(),
            Instruction::BVS => todo!(),
            Instruction::CLC => todo!(),
            Instruction::CLD => todo!(),
            Instruction::CLI => todo!(),
            Instruction::CLV => todo!(),
            Instruction::SEC => todo!(),
            Instruction::SED => todo!(),
            Instruction::SEI => todo!(),
            Instruction::BRK => todo!(),
            Instruction::NOP => todo!(),
            Instruction::RTI => todo!(),
            Instruction::SLO => todo!(),
            Instruction::RLA => todo!(),
            Instruction::SRE => todo!(),
            Instruction::RRA => todo!(),
            Instruction::SAX => todo!(),
            Instruction::LAX => todo!(),
            Instruction::DCP => todo!(),
            Instruction::ISC => todo!(),
            Instruction::ANC => todo!(),
            Instruction::ALR => todo!(),
            Instruction::ARR => todo!(),
            Instruction::XAA => todo!(),
            Instruction::AXS => todo!(),
            Instruction::SBC_NOP => todo!(),
            Instruction::AHX => todo!(),
            Instruction::SHY => todo!(),
            Instruction::SHX => todo!(),
            Instruction::TAS => todo!(),
            Instruction::LAS => todo!(),
        }

        if program_counter_state == self.program_counter {
            self.program_counter += opcode_len - 1
        }
    }
}
