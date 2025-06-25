// http://www.6502.org/users/obelisk/6502/instructions.html
#[derive(Copy, Clone, Debug)]
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

    // ===== Increment & Decrements =====
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
}

// http://www.6502.org/users/obelisk/6502/addressing.html
#[derive(Copy, Clone, Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // aka IndexedIndirect
    IndirectY, // aka IndirectIndexed
}

impl AddressingMode {
    pub const fn extra_bytes(self) -> u16 {
        match self {
            AddressingMode::Accumulator => 0,
            AddressingMode::Implicit => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::ZeroPage => 1,
            AddressingMode::ZeroPageX => 1,
            AddressingMode::ZeroPageY => 1,
            AddressingMode::Relative => 1,
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
            AddressingMode::IndirectX => 1,
            AddressingMode::IndirectY => 1,
        }
    }
}

// http://www.6502.org/users/obelisk/6502/reference.html
pub fn decode_opcode(opcode: u8) -> (Instruction, AddressingMode) {
    match opcode {
        _ => panic!("UNIMPLEMENTED OPCODE: {:02x}", opcode)
    }
}
