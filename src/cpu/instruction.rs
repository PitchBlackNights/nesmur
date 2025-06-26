use crate::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

impl AddressingMode {
    pub const fn opcode_bytes(self) -> u8 {
        match self {
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
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OpCode {
    pub byte: u8,
    pub instruction: Instruction,
    pub mnemonic: &'static str,
    pub len: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        byte: u8,
        instruction: Instruction,
        mnemonic: &'static str,
        mode: AddressingMode,
    ) -> Self {
        Self {
            byte,
            instruction,
            mnemonic,
            len: mode.opcode_bytes(),
            mode,
        }
    }
}

pub fn decode_opcode(opbyte: u8) -> &'static OpCode {
    OPCODES
        .get(&opbyte)
        .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", opbyte))
}

macro_rules! define_opcodes {
    (
        $(
            $( #[$enum_doc:meta] )*
            $instr:ident $( $mnemonic:literal )? {
                $( $opcode:literal => $mode:ident ),+ $(,)?
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

        static OPCODES: Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
            trace!("Building OPCODES hashmap...");
            let mut map = HashMap::new();
            $(
                let instruction: Instruction = Instruction::$instr;
                let mnemonic: &'static str = define_opcodes!(@mnemonic $instr $( $mnemonic )?);
                $(
                    map.insert(
                        $opcode,
                        OpCode::new($opcode, instruction, mnemonic, AddressingMode::$mode)
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
        0xA9 => Immediate,
        0xA5 => ZeroPage,
        0xB5 => ZeroPage_X,
        0xAD => Absolute,
        0xBD => Absolute_X,
        0xB9 => Absolute_Y,
        0xA1 => Indirect_X,
        0xB1 => Indirect_Y,
    },
    /// Load X register
    LDX {
        0xA2 => Immediate,
        0xA6 => ZeroPage,
        0xB6 => ZeroPage_Y,
        0xAE => Absolute,
        0xBE => Absolute_Y,
    },
    /// Load Y register
    LDY {
        0xA0 => Immediate,
        0xA4 => ZeroPage,
        0xB4 => ZeroPage_X,
        0xAC => Absolute,
        0xBC => Absolute_X,
    },
    /// Store Accumulator
    STA {
        0x85 => ZeroPage,
        0x95 => ZeroPage_X,
        0x8D => Absolute,
        0x9D => Absolute_X,
        0x99 => Absolute_Y,
        0x81 => Indirect_X,
        0x91 => Indirect_Y,
    },
    /// Store X register
    STX {
        0x86 => ZeroPage,
        0x96 => ZeroPage_Y,
        0x8E => Absolute,
    },
    /// Store Y register
    STY {
        0x84 => ZeroPage,
        0x94 => ZeroPage_Y,
        0x8C => Absolute,
    },

    // ===== Register Transfers =====
    /// Transfer Accumulator to X
    TAX {
        0xAA => Implicit,
    },
    /// Transfer Accumulator to Y
    TAY {
        0xA8 => Implicit,
    },
    /// Transfer X to Accumulator
    TXA {
        0x8A => Implicit,
    },
    /// Transfer Y to Accumulator
    TYA {
        0x98 => Implicit,
    },

    // ===== Stack Operations =====
    /// Transfer Stack Pointer to X
    TSX {
        0xBA => Implicit,
    },
    /// Transfer X to Stack Pointer
    TXS {
        0x9A => Implicit,
    },
    /// Push Accumulator on Stack
    PHA {
        0x48 => Implicit,
    },
    /// Push Processor Status on Stack
    PHP {
        0x08 => Implicit,
    },
    /// Pull Accumulator from Stack
    PLA {
        0x68 => Implicit,
    },
    /// Pull Processor Status from Stack
    PLP {
        0x28 => Implicit,
    },

    // ===== Logical =====
    /// Logical AND
    AND {
        0x29 => Immediate,
        0x25 => ZeroPage,
        0x35 => ZeroPage_X,
        0x2D => Absolute,
        0x3D => Absolute_X,
        0x39 => Absolute_Y,
        0x21 => Indirect_X,
        0x31 => Indirect_Y,
    },
    /// Exclusive OR
    EOR {
        0x49 => Immediate,
        0x45 => ZeroPage,
        0x55 => ZeroPage_X,
        0x4D => Absolute,
        0x5D => Absolute_X,
        0x59 => Absolute_Y,
        0x41 => Indirect_X,
        0x51 => Indirect_Y,
    },
    /// Logical Inclusive OR
    ORA {
        0x09 => Immediate,
        0x05 => ZeroPage,
        0x15 => ZeroPage_X,
        0x0D => Absolute,
        0x1D => Absolute_X,
        0x19 => Absolute_Y,
        0x01 => Indirect_X,
        0x11 => Indirect_Y,
    },
    /// Bit Test
    BIT {
        0x24 => ZeroPage,
        0x2C => Absolute,
    },

    // ===== Arithmetic =====
    /// Add with Carry
    ADC {
        0x69 => Immediate,
        0x65 => ZeroPage,
        0x75 => ZeroPage_X,
        0x6D => Absolute,
        0x7D => Absolute_X,
        0x79 => Absolute_Y,
        0x61 => Indirect_X,
        0x71 => Indirect_Y,
    },
    /// Subtract with Carry
    SBC {
        0xE9 => Immediate,
        0xE5 => ZeroPage,
        0xF5 => ZeroPage_X,
        0xED => Absolute,
        0xFD => Absolute_X,
        0xF9 => Absolute_Y,
        0xE1 => Indirect_X,
        0xF1 => Indirect_Y,
    },
    /// Compare Accumulator
    CMP {
        0xC9 => Immediate,
        0xC5 => ZeroPage,
        0xD5 => ZeroPage_X,
        0xCD => Absolute,
        0xDD => Absolute_X,
        0xD9 => Absolute_Y,
        0xC1 => Indirect_X,
        0xD1 => Indirect_Y,
    },
    /// Compare X register
    CPX {
        0xE0 => Immediate,
        0xE4 => ZeroPage,
        0xEC => Absolute,
    },
    /// Compare Y register
    CPY {
        0xC0 => Immediate,
        0xC4 => ZeroPage,
        0xCC => Absolute,
    },

    // ===== Increment & Decrements =====
    /// Increment a memory location
    INC {
        0xE6 => ZeroPage,
        0xF6 => ZeroPage_X,
        0xEE => Absolute,
        0xFE => Absolute_X,
    },
    /// Increment the X register
    INX {
        0xE8 => Implicit,
    },
    /// Increment the Y register
    INY {
        0xC8 => Implicit,
    },
    /// Decrement a memory location
    DEC {
        0xC6 => ZeroPage,
        0xD6 => ZeroPage_X,
        0xCE => Absolute,
        0xDE => Absolute_X,
    },
    /// Decrement the X register
    DEX {
        0xCA => Implicit,
    },
    /// Decrement the Y register
    DEY {
        0x88 => Implicit,
    },

    // ===== Shifts =====
    /// Arithmetic Shift Left
    ASL {
        0x0A => Accumulator,
        0x06 => ZeroPage,
        0x16 => ZeroPage_X,
        0x0E => Absolute,
        0x1E => Absolute_X,
    },
    /// Logical Shift Right
    LSR {
        0x4A => Accumulator,
        0x46 => ZeroPage,
        0x56 => ZeroPage_X,
        0x4E => Absolute,
        0x5E => Absolute_X,
    },
    /// Rotate Left
    ROL {
        0x2A => Accumulator,
        0x26 => ZeroPage,
        0x36 => ZeroPage_X,
        0x2E => Absolute,
        0x3E => Absolute_X,
    },
    /// Rotate Right
    ROR {
        0x6A => Accumulator,
        0x66 => ZeroPage,
        0x76 => ZeroPage_X,
        0x6E => Absolute,
        0x7E => Absolute_X,
    },

    // ===== Jumps & Calls =====
    /// Jump to another location
    JMP {
        0x4C => Absolute,
        0x6C => Indirect,
    },
    /// Jump to subroutine
    JSR {
        0x20 => Absolute,
    },
    /// Return from subroutine
    RTS {
        0x60 => Implicit,
    },

    // ===== Branches =====
    /// Branch if Carry flag clear
    BCC {
        0x90 => Relative,
    },
    /// Branch if Carry flag set
    BCS {
        0xB0 => Relative,
    },
    /// Branch if Zero flag set
    BEQ {
        0xF0 => Relative,
    },
    /// Branch if Negative flag set
    BMI {
        0x30 => Relative,
    },
    /// Branch if Zero flag clear
    BNE {
        0xD0 => Relative,
    },
    /// Branch if Negative flag clear
    BPL {
        0x10 => Relative,
    },
    /// Branch if Overflow flag clear
    BVC {
        0x50 => Relative,
    },
    /// Branch if Overflow flag set
    BVS {
        0x70 => Relative,
    },

    // ===== Status Flag Changes =====
    /// Clear Carry flag
    CLC {
        0x18 => Implicit,
    },
    /// Clear Decimal Mode flag
    CLD {
        0xD8 => Implicit,
    },
    /// Clear Interrupt Disable flag
    CLI {
        0x58 => Implicit,
    },
    /// Clear Overflow flag
    CLV {
        0xB8 => Implicit,
    },
    /// Set Carry flag
    SEC {
        0x38 => Implicit,
    },
    /// Set Decimal Mode flag
    SED {
        0xF8 => Implicit,
    },
    /// Set Interrupt Disable flag
    SEI {
        0x78 => Implicit,
    },

    // ===== System Functions =====
    /// Force an Interrupt
    BRK {
        0x00 => Implicit,
    },
    /// No Operation
    NOP {
        0xEA => Implicit,
    },
    /// Return from Interrupt
    RTI {
        0x40 => Implicit,
    },

    // ===== Undocumented Opcodes =====
    /// https:///www.oxyron.de/html/opcodes02.html
    /// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    NOP_ALT "NOP" {
        0xFA => Implicit,
        0x0C => Absolute,
        0xFC => Absolute_X,
        0x64 => ZeroPage,
        0xf4 => ZeroPage_X,
        0xe2 => Immediate,
    },

    // ===== Illegal Opcodes =====
    /// https:///www.oxyron.de/html/opcodes02.html
    /// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    /// https:///www.nesdev.org/wiki/Programming_with_unofficial_opcodes
    /// Equivalent to `ASL value` then `ORA value`
    SLO {
        0x07 => ZeroPage,
        0x17 => ZeroPage_X,
        0x03 => Indirect_X,
        0x13 => Indirect_Y,
        0x0F => Absolute,
        0x1F => Absolute_X,
        0x1B => Absolute_Y,
    },
    /// Equivalent to `ROL value` then `AND value`
    RLA {
        0x27 => ZeroPage,
        0x37 => ZeroPage_X,
        0x23 => Indirect_X,
        0x33 => Indirect_Y,
        0x2F => Absolute,
        0x3F => Absolute_X,
        0x3B => Absolute_Y,
    },
    /// Equivalent to `LSR value` then `EOR value`
    SRE {
        0x47 => ZeroPage,
        0x57 => ZeroPage_X,
        0x43 => Indirect_X,
        0x53 => Indirect_Y,
        0x4F => Absolute,
        0x5F => Absolute_X,
        0x5B => Absolute_Y,
    },
    /// Equivalent to `ROR value` then `ADC value`
    RRA {
        0x67 => ZeroPage,
        0x77 => ZeroPage_X,
        0x63 => Indirect_X,
        0x73 => Indirect_Y,
        0x6F => Absolute,
        0x7F => Absolute_X,
        0x7B => Absolute_Y,
    },
    /// Stores `A & X` into `{adr}`
    SAX {
        0x87 => ZeroPage,
        0x97 => ZeroPage_Y,
        0x83 => Indirect_X,
        0x8F => Absolute,
    },
    /// Shortcut for `LDA value` then `TAX`
    LAX {
        0xAB => Immediate,
        0xA7 => ZeroPage,
        0xB7 => ZeroPage_Y,
        0xA3 => Indirect_X,
        0xB3 => Indirect_Y,
        0xAF => Absolute,
        0xBF => Absolute_Y,
    },
    /// Equivalent to `DEC value` then `CMP value`
    DCP {
        0xC7 => ZeroPage,
        0xD7 => ZeroPage_X,
        0xC3 => Indirect_X,
        0xD3 => Indirect_Y,
        0xCF => Absolute,
        0xDF => Absolute_X,
        0xDB => Absolute_Y,
    },
    /// Equivalent to `INC value` then `SBC value`
    ISC {
        0xE7 => ZeroPage,
        0xF7 => ZeroPage_X,
        0xE3 => Indirect_X,
        0xF3 => Indirect_Y,
        0xEF => Absolute,
        0xFF => Absolute_X,
        0xFB => Absolute_Y,
    },
    /// Does `AND #i` then copies `N` to `C`
    ANC {
        0x2B => Immediate,
    },
    /// Equivalent to `AND #i` then `LSR A`
    ALR {
        0x4B => Immediate,
    },
    /// Similar to `AND #i`, but `C` is `bit 6` and `V` is `bit 6 XOR bit 5`
    ARR {
        0x6B => Immediate,
    },
    /// Unpredictable behavior - https:///www.nesdev.org/wiki/Visual6502wiki/6502_Opcode_8B_(XAA,_ANE)
    XAA {
        0x8B => Immediate,
    },
    /// Sets `X` to `A & X - #{imm}`
    AXS {
        0xCB => Immediate,
    },
    /// Equivalent to `SBC #i` then `NOP`
    SBC_NOP "SBC" {
        0xEB => Immediate,
    },
    /// An incorrectly-implemented version of `SAX value`
    AHX {
        0x93 => Indirect_Y,
        0x9F => Absolute_Y,
    },
    /// An incorrectly-implemented version of `STY a,X`
    SHY {
        0x9C => Absolute_X,
    },
    /// An incorrectly-implemented version of `STX a,Y`
    SHX {
        0x9E => Absolute_Y,
    },
    /// Stores `A & X` into `S` then `AHX a,Y`
    TAS {
        0x9B => Absolute_Y,
    },
    /// Stores `{adr} & S` into `A`, `X`, and `S`
    LAS {
        0xBB => Absolute_Y,
    },
);
