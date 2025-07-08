use crate::cpu::addr::AddressingMode;
use crate::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OpCode {
    pub byte: u8,
    pub instruction: Instruction,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        byte: u8,
        instruction: Instruction,
        mnemonic: &'static str,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            byte,
            instruction,
            mnemonic,
            len: mode.bytes(),
            cycles,
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
    
    // ===== Register Transfers =====
    /// Transfer Accumulator to X
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
    // https:///www.oxyron.de/html/opcodes02.html
    // https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    NOP_ALT,

    // ===== Illegal Opcodes =====
    // https:///www.oxyron.de/html/opcodes02.html
    // https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
    // https:///www.nesdev.org/wiki/Programming_with_unofficial_opcodes
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
        // ===== Load/Store Operations =====
        OpCode::new(0xA9, Instruction::LDA, "LDA", 0, AddressingMode::Immediate),
        OpCode::new(0xA5, Instruction::LDA, "LDA", 0, AddressingMode::ZeroPage),
        OpCode::new(0xB5, Instruction::LDA, "LDA", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, Instruction::LDA, "LDA", 0, AddressingMode::Absolute),
        OpCode::new(0xBD, Instruction::LDA, "LDA", 0, AddressingMode::Absolute_X),
        OpCode::new(0xB9, Instruction::LDA, "LDA", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xA1, Instruction::LDA, "LDA", 0, AddressingMode::Indirect_X),
        OpCode::new(0xB1, Instruction::LDA, "LDA", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xA2, Instruction::LDX, "LDX", 0, AddressingMode::Immediate),
        OpCode::new(0xA6, Instruction::LDX, "LDX", 0, AddressingMode::ZeroPage),
        OpCode::new(0xB6, Instruction::LDX, "LDX", 0, AddressingMode::ZeroPage_Y),
        OpCode::new(0xAE, Instruction::LDX, "LDX", 0, AddressingMode::Absolute),
        OpCode::new(0xBE, Instruction::LDX, "LDX", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xA0, Instruction::LDY, "LDY", 0, AddressingMode::Immediate),
        OpCode::new(0xA4, Instruction::LDY, "LDY", 0, AddressingMode::ZeroPage),
        OpCode::new(0xB4, Instruction::LDY, "LDY", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xAC, Instruction::LDY, "LDY", 0, AddressingMode::Absolute),
        OpCode::new(0xBC, Instruction::LDY, "LDY", 0, AddressingMode::Absolute_X),
        OpCode::new(0x85, Instruction::STA, "STA", 0, AddressingMode::ZeroPage),
        OpCode::new(0x95, Instruction::STA, "STA", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, Instruction::STA, "STA", 0, AddressingMode::Absolute),
        OpCode::new(0x9D, Instruction::STA, "STA", 0, AddressingMode::Absolute_X),
        OpCode::new(0x99, Instruction::STA, "STA", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x81, Instruction::STA, "STA", 0, AddressingMode::Indirect_X),
        OpCode::new(0x91, Instruction::STA, "STA", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x86, Instruction::STX, "STX", 0, AddressingMode::ZeroPage),
        OpCode::new(0x96, Instruction::STX, "STX", 0, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8E, Instruction::STX, "STX", 0, AddressingMode::Absolute),
        OpCode::new(0x84, Instruction::STY, "STY", 0, AddressingMode::ZeroPage),
        OpCode::new(0x94, Instruction::STY, "STY", 0, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8C, Instruction::STY, "STY", 0, AddressingMode::Absolute),
        
        // ===== Register Transfers =====
        OpCode::new(0xAA, Instruction::TAX, "TAX", 0, AddressingMode::Implicit),
        OpCode::new(0xA8, Instruction::TAY, "TAY", 0, AddressingMode::Implicit),
        OpCode::new(0x8A, Instruction::TXA, "TXA", 0, AddressingMode::Implicit),
        OpCode::new(0x98, Instruction::TYA, "TYA", 0, AddressingMode::Implicit),
        OpCode::new(0xBA, Instruction::TSX, "TSX", 0, AddressingMode::Implicit),
        OpCode::new(0x9A, Instruction::TXS, "TXS", 0, AddressingMode::Implicit),
        
        // ===== Stack Operations =====
        OpCode::new(0x48, Instruction::PHA, "PHA", 0, AddressingMode::Implicit),
        OpCode::new(0x08, Instruction::PHP, "PHP", 0, AddressingMode::Implicit),
        OpCode::new(0x68, Instruction::PLA, "PLA", 0, AddressingMode::Implicit),
        OpCode::new(0x28, Instruction::PLP, "PLP", 0, AddressingMode::Implicit),
        
        // ===== Logical =====
        OpCode::new(0x29, Instruction::AND, "AND", 0, AddressingMode::Immediate),
        OpCode::new(0x25, Instruction::AND, "AND", 0, AddressingMode::ZeroPage),
        OpCode::new(0x35, Instruction::AND, "AND", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x2D, Instruction::AND, "AND", 0, AddressingMode::Absolute),
        OpCode::new(0x3D, Instruction::AND, "AND", 0, AddressingMode::Absolute_X),
        OpCode::new(0x39, Instruction::AND, "AND", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x21, Instruction::AND, "AND", 0, AddressingMode::Indirect_X),
        OpCode::new(0x31, Instruction::AND, "AND", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x49, Instruction::EOR, "EOR", 0, AddressingMode::Immediate),
        OpCode::new(0x45, Instruction::EOR, "EOR", 0, AddressingMode::ZeroPage),
        OpCode::new(0x55, Instruction::EOR, "EOR", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x4D, Instruction::EOR, "EOR", 0, AddressingMode::Absolute),
        OpCode::new(0x5D, Instruction::EOR, "EOR", 0, AddressingMode::Absolute_X),
        OpCode::new(0x59, Instruction::EOR, "EOR", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x41, Instruction::EOR, "EOR", 0, AddressingMode::Indirect_X),
        OpCode::new(0x51, Instruction::EOR, "EOR", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x09, Instruction::ORA, "ORA", 0, AddressingMode::Immediate),
        OpCode::new(0x05, Instruction::ORA, "ORA", 0, AddressingMode::ZeroPage),
        OpCode::new(0x15, Instruction::ORA, "ORA", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x0D, Instruction::ORA, "ORA", 0, AddressingMode::Absolute),
        OpCode::new(0x1D, Instruction::ORA, "ORA", 0, AddressingMode::Absolute_X),
        OpCode::new(0x19, Instruction::ORA, "ORA", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x01, Instruction::ORA, "ORA", 0, AddressingMode::Indirect_X),
        OpCode::new(0x11, Instruction::ORA, "ORA", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x24, Instruction::BIT, "BIT", 0, AddressingMode::ZeroPage),
        OpCode::new(0x2C, Instruction::BIT, "BIT", 0, AddressingMode::Absolute),
        
        // ===== Arithmetic =====
        OpCode::new(0x69, Instruction::ADC, "ADC", 2, AddressingMode::Immediate),
        OpCode::new(0x65, Instruction::ADC, "ADC", 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, Instruction::ADC, "ADC", 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6D, Instruction::ADC, "ADC", 4, AddressingMode::Absolute),
        OpCode::new(0x7D, Instruction::ADC, "ADC", 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, Instruction::ADC, "ADC", 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, Instruction::ADC, "ADC", 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, Instruction::ADC, "ADC", 5, AddressingMode::Indirect_Y),
        OpCode::new(0xE9, Instruction::SBC, "SBC", 0, AddressingMode::Immediate),
        OpCode::new(0xE5, Instruction::SBC, "SBC", 0, AddressingMode::ZeroPage),
        OpCode::new(0xF5, Instruction::SBC, "SBC", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xED, Instruction::SBC, "SBC", 0, AddressingMode::Absolute),
        OpCode::new(0xFD, Instruction::SBC, "SBC", 0, AddressingMode::Absolute_X),
        OpCode::new(0xF9, Instruction::SBC, "SBC", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xE1, Instruction::SBC, "SBC", 0, AddressingMode::Indirect_X),
        OpCode::new(0xF1, Instruction::SBC, "SBC", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xC9, Instruction::CMP, "CMP", 0, AddressingMode::Immediate),
        OpCode::new(0xC5, Instruction::CMP, "CMP", 0, AddressingMode::ZeroPage),
        OpCode::new(0xD5, Instruction::CMP, "CMP", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xCD, Instruction::CMP, "CMP", 0, AddressingMode::Absolute),
        OpCode::new(0xDD, Instruction::CMP, "CMP", 0, AddressingMode::Absolute_X),
        OpCode::new(0xD9, Instruction::CMP, "CMP", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xC1, Instruction::CMP, "CMP", 0, AddressingMode::Indirect_X),
        OpCode::new(0xD1, Instruction::CMP, "CMP", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xE0, Instruction::CPX, "CPX", 0, AddressingMode::Immediate),
        OpCode::new(0xE4, Instruction::CPX, "CPX", 0, AddressingMode::ZeroPage),
        OpCode::new(0xEC, Instruction::CPX, "CPX", 0, AddressingMode::Absolute),
        OpCode::new(0xC0, Instruction::CPY, "CPY", 0, AddressingMode::Immediate),
        OpCode::new(0xC4, Instruction::CPY, "CPY", 0, AddressingMode::ZeroPage),
        OpCode::new(0xCC, Instruction::CPY, "CPY", 0, AddressingMode::Absolute),
        
        // ===== Increments & Decrements =====
        OpCode::new(0xE6, Instruction::INC, "INC", 0, AddressingMode::ZeroPage),
        OpCode::new(0xF6, Instruction::INC, "INC", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xEE, Instruction::INC, "INC", 0, AddressingMode::Absolute),
        OpCode::new(0xFE, Instruction::INC, "INC", 0, AddressingMode::Absolute_X),
        OpCode::new(0xE8, Instruction::INX, "INX", 0, AddressingMode::Implicit),
        OpCode::new(0xC8, Instruction::INY, "INY", 0, AddressingMode::Implicit),
        OpCode::new(0xC6, Instruction::DEC, "DEC", 0, AddressingMode::ZeroPage),
        OpCode::new(0xD6, Instruction::DEC, "DEC", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xCE, Instruction::DEC, "DEC", 0, AddressingMode::Absolute),
        OpCode::new(0xDE, Instruction::DEC, "DEC", 0, AddressingMode::Absolute_X),
        OpCode::new(0xCA, Instruction::DEX, "DEX", 0, AddressingMode::Implicit),
        OpCode::new(0x88, Instruction::DEY, "DEY", 0, AddressingMode::Implicit),
        
        // ===== Shifts =====
        OpCode::new(0x0A, Instruction::ASL, "ASL", 0, AddressingMode::Accumulator),
        OpCode::new(0x06, Instruction::ASL, "ASL", 0, AddressingMode::ZeroPage),
        OpCode::new(0x16, Instruction::ASL, "ASL", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x0E, Instruction::ASL, "ASL", 0, AddressingMode::Absolute),
        OpCode::new(0x1E, Instruction::ASL, "ASL", 0, AddressingMode::Absolute_X),
        OpCode::new(0x4A, Instruction::LSR, "LSR", 0, AddressingMode::Accumulator),
        OpCode::new(0x46, Instruction::LSR, "LSR", 0, AddressingMode::ZeroPage),
        OpCode::new(0x56, Instruction::LSR, "LSR", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x4E, Instruction::LSR, "LSR", 0, AddressingMode::Absolute),
        OpCode::new(0x5E, Instruction::LSR, "LSR", 0, AddressingMode::Absolute_X),
        OpCode::new(0x2A, Instruction::ROL, "ROL", 0, AddressingMode::Accumulator),
        OpCode::new(0x26, Instruction::ROL, "ROL", 0, AddressingMode::ZeroPage),
        OpCode::new(0x36, Instruction::ROL, "ROL", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x2E, Instruction::ROL, "ROL", 0, AddressingMode::Absolute),
        OpCode::new(0x3E, Instruction::ROL, "ROL", 0, AddressingMode::Absolute_X),
        OpCode::new(0x6A, Instruction::ROR, "ROR", 0, AddressingMode::Accumulator),
        OpCode::new(0x66, Instruction::ROR, "ROR", 0, AddressingMode::ZeroPage),
        OpCode::new(0x76, Instruction::ROR, "ROR", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x6E, Instruction::ROR, "ROR", 0, AddressingMode::Absolute),
        OpCode::new(0x7E, Instruction::ROR, "ROR", 0, AddressingMode::Absolute_X),
        
        // ===== Jumps & Calls =====
        OpCode::new(0x4C, Instruction::JMP, "JMP", 0, AddressingMode::Absolute),
        OpCode::new(0x6C, Instruction::JMP, "JMP", 0, AddressingMode::Indirect),
        OpCode::new(0x20, Instruction::JSR, "JSR", 0, AddressingMode::Absolute),
        OpCode::new(0x60, Instruction::RTS, "RTS", 0, AddressingMode::Implicit),
        
        // ===== Branches =====
        OpCode::new(0x90, Instruction::BCC, "BCC", 0, AddressingMode::Relative),
        OpCode::new(0xB0, Instruction::BCS, "BCS", 0, AddressingMode::Relative),
        OpCode::new(0xF0, Instruction::BEQ, "BEQ", 0, AddressingMode::Relative),
        OpCode::new(0x30, Instruction::BMI, "BMI", 0, AddressingMode::Relative),
        OpCode::new(0xD0, Instruction::BNE, "BNE", 0, AddressingMode::Relative),
        OpCode::new(0x10, Instruction::BPL, "BPL", 0, AddressingMode::Relative),
        OpCode::new(0x50, Instruction::BVC, "BVC", 0, AddressingMode::Relative),
        OpCode::new(0x70, Instruction::BVS, "BVS", 0, AddressingMode::Relative),
        
        // ===== Status Flag Changes =====
        OpCode::new(0x18, Instruction::CLC, "CLC", 0, AddressingMode::Implicit),
        OpCode::new(0xD8, Instruction::CLD, "CLD", 0, AddressingMode::Implicit),
        OpCode::new(0x58, Instruction::CLI, "CLI", 0, AddressingMode::Implicit),
        OpCode::new(0xB8, Instruction::CLV, "CLV", 0, AddressingMode::Implicit),
        OpCode::new(0x38, Instruction::SEC, "SEC", 0, AddressingMode::Implicit),
        OpCode::new(0xF8, Instruction::SED, "SED", 0, AddressingMode::Implicit),
        OpCode::new(0x78, Instruction::SEI, "SEI", 0, AddressingMode::Implicit),
        
        // ===== System Functions =====
        OpCode::new(0x00, Instruction::BRK, "BRK", 7, AddressingMode::Implicit),
        OpCode::new(0xEA, Instruction::NOP, "NOP", 2, AddressingMode::Implicit),
        OpCode::new(0x40, Instruction::RTI, "RTI", 0, AddressingMode::Implicit),
        
        // ===== Undocumented Opcodes =====
        OpCode::new(0x1A, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
        OpCode::new(0xFA, Instruction::NOP_ALT, "NOP", 0, AddressingMode::Implicit),
        OpCode::new(0x0C, Instruction::NOP_ALT, "NOP", 0, AddressingMode::Absolute),
        OpCode::new(0xFC, Instruction::NOP_ALT, "NOP", 0, AddressingMode::Absolute_X),
        OpCode::new(0x64, Instruction::NOP_ALT, "NOP", 0, AddressingMode::ZeroPage),
        OpCode::new(0x04, Instruction::NOP_ALT, "NOP", 3, AddressingMode::ZeroPage),
        OpCode::new(0x14, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xF4, Instruction::NOP_ALT, "NOP", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xE2, Instruction::NOP_ALT, "NOP", 0, AddressingMode::Immediate),
        
        // ===== Illegal Opcodes =====
        OpCode::new(0x07, Instruction::SLO, "SLO", 0, AddressingMode::ZeroPage),
        OpCode::new(0x17, Instruction::SLO, "SLO", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x03, Instruction::SLO, "SLO", 0, AddressingMode::Indirect_X),
        OpCode::new(0x13, Instruction::SLO, "SLO", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x0F, Instruction::SLO, "SLO", 0, AddressingMode::Absolute),
        OpCode::new(0x1F, Instruction::SLO, "SLO", 0, AddressingMode::Absolute_X),
        OpCode::new(0x1B, Instruction::SLO, "SLO", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x27, Instruction::RLA, "RLA", 0, AddressingMode::ZeroPage),
        OpCode::new(0x37, Instruction::RLA, "RLA", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x23, Instruction::RLA, "RLA", 0, AddressingMode::Indirect_X),
        OpCode::new(0x33, Instruction::RLA, "RLA", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x2F, Instruction::RLA, "RLA", 0, AddressingMode::Absolute),
        OpCode::new(0x3F, Instruction::RLA, "RLA", 0, AddressingMode::Absolute_X),
        OpCode::new(0x3B, Instruction::RLA, "RLA", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x47, Instruction::SRE, "SRE", 0, AddressingMode::ZeroPage),
        OpCode::new(0x57, Instruction::SRE, "SRE", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x43, Instruction::SRE, "SRE", 0, AddressingMode::Indirect_X),
        OpCode::new(0x53, Instruction::SRE, "SRE", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x4F, Instruction::SRE, "SRE", 0, AddressingMode::Absolute),
        OpCode::new(0x5F, Instruction::SRE, "SRE", 0, AddressingMode::Absolute_X),
        OpCode::new(0x5B, Instruction::SRE, "SRE", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x67, Instruction::RRA, "RRA", 0, AddressingMode::ZeroPage),
        OpCode::new(0x77, Instruction::RRA, "RRA", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0x63, Instruction::RRA, "RRA", 0, AddressingMode::Indirect_X),
        OpCode::new(0x73, Instruction::RRA, "RRA", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x6F, Instruction::RRA, "RRA", 0, AddressingMode::Absolute),
        OpCode::new(0x7F, Instruction::RRA, "RRA", 0, AddressingMode::Absolute_X),
        OpCode::new(0x7B, Instruction::RRA, "RRA", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x87, Instruction::SAX, "SAX", 0, AddressingMode::ZeroPage),
        OpCode::new(0x97, Instruction::SAX, "SAX", 0, AddressingMode::ZeroPage_Y),
        OpCode::new(0x83, Instruction::SAX, "SAX", 0, AddressingMode::Indirect_X),
        OpCode::new(0x8F, Instruction::SAX, "SAX", 0, AddressingMode::Absolute),
        OpCode::new(0xAB, Instruction::LAX, "LAX", 0, AddressingMode::Immediate),
        OpCode::new(0xA7, Instruction::LAX, "LAX", 0, AddressingMode::ZeroPage),
        OpCode::new(0xB7, Instruction::LAX, "LAX", 0, AddressingMode::ZeroPage_Y),
        OpCode::new(0xA3, Instruction::LAX, "LAX", 0, AddressingMode::Indirect_X),
        OpCode::new(0xB3, Instruction::LAX, "LAX", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xAF, Instruction::LAX, "LAX", 0, AddressingMode::Absolute),
        OpCode::new(0xBF, Instruction::LAX, "LAX", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xC7, Instruction::DCP, "DCP", 0, AddressingMode::ZeroPage),
        OpCode::new(0xD7, Instruction::DCP, "DCP", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xC3, Instruction::DCP, "DCP", 0, AddressingMode::Indirect_X),
        OpCode::new(0xD3, Instruction::DCP, "DCP", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xCF, Instruction::DCP, "DCP", 0, AddressingMode::Absolute),
        OpCode::new(0xDF, Instruction::DCP, "DCP", 0, AddressingMode::Absolute_X),
        OpCode::new(0xDB, Instruction::DCP, "DCP", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xE7, Instruction::ISC, "ISC", 0, AddressingMode::ZeroPage),
        OpCode::new(0xF7, Instruction::ISC, "ISC", 0, AddressingMode::ZeroPage_X),
        OpCode::new(0xE3, Instruction::ISC, "ISC", 0, AddressingMode::Indirect_X),
        OpCode::new(0xF3, Instruction::ISC, "ISC", 0, AddressingMode::Indirect_Y),
        OpCode::new(0xEF, Instruction::ISC, "ISC", 0, AddressingMode::Absolute),
        OpCode::new(0xFF, Instruction::ISC, "ISC", 0, AddressingMode::Absolute_X),
        OpCode::new(0xFB, Instruction::ISC, "ISC", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x0B, Instruction::ANC, "ANC", 2, AddressingMode::Immediate),
        OpCode::new(0x2B, Instruction::ANC, "ANC", 0, AddressingMode::Immediate),
        OpCode::new(0x4B, Instruction::ALR, "ALR", 0, AddressingMode::Immediate),
        OpCode::new(0x6B, Instruction::ARR, "ARR", 0, AddressingMode::Immediate),
        OpCode::new(0x8B, Instruction::XAA, "XAA", 0, AddressingMode::Immediate),
        OpCode::new(0xCB, Instruction::AXS, "AXS", 0, AddressingMode::Immediate),
        OpCode::new(0xEB, Instruction::SBC_NOP, "SBC", 0, AddressingMode::Immediate),
        OpCode::new(0x93, Instruction::AHX, "AHX", 0, AddressingMode::Indirect_Y),
        OpCode::new(0x9F, Instruction::AHX, "AHX", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x9C, Instruction::SHY, "SHY", 0, AddressingMode::Absolute_X),
        OpCode::new(0x9E, Instruction::SHX, "SHX", 0, AddressingMode::Absolute_Y),
        OpCode::new(0x9B, Instruction::TAS, "TAS", 0, AddressingMode::Absolute_Y),
        OpCode::new(0xBB, Instruction::LAS, "LAS", 0, AddressingMode::Absolute_Y),
    ];

    let mut map = HashMap::new();
    for opcode in opcodes_vec {
        map.insert(opcode.byte, opcode);
    }

    trace!("Finished building OPCODES hashmap");
    map
});

pub fn test_opcodes() {
    info!("{} opcodes defined", OPCODES.len());
    
    let mut missing_opcodes: Vec<u8> = Vec::new();
    let mut no_cycle_opcodes: Vec<OpCode> = Vec::new();
    for byte in 0..256 {
        match OPCODES.get(&(byte as u8)) {
            Some(opcode) => {
                if opcode.cycles <= 0 {
                    no_cycle_opcodes.push(*opcode);
                }
            },
            None => missing_opcodes.push(byte as u8),
        }
    }
    
    if missing_opcodes.len() > 0 {
        error!("{} missing opcodes", missing_opcodes.len());
        error!("[{}]", missing_opcodes
            .iter()
            .map(|b| format!("{:#04X}", b))
            .collect::<Vec<_>>()
            .join(", ")
        );
    }
    if no_cycle_opcodes.len() > 0 {
        error!("{} no cycle opcodes", no_cycle_opcodes.len());
        error!("[{}]", no_cycle_opcodes
            .iter()
            .map(|op| format!("{:?} ({:#04X})", op.instruction, op.byte))
            .collect::<Vec<_>>()
            .join(", ")
        );
    }
    std::process::exit(0);
}