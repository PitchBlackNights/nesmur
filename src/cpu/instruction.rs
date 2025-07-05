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
        .unwrap_or_else(|| panic!("OpCode {opbyte:#04X} is not recognized"))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Instruction {
    // ===== Load/Store Operations =====
    /// Load Accumulator
    LDA,
    /// Load X register
    LDX,
    /// Load Y register
    LDY,
    /// Store Accumulator
    STA,
    /// Store X register
    STX,
    /// Store Y register
    STY,
    /// Transfer Accumulator to X
    // ===== Register Transfers =====
    TAX,
    /// Transfer Accumulator to Y
    TAY,
    /// Transfer X to Accumulator
    TXA,
    /// Transfer Y to Accumulator
    TYA,

    // ===== Stack Operations =====
    /// Transfer Stack Pointer to X
    TSX,
    /// Transfer X to Stack Pointer
    TXS,
    /// Push Accumulator on Stack
    PHA,
    /// Push Processor Status on Stack
    PHP,
    /// Pull Accumulator from Stack
    PLA,
    /// Pull Processor Status from Stack
    PLP,

    // ===== Logical =====
    /// Logical AND
    AND,
    /// Exclusive OR
    EOR,
    /// Logical Inclusive OR
    ORA,
    /// Bit Test
    BIT,

    // ===== Arithmetic =====
    /// Add with Carry
    ADC,
    /// Subtract with Carry
    SBC,
    /// Compare Accumulator
    CMP,
    /// Compare X register
    CPX,
    /// Compare Y register
    CPY,

    // ===== Increments & Decrements =====
    /// Increment a memory location
    INC,
    /// Increment the X register
    INX,
    /// Increment the Y register
    INY,
    /// Decrement a memory location
    DEC,
    /// Decrement the X register
    DEX,
    /// Decrement the Y register
    DEY,

    // ===== Shifts =====
    /// Arithmetic Shift Left
    ASL,
    /// Logical Shift Right
    LSR,
    /// Rotate Left
    ROL,
    /// Rotate Right
    ROR,

    // ===== Jumps & Calls =====
    /// Jump to another location
    JMP,
    /// Jump to subroutine
    JSR,
    /// Return from subroutine
    RTS,

    // ===== Branches =====
    /// Branch if Carry flag clear
    BCC,
    /// Branch if Carry flag set
    BCS,
    /// Branch if Zero flag set
    BEQ,
    /// Branch if Negative flag set
    BMI,
    /// Branch if Zero flag clear
    BNE,
    /// Branch if Negative flag clear
    BPL,
    /// Branch if Overflow flag clear
    BVC,
    /// Branch if Overflow flag set
    BVS,

    // ===== Status Flag Changes =====
    /// Clear Carry flag
    CLC,
    /// Clear Decimal Mode flag
    CLD,
    /// Clear Interrupt Disable flag
    CLI,
    /// Clear Overflow flag
    CLV,
    /// Set Carry flag
    SEC,
    /// Set Decimal Mode flag
    SED,
    /// Set Interrupt Disable flag
    SEI,

    // ===== System Functions =====
    /// Force an Interrupt
    BRK,
    /// No Operation
    NOP,
    /// Return from Interrupt
    RTI,

    // ===== Undocumented Opcodes =====
    /// https:///www.oxyron.de/html/opcodes02.html
    /// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    NOP_ALT,

    // ===== Illegal Opcodes =====
    /// https:///www.oxyron.de/html/opcodes02.html
    /// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    /// https:///www.nesdev.org/wiki/Programming_with_unofficial_opcodes
    /// Equivalent to `ASL value` then `ORA value`
    SLO,
    /// Equivalent to `ROL value` then `AND value`
    RLA,
    /// Equivalent to `LSR value` then `EOR value`
    SRE,
    /// Equivalent to `ROR value` then `ADC value`
    RRA,
    /// Stores `A & X` into `{adr}`
    SAX,
    /// Shortcut for `LDA value` then `TAX`
    LAX,
    /// Equivalent to `DEC value` then `CMP value`
    DCP,
    /// Equivalent to `INC value` then `SBC value`
    ISC,
    /// Does `AND #i` then copies `N` to `C`
    ANC,
    /// Equivalent to `AND #i` then `LSR A`
    ALR,
    /// Similar to `AND #i`, but `C` is `bit 6` and `V` is `bit 6 XOR bit 5`
    ARR,
    /// Unpredictable behavior - https:///www.nesdev.org/wiki/Visual6502wiki/6502_Opcode_8B_(XAA,_ANE)
    XAA,
    /// Sets `X` to `A & X - #{imm}`
    AXS,
    /// Equivalent to `SBC #i` then `NOP`
    SBC_NOP,
    /// An incorrectly-implemented version of `SAX value`
    AHX,
    /// An incorrectly-implemented version of `STY a,X`
    SHY,
    /// An incorrectly-implemented version of `STX a,Y`
    SHX,
    /// Stores `A & X` into `S` then `AHX a,Y`
    TAS,
    /// Stores `{adr} & S` into `A`, `X`, and `S`
    LAS,
}

pub static OPCODES: Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
    trace!("Building OPCODES hashmap...");

    #[rustfmt::skip]
    let opcodes_vec: Vec<OpCode> = vec![
        OpCode::new(0xA9, Instruction::LDA, "LDA", AddressingMode::Immediate),
        OpCode::new(0xA5, Instruction::LDA, "LDA", AddressingMode::ZeroPage),
        OpCode::new(0xB5, Instruction::LDA, "LDA", AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, Instruction::LDA, "LDA", AddressingMode::Absolute),
        OpCode::new(0xBD, Instruction::LDA, "LDA", AddressingMode::Absolute_X),
        OpCode::new(0xB9, Instruction::LDA, "LDA", AddressingMode::Absolute_Y),
        OpCode::new(0xA1, Instruction::LDA, "LDA", AddressingMode::Indirect_X),
        OpCode::new(0xB1, Instruction::LDA, "LDA", AddressingMode::Indirect_Y),
        OpCode::new(0xA2, Instruction::LDX, "LDX", AddressingMode::Immediate),
        OpCode::new(0xA6, Instruction::LDX, "LDX", AddressingMode::ZeroPage),
        OpCode::new(0xB6, Instruction::LDX, "LDX", AddressingMode::ZeroPage_Y),
        OpCode::new(0xAE, Instruction::LDX, "LDX", AddressingMode::Absolute),
        OpCode::new(0xBE, Instruction::LDX, "LDX", AddressingMode::Absolute_Y),
        OpCode::new(0xA0, Instruction::LDY, "LDY", AddressingMode::Immediate),
        OpCode::new(0xA4, Instruction::LDY, "LDY", AddressingMode::ZeroPage),
        OpCode::new(0xB4, Instruction::LDY, "LDY", AddressingMode::ZeroPage_X),
        OpCode::new(0xAC, Instruction::LDY, "LDY", AddressingMode::Absolute),
        OpCode::new(0xBC, Instruction::LDY, "LDY", AddressingMode::Absolute_X),
        OpCode::new(0x85, Instruction::STA, "STA", AddressingMode::ZeroPage),
        OpCode::new(0x95, Instruction::STA, "STA", AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, Instruction::STA, "STA", AddressingMode::Absolute),
        OpCode::new(0x9D, Instruction::STA, "STA", AddressingMode::Absolute_X),
        OpCode::new(0x99, Instruction::STA, "STA", AddressingMode::Absolute_Y),
        OpCode::new(0x81, Instruction::STA, "STA", AddressingMode::Indirect_X),
        OpCode::new(0x91, Instruction::STA, "STA", AddressingMode::Indirect_Y),
        OpCode::new(0x86, Instruction::STX, "STX", AddressingMode::ZeroPage),
        OpCode::new(0x96, Instruction::STX, "STX", AddressingMode::ZeroPage_Y),
        OpCode::new(0x8E, Instruction::STX, "STX", AddressingMode::Absolute),
        OpCode::new(0x84, Instruction::STY, "STY", AddressingMode::ZeroPage),
        OpCode::new(0x94, Instruction::STY, "STY", AddressingMode::ZeroPage_Y),
        OpCode::new(0x8C, Instruction::STY, "STY", AddressingMode::Absolute),
        OpCode::new(0xAA, Instruction::TAX, "TAX", AddressingMode::Implicit),
        OpCode::new(0xA8, Instruction::TAY, "TAY", AddressingMode::Implicit),
        OpCode::new(0x8A, Instruction::TXA, "TXA", AddressingMode::Implicit),
        OpCode::new(0x98, Instruction::TYA, "TYA", AddressingMode::Implicit),
        OpCode::new(0xBA, Instruction::TSX, "TSX", AddressingMode::Implicit),
        OpCode::new(0x9A, Instruction::TXS, "TXS", AddressingMode::Implicit),
        OpCode::new(0x48, Instruction::PHA, "PHA", AddressingMode::Implicit),
        OpCode::new(0x08, Instruction::PHP, "PHP", AddressingMode::Implicit),
        OpCode::new(0x68, Instruction::PLA, "PLA", AddressingMode::Implicit),
        OpCode::new(0x28, Instruction::PLP, "PLP", AddressingMode::Implicit),
        OpCode::new(0x29, Instruction::AND, "AND", AddressingMode::Immediate),
        OpCode::new(0x25, Instruction::AND, "AND", AddressingMode::ZeroPage),
        OpCode::new(0x35, Instruction::AND, "AND", AddressingMode::ZeroPage_X),
        OpCode::new(0x2D, Instruction::AND, "AND", AddressingMode::Absolute),
        OpCode::new(0x3D, Instruction::AND, "AND", AddressingMode::Absolute_X),
        OpCode::new(0x39, Instruction::AND, "AND", AddressingMode::Absolute_Y),
        OpCode::new(0x21, Instruction::AND, "AND", AddressingMode::Indirect_X),
        OpCode::new(0x31, Instruction::AND, "AND", AddressingMode::Indirect_Y),
        OpCode::new(0x49, Instruction::EOR, "EOR", AddressingMode::Immediate),
        OpCode::new(0x45, Instruction::EOR, "EOR", AddressingMode::ZeroPage),
        OpCode::new(0x55, Instruction::EOR, "EOR", AddressingMode::ZeroPage_X),
        OpCode::new(0x4D, Instruction::EOR, "EOR", AddressingMode::Absolute),
        OpCode::new(0x5D, Instruction::EOR, "EOR", AddressingMode::Absolute_X),
        OpCode::new(0x59, Instruction::EOR, "EOR", AddressingMode::Absolute_Y),
        OpCode::new(0x41, Instruction::EOR, "EOR", AddressingMode::Indirect_X),
        OpCode::new(0x51, Instruction::EOR, "EOR", AddressingMode::Indirect_Y),
        OpCode::new(0x09, Instruction::ORA, "ORA", AddressingMode::Immediate),
        OpCode::new(0x05, Instruction::ORA, "ORA", AddressingMode::ZeroPage),
        OpCode::new(0x15, Instruction::ORA, "ORA", AddressingMode::ZeroPage_X),
        OpCode::new(0x0D, Instruction::ORA, "ORA", AddressingMode::Absolute),
        OpCode::new(0x1D, Instruction::ORA, "ORA", AddressingMode::Absolute_X),
        OpCode::new(0x19, Instruction::ORA, "ORA", AddressingMode::Absolute_Y),
        OpCode::new(0x01, Instruction::ORA, "ORA", AddressingMode::Indirect_X),
        OpCode::new(0x11, Instruction::ORA, "ORA", AddressingMode::Indirect_Y),
        OpCode::new(0x24, Instruction::BIT, "BIT", AddressingMode::ZeroPage),
        OpCode::new(0x2C, Instruction::BIT, "BIT", AddressingMode::Absolute),
        OpCode::new(0x69, Instruction::ADC, "ADC", AddressingMode::Immediate),
        OpCode::new(0x65, Instruction::ADC, "ADC", AddressingMode::ZeroPage),
        OpCode::new(0x75, Instruction::ADC, "ADC", AddressingMode::ZeroPage_X),
        OpCode::new(0x6D, Instruction::ADC, "ADC", AddressingMode::Absolute),
        OpCode::new(0x7D, Instruction::ADC, "ADC", AddressingMode::Absolute_X),
        OpCode::new(0x79, Instruction::ADC, "ADC", AddressingMode::Absolute_Y),
        OpCode::new(0x61, Instruction::ADC, "ADC", AddressingMode::Indirect_X),
        OpCode::new(0x71, Instruction::ADC, "ADC", AddressingMode::Indirect_Y),
        OpCode::new(0xE9, Instruction::SBC, "SBC", AddressingMode::Immediate),
        OpCode::new(0xE5, Instruction::SBC, "SBC", AddressingMode::ZeroPage),
        OpCode::new(0xF5, Instruction::SBC, "SBC", AddressingMode::ZeroPage_X),
        OpCode::new(0xED, Instruction::SBC, "SBC", AddressingMode::Absolute),
        OpCode::new(0xFD, Instruction::SBC, "SBC", AddressingMode::Absolute_X),
        OpCode::new(0xF9, Instruction::SBC, "SBC", AddressingMode::Absolute_Y),
        OpCode::new(0xE1, Instruction::SBC, "SBC", AddressingMode::Indirect_X),
        OpCode::new(0xF1, Instruction::SBC, "SBC", AddressingMode::Indirect_Y),
        OpCode::new(0xC9, Instruction::CMP, "CMP", AddressingMode::Immediate),
        OpCode::new(0xC5, Instruction::CMP, "CMP", AddressingMode::ZeroPage),
        OpCode::new(0xD5, Instruction::CMP, "CMP", AddressingMode::ZeroPage_X),
        OpCode::new(0xCD, Instruction::CMP, "CMP", AddressingMode::Absolute),
        OpCode::new(0xDD, Instruction::CMP, "CMP", AddressingMode::Absolute_X),
        OpCode::new(0xD9, Instruction::CMP, "CMP", AddressingMode::Absolute_Y),
        OpCode::new(0xC1, Instruction::CMP, "CMP", AddressingMode::Indirect_X),
        OpCode::new(0xD1, Instruction::CMP, "CMP", AddressingMode::Indirect_Y),
        OpCode::new(0xE0, Instruction::CPX, "CPX", AddressingMode::Immediate),
        OpCode::new(0xE4, Instruction::CPX, "CPX", AddressingMode::ZeroPage),
        OpCode::new(0xEC, Instruction::CPX, "CPX", AddressingMode::Absolute),
        OpCode::new(0xC0, Instruction::CPY, "CPY", AddressingMode::Immediate),
        OpCode::new(0xC4, Instruction::CPY, "CPY", AddressingMode::ZeroPage),
        OpCode::new(0xCC, Instruction::CPY, "CPY", AddressingMode::Absolute),
        OpCode::new(0xE6, Instruction::INC, "INC", AddressingMode::ZeroPage),
        OpCode::new(0xF6, Instruction::INC, "INC", AddressingMode::ZeroPage_X),
        OpCode::new(0xEE, Instruction::INC, "INC", AddressingMode::Absolute),
        OpCode::new(0xFE, Instruction::INC, "INC", AddressingMode::Absolute_X),
        OpCode::new(0xE8, Instruction::INX, "INX", AddressingMode::Implicit),
        OpCode::new(0xC8, Instruction::INY, "INY", AddressingMode::Implicit),
        OpCode::new(0xC6, Instruction::DEC, "DEC", AddressingMode::ZeroPage),
        OpCode::new(0xD6, Instruction::DEC, "DEC", AddressingMode::ZeroPage_X),
        OpCode::new(0xCE, Instruction::DEC, "DEC", AddressingMode::Absolute),
        OpCode::new(0xDE, Instruction::DEC, "DEC", AddressingMode::Absolute_X),
        OpCode::new(0xCA, Instruction::DEX, "DEX", AddressingMode::Implicit),
        OpCode::new(0x88, Instruction::DEY, "DEY", AddressingMode::Implicit),
        OpCode::new(0x0A, Instruction::ASL, "ASL", AddressingMode::Accumulator),
        OpCode::new(0x06, Instruction::ASL, "ASL", AddressingMode::ZeroPage),
        OpCode::new(0x16, Instruction::ASL, "ASL", AddressingMode::ZeroPage_X),
        OpCode::new(0x0E, Instruction::ASL, "ASL", AddressingMode::Absolute),
        OpCode::new(0x1E, Instruction::ASL, "ASL", AddressingMode::Absolute_X),
        OpCode::new(0x4A, Instruction::LSR, "LSR", AddressingMode::Accumulator),
        OpCode::new(0x46, Instruction::LSR, "LSR", AddressingMode::ZeroPage),
        OpCode::new(0x56, Instruction::LSR, "LSR", AddressingMode::ZeroPage_X),
        OpCode::new(0x4E, Instruction::LSR, "LSR", AddressingMode::Absolute),
        OpCode::new(0x5E, Instruction::LSR, "LSR", AddressingMode::Absolute_X),
        OpCode::new(0x2A, Instruction::ROL, "ROL", AddressingMode::Accumulator),
        OpCode::new(0x26, Instruction::ROL, "ROL", AddressingMode::ZeroPage),
        OpCode::new(0x36, Instruction::ROL, "ROL", AddressingMode::ZeroPage_X),
        OpCode::new(0x2E, Instruction::ROL, "ROL", AddressingMode::Absolute),
        OpCode::new(0x3E, Instruction::ROL, "ROL", AddressingMode::Absolute_X),
        OpCode::new(0x6A, Instruction::ROR, "ROR", AddressingMode::Accumulator),
        OpCode::new(0x66, Instruction::ROR, "ROR", AddressingMode::ZeroPage),
        OpCode::new(0x76, Instruction::ROR, "ROR", AddressingMode::ZeroPage_X),
        OpCode::new(0x6E, Instruction::ROR, "ROR", AddressingMode::Absolute),
        OpCode::new(0x7E, Instruction::ROR, "ROR", AddressingMode::Absolute_X),
        OpCode::new(0x4C, Instruction::JMP, "JMP", AddressingMode::Absolute),
        OpCode::new(0x6C, Instruction::JMP, "JMP", AddressingMode::Indirect),
        OpCode::new(0x20, Instruction::JSR, "JSR", AddressingMode::Absolute),
        OpCode::new(0x60, Instruction::RTS, "RTS", AddressingMode::Implicit),
        OpCode::new(0x90, Instruction::BCC, "BCC", AddressingMode::Relative),
        OpCode::new(0xB0, Instruction::BCS, "BCS", AddressingMode::Relative),
        OpCode::new(0xF0, Instruction::BEQ, "BEQ", AddressingMode::Relative),
        OpCode::new(0x30, Instruction::BMI, "BMI", AddressingMode::Relative),
        OpCode::new(0xD0, Instruction::BNE, "BNE", AddressingMode::Relative),
        OpCode::new(0x10, Instruction::BPL, "BPL", AddressingMode::Relative),
        OpCode::new(0x50, Instruction::BVC, "BVC", AddressingMode::Relative),
        OpCode::new(0x70, Instruction::BVS, "BVS", AddressingMode::Relative),
        OpCode::new(0x18, Instruction::CLC, "CLC", AddressingMode::Implicit),
        OpCode::new(0xD8, Instruction::CLD, "CLD", AddressingMode::Implicit),
        OpCode::new(0x58, Instruction::CLI, "CLI", AddressingMode::Implicit),
        OpCode::new(0xB8, Instruction::CLV, "CLV", AddressingMode::Implicit),
        OpCode::new(0x38, Instruction::SEC, "SEC", AddressingMode::Implicit),
        OpCode::new(0xF8, Instruction::SED, "SED", AddressingMode::Implicit),
        OpCode::new(0x78, Instruction::SEI, "SEI", AddressingMode::Implicit),
        OpCode::new(0x00, Instruction::BRK, "BRK", AddressingMode::Implicit),
        OpCode::new(0xEA, Instruction::NOP, "NOP", AddressingMode::Implicit),
        OpCode::new(0x40, Instruction::RTI, "RTI", AddressingMode::Implicit),
        OpCode::new(0xFA, Instruction::NOP_ALT, "NOP", AddressingMode::Implicit),
        OpCode::new(0x0C, Instruction::NOP_ALT, "NOP", AddressingMode::Absolute),
        OpCode::new(0xFC, Instruction::NOP_ALT, "NOP", AddressingMode::Absolute_X),
        OpCode::new(0x64, Instruction::NOP_ALT, "NOP", AddressingMode::ZeroPage),
        OpCode::new(0xF4, Instruction::NOP_ALT, "NOP", AddressingMode::ZeroPage_X),
        OpCode::new(0xE2, Instruction::NOP_ALT, "NOP", AddressingMode::Immediate),
        OpCode::new(0x07, Instruction::SLO, "SLO", AddressingMode::ZeroPage),
        OpCode::new(0x17, Instruction::SLO, "SLO", AddressingMode::ZeroPage_X),
        OpCode::new(0x03, Instruction::SLO, "SLO", AddressingMode::Indirect_X),
        OpCode::new(0x13, Instruction::SLO, "SLO", AddressingMode::Indirect_Y),
        OpCode::new(0x0F, Instruction::SLO, "SLO", AddressingMode::Absolute),
        OpCode::new(0x1F, Instruction::SLO, "SLO", AddressingMode::Absolute_X),
        OpCode::new(0x1B, Instruction::SLO, "SLO", AddressingMode::Absolute_Y),
        OpCode::new(0x27, Instruction::RLA, "RLA", AddressingMode::ZeroPage),
        OpCode::new(0x37, Instruction::RLA, "RLA", AddressingMode::ZeroPage_X),
        OpCode::new(0x23, Instruction::RLA, "RLA", AddressingMode::Indirect_X),
        OpCode::new(0x33, Instruction::RLA, "RLA", AddressingMode::Indirect_Y),
        OpCode::new(0x2F, Instruction::RLA, "RLA", AddressingMode::Absolute),
        OpCode::new(0x3F, Instruction::RLA, "RLA", AddressingMode::Absolute_X),
        OpCode::new(0x3B, Instruction::RLA, "RLA", AddressingMode::Absolute_Y),
        OpCode::new(0x47, Instruction::SRE, "SRE", AddressingMode::ZeroPage),
        OpCode::new(0x57, Instruction::SRE, "SRE", AddressingMode::ZeroPage_X),
        OpCode::new(0x43, Instruction::SRE, "SRE", AddressingMode::Indirect_X),
        OpCode::new(0x53, Instruction::SRE, "SRE", AddressingMode::Indirect_Y),
        OpCode::new(0x4F, Instruction::SRE, "SRE", AddressingMode::Absolute),
        OpCode::new(0x5F, Instruction::SRE, "SRE", AddressingMode::Absolute_X),
        OpCode::new(0x5B, Instruction::SRE, "SRE", AddressingMode::Absolute_Y),
        OpCode::new(0x67, Instruction::RRA, "RRA", AddressingMode::ZeroPage),
        OpCode::new(0x77, Instruction::RRA, "RRA", AddressingMode::ZeroPage_X),
        OpCode::new(0x63, Instruction::RRA, "RRA", AddressingMode::Indirect_X),
        OpCode::new(0x73, Instruction::RRA, "RRA", AddressingMode::Indirect_Y),
        OpCode::new(0x6F, Instruction::RRA, "RRA", AddressingMode::Absolute),
        OpCode::new(0x7F, Instruction::RRA, "RRA", AddressingMode::Absolute_X),
        OpCode::new(0x7B, Instruction::RRA, "RRA", AddressingMode::Absolute_Y),
        OpCode::new(0x87, Instruction::SAX, "SAX", AddressingMode::ZeroPage),
        OpCode::new(0x97, Instruction::SAX, "SAX", AddressingMode::ZeroPage_Y),
        OpCode::new(0x83, Instruction::SAX, "SAX", AddressingMode::Indirect_X),
        OpCode::new(0x8F, Instruction::SAX, "SAX", AddressingMode::Absolute),
        OpCode::new(0xAB, Instruction::LAX, "LAX", AddressingMode::Immediate),
        OpCode::new(0xA7, Instruction::LAX, "LAX", AddressingMode::ZeroPage),
        OpCode::new(0xB7, Instruction::LAX, "LAX", AddressingMode::ZeroPage_Y),
        OpCode::new(0xA3, Instruction::LAX, "LAX", AddressingMode::Indirect_X),
        OpCode::new(0xB3, Instruction::LAX, "LAX", AddressingMode::Indirect_Y),
        OpCode::new(0xAF, Instruction::LAX, "LAX", AddressingMode::Absolute),
        OpCode::new(0xBF, Instruction::LAX, "LAX", AddressingMode::Absolute_Y),
        OpCode::new(0xC7, Instruction::DCP, "DCP", AddressingMode::ZeroPage),
        OpCode::new(0xD7, Instruction::DCP, "DCP", AddressingMode::ZeroPage_X),
        OpCode::new(0xC3, Instruction::DCP, "DCP", AddressingMode::Indirect_X),
        OpCode::new(0xD3, Instruction::DCP, "DCP", AddressingMode::Indirect_Y),
        OpCode::new(0xCF, Instruction::DCP, "DCP", AddressingMode::Absolute),
        OpCode::new(0xDF, Instruction::DCP, "DCP", AddressingMode::Absolute_X),
        OpCode::new(0xDB, Instruction::DCP, "DCP", AddressingMode::Absolute_Y),
        OpCode::new(0xE7, Instruction::ISC, "ISC", AddressingMode::ZeroPage),
        OpCode::new(0xF7, Instruction::ISC, "ISC", AddressingMode::ZeroPage_X),
        OpCode::new(0xE3, Instruction::ISC, "ISC", AddressingMode::Indirect_X),
        OpCode::new(0xF3, Instruction::ISC, "ISC", AddressingMode::Indirect_Y),
        OpCode::new(0xEF, Instruction::ISC, "ISC", AddressingMode::Absolute),
        OpCode::new(0xFF, Instruction::ISC, "ISC", AddressingMode::Absolute_X),
        OpCode::new(0xFB, Instruction::ISC, "ISC", AddressingMode::Absolute_Y),
        OpCode::new(0x2B, Instruction::ANC, "ANC", AddressingMode::Immediate),
        OpCode::new(0x4B, Instruction::ALR, "ALR", AddressingMode::Immediate),
        OpCode::new(0x6B, Instruction::ARR, "ARR", AddressingMode::Immediate),
        OpCode::new(0x8B, Instruction::XAA, "XAA", AddressingMode::Immediate),
        OpCode::new(0xCB, Instruction::AXS, "AXS", AddressingMode::Immediate),
        OpCode::new(0xEB, Instruction::SBC_NOP, "SBC", AddressingMode::Immediate),
        OpCode::new(0x93, Instruction::AHX, "AHX", AddressingMode::Indirect_Y),
        OpCode::new(0x9F, Instruction::AHX, "AHX", AddressingMode::Absolute_Y),
        OpCode::new(0x9C, Instruction::SHY, "SHY", AddressingMode::Absolute_X),
        OpCode::new(0x9E, Instruction::SHX, "SHX", AddressingMode::Absolute_Y),
        OpCode::new(0x9B, Instruction::TAS, "TAS", AddressingMode::Absolute_Y),
        OpCode::new(0xBB, Instruction::LAS, "LAS", AddressingMode::Absolute_Y),
    ];

    let mut map = HashMap::new();
    for opcode in opcodes_vec {
        map.insert(opcode.byte, opcode);
    }

    trace!("Finished building OPCODES hashmap");
    map
});
