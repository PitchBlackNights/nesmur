use crate::cpu::CPU;
use crate::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OpCode {
    pub byte: u8,
    pub instruction: Instruction,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: usize,
    pub mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        byte: u8,
        instruction: Instruction,
        mnemonic: &'static str,
        cycles: usize,
        mode: AddressingMode,
    ) -> Self {
        let len: u8 = match mode {
            AddressingMode::Implicit => 1,
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPage_X => 2,
            AddressingMode::ZeroPage_Y => 2,
            AddressingMode::Relative => 2,
            AddressingMode::Absolute => 3,
            AddressingMode::Absolute_X => 3,
            AddressingMode::Absolute_Y => 3,
            AddressingMode::Indirect => 3,
            AddressingMode::Indirect_X => 2,
            AddressingMode::Indirect_Y => 2,
        };

        Self {
            byte,
            instruction,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }

    pub fn get_operand_address(self, cpu: &CPU) -> (u16, bool) {
        match self.mode {
            AddressingMode::Immediate => (cpu.program_counter, false),
            _ => self.get_absolute_address(cpu, cpu.program_counter),
        }
    }

    pub fn get_absolute_address(self, cpu: &CPU, addr: u16) -> (u16, bool) {
        match self.mode {
            AddressingMode::ZeroPage => (cpu.bus_mut().read(addr) as u16, false),
            AddressingMode::ZeroPage_X => {
                let pos: u8 = cpu.bus_mut().read(addr);
                let addr: u16 = pos.wrapping_add(cpu.index_x) as u16;
                (addr, false)
            }
            AddressingMode::ZeroPage_Y => {
                let pos: u8 = cpu.bus_mut().read(addr);
                let addr: u16 = pos.wrapping_add(cpu.index_y) as u16;
                (addr, false)
            }
            AddressingMode::Relative => {
                let jump: i8 = cpu.bus_mut().read(addr) as i8;
                let jump_addr: u16 = addr.wrapping_add(1).wrapping_add(jump as u16);
                (
                    jump_addr,
                    tools::page_cross(addr.wrapping_add(1), jump_addr),
                )
            }
            AddressingMode::Absolute => (cpu.bus_mut().read_u16(addr), false),
            AddressingMode::Absolute_X => {
                let base: u16 = cpu.bus_mut().read_u16(addr);
                let addr: u16 = base.wrapping_add(cpu.index_x as u16);
                (addr, tools::page_cross(base, addr))
            }
            AddressingMode::Absolute_Y => {
                let base: u16 = cpu.bus_mut().read_u16(addr);
                let addr: u16 = base.wrapping_add(cpu.index_y as u16);
                (addr, tools::page_cross(base, addr))
            }
            AddressingMode::Indirect => {
                // JMP ($xxyy), or JMP indirect, does not advance pages if the
                // lower eight bits of the specified address is $FF; the upper
                // eight bits are fetched from $xx00, 255 bytes earlier,
                // instead of the expected following byte.
                let base: u16 = cpu.bus_mut().read_u16(addr);
                let addr: u16 = if base & 0x00FF == 0x00FF {
                    tools::bytes_to_u16(&[cpu.bus_mut().read(base), cpu.bus_mut().read(base & 0xFF00)])
                } else {
                    cpu.bus_mut().read_u16(base)
                };
                (addr, false)
            }
            AddressingMode::Indirect_X => {
                let base: u8 = cpu.bus_mut().read(addr);
                let ptr: u8 = base.wrapping_add(cpu.index_x);
                let lo: u8 = cpu.bus_mut().read(ptr as u16);
                let hi: u8 = cpu.bus_mut().read(ptr.wrapping_add(1) as u16);
                (tools::bytes_to_u16(&[lo, hi]), false)
            }
            AddressingMode::Indirect_Y => {
                let base: u8 = cpu.bus_mut().read(addr);
                let lo: u8 = cpu.bus_mut().read(base as u16);
                let hi: u8 = cpu.bus_mut().read(base.wrapping_add(1) as u16);
                let deref_base: u16 = tools::bytes_to_u16(&[lo, hi]);
                let deref: u16 = deref_base.wrapping_add(cpu.index_y as u16);
                (deref, tools::page_cross(deref, deref_base))
            }
            _ => {
                panic!(
                    "Getting the operand address for Addressing Mode {:?} is not supported",
                    self
                );
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Relative,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X, // aka IndexedIndirect
    Indirect_Y, // aka IndirectIndexed
}

pub fn decode_opcode(opbyte: u8) -> &'static OpCode {
    OPCODES
        .get(&opbyte)
        .unwrap_or_else(|| panic!("OpCode {opbyte:#04X} is not recognized"))
}

macro_rules! define_opcodes {
    (
        $(
            $( #[$enum_doc:meta] )*
            $instr:ident $( $mnemonic:literal )? {
                $( $opcode:literal => $cycles:literal, $mode:ident ),+ $(,)?
            }
        ),+ $(,)?
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
        pub enum Instruction {
            $(
                $( #[$enum_doc] )*
                $instr,
            )+
        }

        pub static OPCODES: Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
            trace!("Building OPCODES hashmap...");
            let mut map = HashMap::new();
            $(
                let instruction: Instruction = Instruction::$instr;
                let mnemonic: &'static str = define_opcodes!(@mnemonic $instr $( $mnemonic )?);
                $(
                    map.insert(
                        $opcode,
                        OpCode::new($opcode, instruction, mnemonic, $cycles, AddressingMode::$mode)
                    );
                )+
            )+
            trace!("Finished building OPCODES hashmap");
            map
        });
    };

    // Helper for mnemonic: use custom if present, else stringify enum name
    (@mnemonic $instr:ident $mnemonic:literal) => { $mnemonic };
    (@mnemonic $instr:ident) => { stringify!($instr) };
}

define_opcodes!(
    // ===== Load/Store Operations =====
    /// Load Accumulator
    LDA {
        0xA9 => 2, Immediate,
        0xA5 => 3, ZeroPage,
        0xB5 => 4, ZeroPage_X,
        0xAD => 4, Absolute,
        0xBD => 4, Absolute_X,
        0xB9 => 4, Absolute_Y,
        0xA1 => 6, Indirect_X,
        0xB1 => 5, Indirect_Y,
    },
    /// Load X register
    LDX {
        0xA2 => 2, Immediate,
        0xA6 => 3, ZeroPage,
        0xB6 => 4, ZeroPage_Y,
        0xAE => 4, Absolute,
        0xBE => 4, Absolute_Y,
    },
    /// Load Y register
    LDY {
        0xA0 => 2, Immediate,
        0xA4 => 3, ZeroPage,
        0xB4 => 4, ZeroPage_X,
        0xAC => 4, Absolute,
        0xBC => 4, Absolute_X,
    },
    /// Store Accumulator
    STA {
        0x85 => 3, ZeroPage,
        0x95 => 4, ZeroPage_X,
        0x8D => 4, Absolute,
        0x9D => 5, Absolute_X,
        0x99 => 5, Absolute_Y,
        0x81 => 6, Indirect_X,
        0x91 => 6, Indirect_Y,
    },
    /// Store X register
    STX {
        0x86 => 3, ZeroPage,
        0x96 => 4, ZeroPage_Y,
        0x8E => 4, Absolute,
    },
    /// Store Y register
    STY {
        0x84 => 3, ZeroPage,
        0x94 => 4, ZeroPage_X,
        0x8C => 4, Absolute,
    },

    // ===== Register Transfers =====
    /// Transfer Accumulator to X
    TAX {
        0xAA => 2, Implicit,
    },
    /// Transfer Accumulator to Y
    TAY {
        0xA8 => 2, Implicit,
    },
    /// Transfer X to Accumulator
    TXA {
        0x8A => 2, Implicit,
    },
    /// Transfer Y to Accumulator
    TYA {
        0x98 => 2, Implicit,
    },

    // ===== Stack Operations =====
    /// Transfer Stack Pointer to X
    TSX {
        0xBA => 2, Implicit,
    },
    /// Transfer X to Stack Pointer
    TXS {
        0x9A => 2, Implicit,
    },
    /// Push Accumulator on Stack
    PHA {
        0x48 => 3, Implicit,
    },
    /// Push Processor Status on Stack
    PHP {
        0x08 => 3, Implicit,
    },
    /// Pull Accumulator from Stack
    PLA {
        0x68 => 4, Implicit,
    },
    /// Pull Processor Status from Stack
    PLP {
        0x28 => 4, Implicit,
    },

    // ===== Logical =====
    /// Logical AND
    AND {
        0x29 => 2, Immediate,
        0x25 => 3, ZeroPage,
        0x35 => 4, ZeroPage_X,
        0x2D => 4, Absolute,
        0x3D => 4, Absolute_X,
        0x39 => 4, Absolute_Y,
        0x21 => 6, Indirect_X,
        0x31 => 5, Indirect_Y,
    },
    /// Exclusive OR
    EOR {
        0x49 => 2, Immediate,
        0x45 => 3, ZeroPage,
        0x55 => 4, ZeroPage_X,
        0x4D => 4, Absolute,
        0x5D => 4, Absolute_X,
        0x59 => 4, Absolute_Y,
        0x41 => 6, Indirect_X,
        0x51 => 5, Indirect_Y,
    },
    /// Logical Inclusive OR
    ORA {
        0x09 => 2, Immediate,
        0x05 => 3, ZeroPage,
        0x15 => 4, ZeroPage_X,
        0x0D => 4, Absolute,
        0x1D => 4, Absolute_X,
        0x19 => 4, Absolute_Y,
        0x01 => 6, Indirect_X,
        0x11 => 5, Indirect_Y,
    },
    /// Bit Test
    BIT {
        0x24 => 3, ZeroPage,
        0x2C => 4, Absolute,
    },

    // ===== Arithmetic =====
    /// Add with Carry
    ADC {
        0x69 => 2, Immediate,
        0x65 => 3, ZeroPage,
        0x75 => 4, ZeroPage_X,
        0x6D => 4, Absolute,
        0x7D => 4, Absolute_X,
        0x79 => 4, Absolute_Y,
        0x61 => 6, Indirect_X,
        0x71 => 5, Indirect_Y,
    },
    /// Subtract with Carry
    SBC {
        0xE9 => 2, Immediate,
        0xE5 => 3, ZeroPage,
        0xF5 => 4, ZeroPage_X,
        0xED => 4, Absolute,
        0xFD => 4, Absolute_X,
        0xF9 => 4, Absolute_Y,
        0xE1 => 6, Indirect_X,
        0xF1 => 5, Indirect_Y,
    },
    /// Compare Accumulator
    CMP {
        0xC9 => 2, Immediate,
        0xC5 => 3, ZeroPage,
        0xD5 => 4, ZeroPage_X,
        0xCD => 4, Absolute,
        0xDD => 4, Absolute_X,
        0xD9 => 4, Absolute_Y,
        0xC1 => 6, Indirect_X,
        0xD1 => 5, Indirect_Y,
    },
    /// Compare X register
    CPX {
        0xE0 => 2, Immediate,
        0xE4 => 3, ZeroPage,
        0xEC => 4, Absolute,
    },
    /// Compare Y register
    CPY {
        0xC0 => 2, Immediate,
        0xC4 => 3, ZeroPage,
        0xCC => 4, Absolute,
    },

    // ===== Increments & Decrements =====
    /// Increment a memory location
    INC {
        0xE6 => 5, ZeroPage,
        0xF6 => 6, ZeroPage_X,
        0xEE => 6, Absolute,
        0xFE => 7, Absolute_X,
    },
    /// Increment the X register
    INX {
        0xE8 => 2, Implicit,
    },
    /// Increment the Y register
    INY {
        0xC8 => 2, Implicit,
    },
    /// Decrement a memory location
    DEC {
        0xC6 => 5, ZeroPage,
        0xD6 => 6, ZeroPage_X,
        0xCE => 6, Absolute,
        0xDE => 7, Absolute_X,
    },
    /// Decrement the X register
    DEX {
        0xCA => 2, Implicit,
    },
    /// Decrement the Y register
    DEY {
        0x88 => 2, Implicit,
    },

    // ===== Shifts =====
    /// Arithmetic Shift Left
    ASL {
        0x0A => 2, Accumulator,
        0x06 => 5, ZeroPage,
        0x16 => 6, ZeroPage_X,
        0x0E => 6, Absolute,
        0x1E => 7, Absolute_X,
    },
    /// Logical Shift Right
    LSR {
        0x4A => 2, Accumulator,
        0x46 => 5, ZeroPage,
        0x56 => 6, ZeroPage_X,
        0x4E => 6, Absolute,
        0x5E => 7, Absolute_X,
    },
    /// Rotate Left
    ROL {
        0x2A => 2, Accumulator,
        0x26 => 5, ZeroPage,
        0x36 => 6, ZeroPage_X,
        0x2E => 6, Absolute,
        0x3E => 7, Absolute_X,
    },
    /// Rotate Right
    ROR {
        0x6A => 2, Accumulator,
        0x66 => 5, ZeroPage,
        0x76 => 6, ZeroPage_X,
        0x6E => 6, Absolute,
        0x7E => 7, Absolute_X,
    },

    // ===== Jumps & Calls =====
    /// Jump to another location
    JMP {
        0x4C => 3, Absolute,
        0x6C => 5, Indirect,
    },
    /// Jump to subroutine
    JSR {
        0x20 => 6, Absolute,
    },
    /// Return from subroutine
    RTS {
        0x60 => 6, Implicit,
    },

    // ===== Branches =====
    /// Branch if Carry flag clear
    BCC {
        0x90 => 2, Relative,
    },
    /// Branch if Carry flag set
    BCS {
        0xB0 => 2, Relative,
    },
    /// Branch if Zero flag set
    BEQ {
        0xF0 => 2, Relative,
    },
    /// Branch if Negative flag set
    BMI {
        0x30 => 2, Relative,
    },
    /// Branch if Zero flag clear
    BNE {
        0xD0 => 2, Relative,
    },
    /// Branch if Negative flag clear
    BPL {
        0x10 => 2, Relative,
    },
    /// Branch if Overflow flag clear
    BVC {
        0x50 => 2, Relative,
    },
    /// Branch if Overflow flag set
    BVS {
        0x70 => 2, Relative,
    },

    // ===== Status Flag Changes =====
    /// Clear Carry flag
    CLC {
        0x18 => 2, Implicit,
    },
    /// Clear Decimal Mode flag
    CLD {
        0xD8 => 2, Implicit,
    },
    /// Clear Interrupt Disable flag
    CLI {
        0x58 => 2, Implicit,
    },
    /// Clear Overflow flag
    CLV {
        0xB8 => 2, Implicit,
    },
    /// Set Carry flag
    SEC {
        0x38 => 2, Implicit,
    },
    /// Set Decimal Mode flag
    SED {
        0xF8 => 2, Implicit,
    },
    /// Set Interrupt Disable flag
    SEI {
        0x78 => 2, Implicit,
    },

    // ===== System Functions =====
    /// Force an Interrupt
    BRK {
        0x00 => 7, Implicit,
    },
    /// No Operation
    NOP {
        0xEA => 2, Implicit,
    },
    /// Return from Interrupt
    RTI {
        0x40 => 6, Implicit,
    },

    // ===== Undocumented Opcodes =====
    // https:///www.oxyron.de/html/opcodes02.html
    // https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    /// No Operation
    NOP_ALT "NOP" {
        0x1A => 2, Implicit,
        0x3A => 2, Implicit,
        0x5A => 2, Implicit,
        0x7A => 2, Implicit,
        0xDA => 2, Implicit,
        0xFA => 2, Implicit,
        0x80 => 2, Immediate,
        0x82 => 2, Immediate,
        0x89 => 2, Immediate,
        0xC2 => 2, Immediate,
        0xE2 => 2, Immediate,
        0x04 => 3, ZeroPage,
        0x44 => 3, ZeroPage,
        0x64 => 3, ZeroPage,
        0x14 => 4, ZeroPage_X,
        0x34 => 4, ZeroPage_X,
        0x54 => 4, ZeroPage_X,
        0x74 => 4, ZeroPage_X,
        0xD4 => 4, ZeroPage_X,
        0xF4 => 4, ZeroPage_X,
        0x0C => 4, Absolute,
        0x1C => 4, Absolute_X,
        0x3C => 4, Absolute_X,
        0x5C => 4, Absolute_X,
        0x7C => 4, Absolute_X,
        0xDC => 4, Absolute_X,
        0xFC => 4, Absolute_X,
    },

    // ===== Illegal Opcodes =====
    // https://www.oxyron.de/html/opcodes02.html
    // https://www.nesdev.org/wiki/CPU_unofficial_opcodes
    // https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes
    // https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes
    /// Equivalent to `ASL value` then `ORA value`
    SLO {
        0x07 => 5, ZeroPage,
        0x17 => 6, ZeroPage_X,
        0x0F => 6, Absolute,
        0x1F => 7, Absolute_X,
        0x1B => 7, Absolute_Y,
        0x03 => 8, Indirect_X,
        0x13 => 8, Indirect_Y,
    },
    /// Equivalent to `ROL value` then `AND value`
    RLA {
        0x27 => 5, ZeroPage,
        0x37 => 6, ZeroPage_X,
        0x2F => 6, Absolute,
        0x3F => 7, Absolute_X,
        0x3B => 7, Absolute_Y,
        0x23 => 8, Indirect_X,
        0x33 => 8, Indirect_Y,
    },
    /// Equivalent to `LSR value` then `EOR value`
    SRE {
        0x47 => 5, ZeroPage,
        0x57 => 6, ZeroPage_X,
        0x4F => 6, Absolute,
        0x5F => 7, Absolute_X,
        0x5B => 7, Absolute_Y,
        0x43 => 8, Indirect_X,
        0x53 => 8, Indirect_Y,
    },
    /// Equivalent to `ROR value` then `ADC value`
    RRA {
        0x67 => 5, ZeroPage,
        0x77 => 6, ZeroPage_X,
        0x6F => 6, Absolute,
        0x7F => 7, Absolute_X,
        0x7B => 7, Absolute_Y,
        0x63 => 8, Indirect_X,
        0x73 => 8, Indirect_Y,
    },
    /// Stores `A & X` into `{adr}`
    SAX {
        0x87 => 3, ZeroPage,
        0x97 => 4, ZeroPage_Y,
        0x8F => 4, Absolute,
        0x83 => 6, Indirect_X,
    },
    /// Shortcut for `LDA value` then `TAX`
    LAX {
        0xAB => 2, Immediate,
        0xA7 => 3, ZeroPage,
        0xB7 => 4, ZeroPage_Y,
        0xAF => 4, Absolute,
        0xBF => 4, Absolute_Y,
        0xA3 => 6, Indirect_X,
        0xB3 => 5, Indirect_Y,
    },
    /// Equivalent to `DEC value` then `CMP value`
    DCP {
        0xC7 => 5, ZeroPage,
        0xD7 => 6, ZeroPage_X,
        0xCF => 6, Absolute,
        0xDF => 7, Absolute_X,
        0xDB => 7, Absolute_Y,
        0xC3 => 8, Indirect_X,
        0xD3 => 8, Indirect_Y,
    },
    /// Equivalent to `INC value` then `SBC value`
    ISC {
        0xE7 => 5, ZeroPage,
        0xF7 => 6, ZeroPage_X,
        0xEF => 6, Absolute,
        0xFF => 7, Absolute_X,
        0xFB => 7, Absolute_Y,
        0xE3 => 8, Indirect_X,
        0xF3 => 8, Indirect_Y,
    },
    /// Does `AND #i` then copies `N` to `C`
    ANC {
        0x0B => 2, Immediate,
        0x2B => 2, Immediate,
    },
    /// Equivalent to `AND #i` then `LSR A`
    ALR {
        0x4B => 2, Immediate,
    },
    /// Similar to `AND #i`, but `C` is `bit 6` and `V` is `bit 6 XOR bit 5`
    ARR {
        0x6B => 2, Immediate,
    },
    /// Unpredictable behavior - https:///www.nesdev.org/wiki/Visual6502wiki/6502_Opcode_8B_(XAA,_ANE) \
    /// ***WARNING:*** Highly Unstable
    XAA {
        0x8B => 2, Immediate,
    },
    /// Sets `X` to `A & X - #{imm}`
    AXS {
        0xCB => 2, Immediate,
    },
    /// Equivalent to `SBC #i` then `NOP`
    SBC_NOP "SBC" {
        0xEB => 2, Immediate,
    },
    /// An incorrectly-implemented version of `SAX value` \
    /// **WARNING:** Unstable in certain situations
    AHX {
        0x9F => 5, Absolute_Y,
        0x93 => 6, Indirect_Y,
    },
    /// An incorrectly-implemented version of `STY a,X` \
    /// **WARNING:** Unstable in certain situations
    SHY {
        0x9C => 5, Absolute_X,
    },
    /// An incorrectly-implemented version of `STX a,Y` \
    /// **WARNING:** Unstable in certain situations
    SHX {
        0x9E => 5, Absolute_Y,
    },
    /// Stores `A & X` into `S` then `AHX a,Y` \
    /// **WARNING:** Unstable in certain situations
    TAS {
        0x9B => 5, Absolute_Y,
    },
    /// Stores `{adr} & S` into `A`, `X`, and `S`
    LAS {
        0xBB => 4, Absolute_Y,
    },
    /// Traps the CPU indefinitely with $FF on the bus, requires a reset to fix
    KIL {
        0x02 => 0, Implicit,
        0x12 => 0, Implicit,
        0x22 => 0, Implicit,
        0x32 => 0, Implicit,
        0x42 => 0, Implicit,
        0x52 => 0, Implicit,
        0x62 => 0, Implicit,
        0x72 => 0, Implicit,
        0x92 => 0, Implicit,
        0xB2 => 0, Implicit,
        0xD2 => 0, Implicit,
        0xF2 => 0, Implicit,
    },
);
