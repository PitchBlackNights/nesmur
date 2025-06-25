import re

old_instr = """// ===== Load/Store Operations =====
/// Load Accumulator
0xA9 => (Instruction::LDA, AddressingMode::Immediate),
0xA5 => (Instruction::LDA, AddressingMode::ZeroPage),
0xB5 => (Instruction::LDA, AddressingMode::ZeroPage_X),
0xAD => (Instruction::LDA, AddressingMode::Absolute),
0xBD => (Instruction::LDA, AddressingMode::Absolute_X),
0xB9 => (Instruction::LDA, AddressingMode::Absolute_Y),
0xA1 => (Instruction::LDA, AddressingMode::Indirect_X),
0xB1 => (Instruction::LDA, AddressingMode::Indirect_Y),
/// Load X register
0xA2 => (Instruction::LDX, AddressingMode::Immediate),
0xA6 => (Instruction::LDX, AddressingMode::ZeroPage),
0xB6 => (Instruction::LDX, AddressingMode::ZeroPage_Y),
0xAE => (Instruction::LDX, AddressingMode::Absolute),
0xBE => (Instruction::LDX, AddressingMode::Absolute_Y),
/// Load Y register
0xA0 => (Instruction::LDY, AddressingMode::Immediate),
0xA4 => (Instruction::LDY, AddressingMode::ZeroPage),
0xB4 => (Instruction::LDY, AddressingMode::ZeroPage_X),
0xAC => (Instruction::LDY, AddressingMode::Absolute),
0xBC => (Instruction::LDY, AddressingMode::Absolute_X),
/// Store Accumulator
0x85 => (Instruction::STA, AddressingMode::ZeroPage),
0x95 => (Instruction::STA, AddressingMode::ZeroPage_X),
0x8D => (Instruction::STA, AddressingMode::Absolute),
0x9D => (Instruction::STA, AddressingMode::Absolute_X),
0x99 => (Instruction::STA, AddressingMode::Absolute_Y),
0x81 => (Instruction::STA, AddressingMode::Indirect_X),
0x91 => (Instruction::STA, AddressingMode::Indirect_Y),
/// Store X register
0x86 => (Instruction::STX, AddressingMode::ZeroPage),
0x96 => (Instruction::STX, AddressingMode::ZeroPage_Y),
0x8E => (Instruction::STX, AddressingMode::Absolute),
/// Store Y register
0x84 => (Instruction::STY, AddressingMode::ZeroPage),
0x94 => (Instruction::STY, AddressingMode::ZeroPage_Y),
0x8C => (Instruction::STY, AddressingMode::Absolute),

// ===== Register Transfers =====
/// Transfer Accumulator to X
0xAA => (Instruction::TAX, AddressingMode::Implicit),
/// Transfer Accumulator to Y
0xA8 => (Instruction::TAY, AddressingMode::Implicit),
/// Transfer X to Accumulator
0x8A => (Instruction::TXA, AddressingMode::Implicit),
/// Transfer Y to Accumulator
0x98 => (Instruction::TYA, AddressingMode::Implicit),

// ===== Stack Operations =====
/// Transfer Stack Pointer to X
0xBA => (Instruction::TSX, AddressingMode::Implicit),
/// Transfer X to Stack Pointer
0x9A => (Instruction::TXS, AddressingMode::Implicit),
/// Push Accumulator on Stack
0x48 => (Instruction::PHA, AddressingMode::Implicit),
/// Push Processor Status on Stack
0x08 => (Instruction::PHP, AddressingMode::Implicit),
/// Pull Accumulator from Stack
0x68 => (Instruction::PLA, AddressingMode::Implicit),
/// Pull Processor Status from Stack
0x28 => (Instruction::PLP, AddressingMode::Implicit),

// ===== Logical =====
/// Logical AND
0x29 => (Instruction::AND, AddressingMode::Immediate),
0x25 => (Instruction::AND, AddressingMode::ZeroPage),
0x35 => (Instruction::AND, AddressingMode::ZeroPage_X),
0x2D => (Instruction::AND, AddressingMode::Absolute),
0x3D => (Instruction::AND, AddressingMode::Absolute_X),
0x39 => (Instruction::AND, AddressingMode::Absolute_Y),
0x21 => (Instruction::AND, AddressingMode::Indirect_X),
0x31 => (Instruction::AND, AddressingMode::Indirect_Y),
/// Exclusive OR
0x49 => (Instruction::EOR, AddressingMode::Immediate),
0x45 => (Instruction::EOR, AddressingMode::ZeroPage),
0x55 => (Instruction::EOR, AddressingMode::ZeroPage_X),
0x4D => (Instruction::EOR, AddressingMode::Absolute),
0x5D => (Instruction::EOR, AddressingMode::Absolute_X),
0x59 => (Instruction::EOR, AddressingMode::Absolute_Y),
0x41 => (Instruction::EOR, AddressingMode::Indirect_X),
0x51 => (Instruction::EOR, AddressingMode::Indirect_Y),
/// Logical Inclusive OR
0x09 => (Instruction::ORA, AddressingMode::Immediate),
0x05 => (Instruction::ORA, AddressingMode::ZeroPage),
0x15 => (Instruction::ORA, AddressingMode::ZeroPage_X),
0x0D => (Instruction::ORA, AddressingMode::Absolute),
0x1D => (Instruction::ORA, AddressingMode::Absolute_X),
0x19 => (Instruction::ORA, AddressingMode::Absolute_Y),
0x01 => (Instruction::ORA, AddressingMode::Indirect_X),
0x11 => (Instruction::ORA, AddressingMode::Indirect_Y),
/// Bit Test
0x24 => (Instruction::BIT, AddressingMode::ZeroPage),
0x2C => (Instruction::BIT, AddressingMode::Absolute),

// ===== Arithmetic =====
/// Add with Carry
0x69 => (Instruction::ADC, AddressingMode::Immediate),
0x65 => (Instruction::ADC, AddressingMode::ZeroPage),
0x75 => (Instruction::ADC, AddressingMode::ZeroPage_X),
0x6D => (Instruction::ADC, AddressingMode::Absolute),
0x7D => (Instruction::ADC, AddressingMode::Absolute_X),
0x79 => (Instruction::ADC, AddressingMode::Absolute_Y),
0x61 => (Instruction::ADC, AddressingMode::Indirect_X),
0x71 => (Instruction::ADC, AddressingMode::Indirect_Y),
/// Subtract with Carry
0xE9 => (Instruction::SBC, AddressingMode::Immediate),
0xE5 => (Instruction::SBC, AddressingMode::ZeroPage),
0xF5 => (Instruction::SBC, AddressingMode::ZeroPage_X),
0xED => (Instruction::SBC, AddressingMode::Absolute),
0xFD => (Instruction::SBC, AddressingMode::Absolute_X),
0xF9 => (Instruction::SBC, AddressingMode::Absolute_Y),
0xE1 => (Instruction::SBC, AddressingMode::Indirect_X),
0xF1 => (Instruction::SBC, AddressingMode::Indirect_Y),
/// Compare Accumulator
0xC9 => (Instruction::CMP, AddressingMode::Immediate),
0xC5 => (Instruction::CMP, AddressingMode::ZeroPage),
0xD5 => (Instruction::CMP, AddressingMode::ZeroPage_X),
0xCD => (Instruction::CMP, AddressingMode::Absolute),
0xDD => (Instruction::CMP, AddressingMode::Absolute_X),
0xD9 => (Instruction::CMP, AddressingMode::Absolute_Y),
0xC1 => (Instruction::CMP, AddressingMode::Indirect_X),
0xD1 => (Instruction::CMP, AddressingMode::Indirect_Y),
/// Compare X register
0xE0 => (Instruction::CPX, AddressingMode::Immediate),
0xE4 => (Instruction::CPX, AddressingMode::ZeroPage),
0xEC => (Instruction::CPX, AddressingMode::Absolute),
/// Compare Y register
0xC0 => (Instruction::CPY, AddressingMode::Immediate),
0xC4 => (Instruction::CPY, AddressingMode::ZeroPage),
0xCC => (Instruction::CPY, AddressingMode::Absolute),

// ===== Increment & Decrements =====
/// Increment a memory location
0xE6 => (Instruction::INC, AddressingMode::ZeroPage),
0xF6 => (Instruction::INC, AddressingMode::ZeroPage_X),
0xEE => (Instruction::INC, AddressingMode::Absolute),
0xFE => (Instruction::INC, AddressingMode::Absolute_X),
/// Increment the X register
0xE8 => (Instruction::INX, AddressingMode::Implicit),
/// Increment the Y register
0xC8 => (Instruction::INY, AddressingMode::Implicit),
/// Decrement a memory location
0xC6 => (Instruction::DEC, AddressingMode::ZeroPage),
0xD6 => (Instruction::DEC, AddressingMode::ZeroPage_X),
0xCE => (Instruction::DEC, AddressingMode::Absolute),
0xDE => (Instruction::DEC, AddressingMode::Absolute_X),
/// Decrement the X register
0xCA => (Instruction::DEX, AddressingMode::Implicit),
/// Decrement the Y register
0x88 => (Instruction::DEY, AddressingMode::Implicit),

// ===== Shifts =====
/// Arithmetic Shift Left
0x0A => (Instruction::ASL, AddressingMode::Accumulator),
0x06 => (Instruction::ASL, AddressingMode::ZeroPage),
0x16 => (Instruction::ASL, AddressingMode::ZeroPage_X),
0x0E => (Instruction::ASL, AddressingMode::Absolute),
0x1E => (Instruction::ASL, AddressingMode::Absolute_X),
/// Logical Shift Right
0x4A => (Instruction::LSR, AddressingMode::Accumulator),
0x46 => (Instruction::LSR, AddressingMode::ZeroPage),
0x56 => (Instruction::LSR, AddressingMode::ZeroPage_X),
0x4E => (Instruction::LSR, AddressingMode::Absolute),
0x5E => (Instruction::LSR, AddressingMode::Absolute_X),
/// Rotate Left
0x2A => (Instruction::ROL, AddressingMode::Accumulator),
0x26 => (Instruction::ROL, AddressingMode::ZeroPage),
0x36 => (Instruction::ROL, AddressingMode::ZeroPage_X),
0x2E => (Instruction::ROL, AddressingMode::Absolute),
0x3E => (Instruction::ROL, AddressingMode::Absolute_X),
/// Rotate Right
0x6A => (Instruction::ROR, AddressingMode::Accumulator),
0x66 => (Instruction::ROR, AddressingMode::ZeroPage),
0x76 => (Instruction::ROR, AddressingMode::ZeroPage_X),
0x6E => (Instruction::ROR, AddressingMode::Absolute),
0x7E => (Instruction::ROR, AddressingMode::Absolute_X),

// ===== Jumps & Calls =====
/// Jump to another location
0x4C => (Instruction::JMP, AddressingMode::Absolute),
0x6C => (Instruction::JMP, AddressingMode::Indirect),
/// Jump to subroutine
0x20 => (Instruction::JSR, AddressingMode::Absolute),
/// Return from subroutine
0x60 => (Instruction::RTS, AddressingMode::Implicit),

// ===== Branches =====
/// Branch if Carry flag clear
0x90 => (Instruction::BCC, AddressingMode::Relative),
/// Branch if Carry flag set
0xB0 => (Instruction::BCS, AddressingMode::Relative),
/// Branch if Zero flag set
0xF0 => (Instruction::BEQ, AddressingMode::Relative),
/// Branch if Negative flag set
0x30 => (Instruction::BMI, AddressingMode::Relative),
/// Branch if Zero flag clear
0xD0 => (Instruction::BNE, AddressingMode::Relative),
/// Branch if Negative flag clear
0x10 => (Instruction::BPL, AddressingMode::Relative),
/// Branch if Overflow flag clear
0x50 => (Instruction::BVC, AddressingMode::Relative),
/// Branch if Overflow flag set
0x70 => (Instruction::BVS, AddressingMode::Relative),

// ===== Status Flag Changes =====
/// Clear Carry flag
0x18 => (Instruction::CLC, AddressingMode::Implicit),
/// Clear Decimal Mode flag
0xD8 => (Instruction::CLD, AddressingMode::Implicit),
/// Clear Interrupt Disable flag
0x58 => (Instruction::CLI, AddressingMode::Implicit),
/// Clear Overflow flag
0xB8 => (Instruction::CLV, AddressingMode::Implicit),
/// Set Carry flag
0x38 => (Instruction::SEC, AddressingMode::Implicit),
/// Set Decimal Mode flag
0xF8 => (Instruction::SED, AddressingMode::Implicit),
/// Set Interrupt Disable flag
0x78 => (Instruction::SEI, AddressingMode::Implicit),

// ===== System Functions =====
/// Force an Interrupt
0x00 => (Instruction::BRK, AddressingMode::Implicit),
/// No Operation
0xEA => (Instruction::NOP, AddressingMode::Implicit),
/// Return from Interrupt
0x40 => (Instruction::RTI, AddressingMode::Implicit),

// ===== Undocumented Opcodes =====
/// https:///www.oxyron.de/html/opcodes02.html
/// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => (Instruction::NOP, AddressingMode::Implicit),
0x0C => (Instruction::NOP, AddressingMode::Absolute),
0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => (Instruction::NOP, AddressingMode::Absolute_X),
0x04 | 0x44 | 0x64 => (Instruction::NOP, AddressingMode::ZeroPage),
0x14 | 0x34 | 0x54 | 0x74 | 0xd4 | 0xf4 => (Instruction::NOP, AddressingMode::ZeroPage_X),
0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 => (Instruction::NOP, AddressingMode::Immediate),

// ===== Illegal Opcodes =====
/// https:///www.oxyron.de/html/opcodes02.html
/// https:///www.nesdev.org/wiki/CPU_unofficial_opcodes
/// https:///www.nesdev.org/wiki/Programming_with_unofficial_opcodes
/// Equivalent to `ASL value` then `ORA value`
0x07 => (Instruction::SLO, AddressingMode::ZeroPage),
0x17 => (Instruction::SLO, AddressingMode::ZeroPage_X),
0x03 => (Instruction::SLO, AddressingMode::Indirect_X),
0x13 => (Instruction::SLO, AddressingMode::Indirect_Y),
0x0F => (Instruction::SLO, AddressingMode::Absolute),
0x1F => (Instruction::SLO, AddressingMode::Absolute_X),
0x1B => (Instruction::SLO, AddressingMode::Absolute_Y),

/// Equivalent to `ROL value` then `AND value`
0x27 => (Instruction::RLA, AddressingMode::ZeroPage),
0x37 => (Instruction::RLA, AddressingMode::ZeroPage_X),
0x23 => (Instruction::RLA, AddressingMode::Indirect_X),
0x33 => (Instruction::RLA, AddressingMode::Indirect_Y),
0x2F => (Instruction::RLA, AddressingMode::Absolute),
0x3F => (Instruction::RLA, AddressingMode::Absolute_X),
0x3B => (Instruction::RLA, AddressingMode::Absolute_Y),

/// Equivalent to `LSR value` then `EOR value`
0x47 => (Instruction::SRE, AddressingMode::ZeroPage),
0x57 => (Instruction::SRE, AddressingMode::ZeroPage_X),
0x43 => (Instruction::SRE, AddressingMode::Indirect_X),
0x53 => (Instruction::SRE, AddressingMode::Indirect_Y),
0x4F => (Instruction::SRE, AddressingMode::Absolute),
0x5F => (Instruction::SRE, AddressingMode::Absolute_X),
0x5B => (Instruction::SRE, AddressingMode::Absolute_Y),

/// Equivalent to `ROR value` then `ADC value`
0x67 => (Instruction::RRA, AddressingMode::ZeroPage),
0x77 => (Instruction::RRA, AddressingMode::ZeroPage_X),
0x63 => (Instruction::RRA, AddressingMode::Indirect_X),
0x73 => (Instruction::RRA, AddressingMode::Indirect_Y),
0x6F => (Instruction::RRA, AddressingMode::Absolute),
0x7F => (Instruction::RRA, AddressingMode::Absolute_X),
0x7B => (Instruction::RRA, AddressingMode::Absolute_Y),

/// Stores `A & X` into `{adr}`
0x87 => (Instruction::SAX, AddressingMode::ZeroPage),
0x97 => (Instruction::SAX, AddressingMode::ZeroPage_Y),
0x83 => (Instruction::SAX, AddressingMode::Indirect_X),
0x8F => (Instruction::SAX, AddressingMode::Absolute),

/// Shortcut for `LDA value` then `TAX`
0xAB => (Instruction::LAX, AddressingMode::Immediate),
0xA7 => (Instruction::LAX, AddressingMode::ZeroPage),
0xB7 => (Instruction::LAX, AddressingMode::ZeroPage_Y),
0xA3 => (Instruction::LAX, AddressingMode::Indirect_X),
0xB3 => (Instruction::LAX, AddressingMode::Indirect_Y),
0xAF => (Instruction::LAX, AddressingMode::Absolute),
0xBF => (Instruction::LAX, AddressingMode::Absolute_Y),

/// Equivalent to `DEC value` then `CMP value`
0xC7 => (Instruction::DCP, AddressingMode::ZeroPage),
0xD7 => (Instruction::DCP, AddressingMode::ZeroPage_X),
0xC3 => (Instruction::DCP, AddressingMode::Indirect_X),
0xD3 => (Instruction::DCP, AddressingMode::Indirect_Y),
0xCF => (Instruction::DCP, AddressingMode::Absolute),
0xDF => (Instruction::DCP, AddressingMode::Absolute_X),
0xDB => (Instruction::DCP, AddressingMode::Absolute_Y),

/// Equivalent to `INC value` then `SBC value`
0xE7 => (Instruction::ISC, AddressingMode::ZeroPage),
0xF7 => (Instruction::ISC, AddressingMode::ZeroPage_X),
0xE3 => (Instruction::ISC, AddressingMode::Indirect_X),
0xF3 => (Instruction::ISC, AddressingMode::Indirect_Y),
0xEF => (Instruction::ISC, AddressingMode::Absolute),
0xFF => (Instruction::ISC, AddressingMode::Absolute_X),
0xFB => (Instruction::ISC, AddressingMode::Absolute_Y),

/// Does `AND #i` then copies `N` to `C`
0x0B | 0x2B => (Instruction::ANC, AddressingMode::Immediate),

/// Equivalent to `AND #i` then `LSR A`
0x4B => (Instruction::ALR, AddressingMode::Immediate),

/// Similar to `AND #i`, but `C` is `bit 6` and `V` is `bit 6 XOR bit 5`
0x6B => (Instruction::ARR, AddressingMode::Immediate),

/// Unpredictable behavior - https:///www.nesdev.org/wiki/Visual6502wiki/6502_Opcode_8B_(XAA,_ANE)
0x8B => (Instruction::XAA, AddressingMode::Immediate),

/// Sets `X` to `A & X - #{imm}`
0xCB => (Instruction::AXS, AddressingMode::Immediate),

/// Equivalent to `SBC #i` then `NOP`
0xEB => (Instruction::SBC_NOP, AddressingMode::Immediate),

/// An incorrectly-implemented version of `SAX value`
0x93 => (Instruction::AHX, AddressingMode::Indirect_Y),
0x9F => (Instruction::AHX, AddressingMode::Absolute_Y),

/// An incorrectly-implemented version of `STY a,X`
0x9C => (Instruction::SHY, AddressingMode::Absolute_X),

/// An incorrectly-implemented version of `STX a,Y`
0x9E => (Instruction::SHX, AddressingMode::Absolute_Y),

/// Stores `A & X` into `S` then `AHX a,Y`
0x9B => (Instruction::TAS, AddressingMode::Absolute_Y),

/// Stores `{adr} & S` into `A`, `X`, and `S`
0xBB => (Instruction::LAS, AddressingMode::Absolute_Y),"""

regex_match = re.findall(
    r"(\/{2,3} .+)|(?:(0x[\w\d]{2})(?: => \(Instruction\:\:)(.+)(?:, AddressingMode\:\:)(.+)(?:\),))",
    old_instr,
)

tree = ""
tree_tmp = ["", ""]
for match in regex_match:
    docum = match[0]
    instr = match[2]
    opcode = match[1]
    mode = match[3]

    if docum != "":
        if tree_tmp[0] != "":
            tree += f"{tree_tmp[1]}}},\n"
            tree_tmp = ["", ""]
        tree += f"{docum}\n"

    elif instr != "" and opcode != "" and mode != "":
        if tree_tmp[0] != "" and tree_tmp[0] != instr:
            tree += f"{tree_tmp[1]}}}\n"
            tree_tmp = ["", ""]
        if tree_tmp[0] == "":
            tree_tmp[0] = instr
            tree_tmp[1] += f"{instr} {{\n"
        tree_tmp[1] += f"    {opcode} => {mode},\n"

tree += f"{tree_tmp[1]}}},\n"

print(tree)
