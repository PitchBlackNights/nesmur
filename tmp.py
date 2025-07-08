old_codes = """OpCode::new(0x00, Instruction::BRK, "BRK", 7, AddressingMode::Implicit),
OpCode::new(0x01, Instruction::ORA, "ORA", 6, AddressingMode::Indirect_X),
OpCode::new(0x02, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x03, Instruction::SLO, "SLO", 8, AddressingMode::Indirect_X),
OpCode::new(0x04, Instruction::NOP_ALT, "NOP", 3, AddressingMode::ZeroPage),
OpCode::new(0x05, Instruction::ORA, "ORA", 3, AddressingMode::ZeroPage),
OpCode::new(0x06, Instruction::ASL, "ASL", 5, AddressingMode::ZeroPage),
OpCode::new(0x07, Instruction::SLO, "SLO", 5, AddressingMode::ZeroPage),
OpCode::new(0x08, Instruction::PHP, "PHP", 3, AddressingMode::Implicit),
OpCode::new(0x09, Instruction::ORA, "ORA", 2, AddressingMode::Immediate),
OpCode::new(0x0A, Instruction::ASL, "ASL", 2, AddressingMode::Accumulator),
OpCode::new(0x0B, Instruction::ANC, "ANC", 2, AddressingMode::Immediate),
OpCode::new(0x0C, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute),
OpCode::new(0x0D, Instruction::ORA, "ORA", 4, AddressingMode::Absolute),
OpCode::new(0x0E, Instruction::ASL, "ASL", 6, AddressingMode::Absolute),
OpCode::new(0x0F, Instruction::SLO, "SLO", 6, AddressingMode::Absolute),

OpCode::new(0x10, Instruction::BPL, "BPL", 2, AddressingMode::Relative),
OpCode::new(0x11, Instruction::ORA, "ORA", 5, AddressingMode::Indirect_Y),
OpCode::new(0x12, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x13, Instruction::SLO, "SLO", 8, AddressingMode::Indirect_Y),
OpCode::new(0x14, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x15, Instruction::ORA, "ORA", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x16, Instruction::ASL, "ASL", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x17, Instruction::SLO, "SLO", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x18, Instruction::CLC, "CLC", 2, AddressingMode::Implicit),
OpCode::new(0x19, Instruction::ORA, "ORA", 4, AddressingMode::Absolute_Y),
OpCode::new(0x1A, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0x1B, Instruction::SLO, "SLO", 7, AddressingMode::Absolute_Y),
OpCode::new(0x1C, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0x1D, Instruction::ORA, "ORA", 4, AddressingMode::Absolute_X),
OpCode::new(0x1E, Instruction::ASL, "ASL", 7, AddressingMode::Absolute_X),
OpCode::new(0x1F, Instruction::SLO, "SLO", 7, AddressingMode::Absolute_X),

OpCode::new(0x20, Instruction::JSR, "JSR", 6, AddressingMode::Absolute),
OpCode::new(0x21, Instruction::AND, "AND", 6, AddressingMode::Indirect_X),
OpCode::new(0x22, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x23, Instruction::RLA, "RLA", 8, AddressingMode::Indirect_X),
OpCode::new(0x24, Instruction::BIT, "BIT", 3, AddressingMode::ZeroPage),
OpCode::new(0x25, Instruction::AND, "AND", 3, AddressingMode::ZeroPage),
OpCode::new(0x26, Instruction::ROL, "ROL", 5, AddressingMode::ZeroPage),
OpCode::new(0x27, Instruction::RLA, "RLA", 5, AddressingMode::ZeroPage),
OpCode::new(0x28, Instruction::PLP, "PLP", 4, AddressingMode::Implicit),
OpCode::new(0x29, Instruction::AND, "AND", 2, AddressingMode::Immediate),
OpCode::new(0x2A, Instruction::ROL, "ROL", 2, AddressingMode::Accumulator),
OpCode::new(0x2B, Instruction::ANC, "ANC", 2, AddressingMode::Immediate),
OpCode::new(0x2C, Instruction::BIT, "BIT", 4, AddressingMode::Absolute),
OpCode::new(0x2D, Instruction::AND, "AND", 4, AddressingMode::Absolute),
OpCode::new(0x2E, Instruction::ROL, "ROL", 6, AddressingMode::Absolute),
OpCode::new(0x2F, Instruction::RLA, "RLA", 6, AddressingMode::Absolute),

OpCode::new(0x30, Instruction::BMI, "BMI", 2, AddressingMode::Relative),
OpCode::new(0x31, Instruction::AND, "AND", 5, AddressingMode::Indirect_Y),
OpCode::new(0x32, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x33, Instruction::RLA, "RLA", 8, AddressingMode::Indirect_Y),
OpCode::new(0x34, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x35, Instruction::AND, "AND", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x36, Instruction::ROL, "ROL", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x37, Instruction::RLA, "RLA", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x38, Instruction::SEC, "SEC", 2, AddressingMode::Implicit),
OpCode::new(0x39, Instruction::AND, "AND", 4, AddressingMode::Absolute_Y),
OpCode::new(0x3A, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0x3B, Instruction::RLA, "RLA", 7, AddressingMode::Absolute_Y),
OpCode::new(0x3C, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0x3D, Instruction::AND, "AND", 4, AddressingMode::Absolute_X),
OpCode::new(0x3E, Instruction::ROL, "ROL", 7, AddressingMode::Absolute_X),
OpCode::new(0x3F, Instruction::RLA, "RLA", 7, AddressingMode::Absolute_X),

OpCode::new(0x40, Instruction::RTI, "RTI", 6, AddressingMode::Implicit),
OpCode::new(0x41, Instruction::EOR, "EOR", 6, AddressingMode::Indirect_X),
OpCode::new(0x42, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x43, Instruction::SRE, "SRE", 8, AddressingMode::Indirect_X),
OpCode::new(0x44, Instruction::NOP_ALT, "NOP", 3, AddressingMode::ZeroPage),
OpCode::new(0x45, Instruction::EOR, "EOR", 3, AddressingMode::ZeroPage),
OpCode::new(0x46, Instruction::LSR, "LSR", 5, AddressingMode::ZeroPage),
OpCode::new(0x47, Instruction::SRE, "SRE", 5, AddressingMode::ZeroPage),
OpCode::new(0x48, Instruction::PHA, "PHA", 3, AddressingMode::Implicit),
OpCode::new(0x49, Instruction::EOR, "EOR", 2, AddressingMode::Immediate),
OpCode::new(0x4A, Instruction::LSR, "LSR", 2, AddressingMode::Accumulator),
OpCode::new(0x4B, Instruction::ALR, "ALR", 2, AddressingMode::Immediate),
OpCode::new(0x4C, Instruction::JMP, "JMP", 3, AddressingMode::Absolute),
OpCode::new(0x4D, Instruction::EOR, "EOR", 4, AddressingMode::Absolute),
OpCode::new(0x4E, Instruction::LSR, "LSR", 6, AddressingMode::Absolute),
OpCode::new(0x4F, Instruction::SRE, "SRE", 6, AddressingMode::Absolute),

OpCode::new(0x50, Instruction::BVC, "BVC", 2, AddressingMode::Relative),
OpCode::new(0x51, Instruction::EOR, "EOR", 5, AddressingMode::Indirect_Y),
OpCode::new(0x52, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x53, Instruction::SRE, "SRE", 8, AddressingMode::Indirect_Y),
OpCode::new(0x54, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x55, Instruction::EOR, "EOR", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x56, Instruction::LSR, "LSR", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x57, Instruction::SRE, "SRE", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x58, Instruction::CLI, "CLI", 2, AddressingMode::Implicit),
OpCode::new(0x59, Instruction::EOR, "EOR", 4, AddressingMode::Absolute_Y),
OpCode::new(0x5A, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0x5B, Instruction::SRE, "SRE", 7, AddressingMode::Absolute_Y),
OpCode::new(0x5C, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0x5D, Instruction::EOR, "EOR", 4, AddressingMode::Absolute_X),
OpCode::new(0x5E, Instruction::LSR, "LSR", 7, AddressingMode::Absolute_X),
OpCode::new(0x5F, Instruction::SRE, "SRE", 7, AddressingMode::Absolute_X),

OpCode::new(0x60, Instruction::RTS, "RTS", 6, AddressingMode::Implicit),
OpCode::new(0x61, Instruction::ADC, "ADC", 6, AddressingMode::Indirect_X),
OpCode::new(0x62, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x63, Instruction::RRA, "RRA", 8, AddressingMode::Indirect_X),
OpCode::new(0x64, Instruction::NOP_ALT, "NOP", 3, AddressingMode::ZeroPage),
OpCode::new(0x65, Instruction::ADC, "ADC", 3, AddressingMode::ZeroPage),
OpCode::new(0x66, Instruction::ROR, "ROR", 5, AddressingMode::ZeroPage),
OpCode::new(0x67, Instruction::RRA, "RRA", 5, AddressingMode::ZeroPage),
OpCode::new(0x68, Instruction::PLA, "PLA", 4, AddressingMode::Implicit),
OpCode::new(0x69, Instruction::ADC, "ADC", 2, AddressingMode::Immediate),
OpCode::new(0x6A, Instruction::ROR, "ROR", 2, AddressingMode::Accumulator),
OpCode::new(0x6B, Instruction::ARR, "ARR", 2, AddressingMode::Immediate),
OpCode::new(0x6C, Instruction::JMP, "JMP", 5, AddressingMode::Indirect),
OpCode::new(0x6D, Instruction::ADC, "ADC", 4, AddressingMode::Absolute),
OpCode::new(0x6E, Instruction::ROR, "ROR", 6, AddressingMode::Absolute),
OpCode::new(0x6F, Instruction::RRA, "RRA", 6, AddressingMode::Absolute),

OpCode::new(0x70, Instruction::BVS, "BVS", 2, AddressingMode::Relative),
OpCode::new(0x71, Instruction::ADC, "ADC", 5, AddressingMode::Indirect_Y),
OpCode::new(0x72, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x73, Instruction::RRA, "RRA", 8, AddressingMode::Indirect_Y),
OpCode::new(0x74, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x75, Instruction::ADC, "ADC", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x76, Instruction::ROR, "ROR", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x77, Instruction::RRA, "RRA", 6, AddressingMode::ZeroPage_X),
OpCode::new(0x78, Instruction::SEI, "SEI", 2, AddressingMode::Implicit),
OpCode::new(0x79, Instruction::ADC, "ADC", 4, AddressingMode::Absolute_Y),
OpCode::new(0x7A, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0x7B, Instruction::RRA, "RRA", 7, AddressingMode::Absolute_Y),
OpCode::new(0x7C, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0x7D, Instruction::ADC, "ADC", 4, AddressingMode::Absolute_X),
OpCode::new(0x7E, Instruction::ROR, "ROR", 7, AddressingMode::Absolute_X),
OpCode::new(0x7F, Instruction::RRA, "RRA", 7, AddressingMode::Absolute_X),

OpCode::new(0x80, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Immediate),
OpCode::new(0x81, Instruction::STA, "STA", 6, AddressingMode::Indirect_X),
OpCode::new(0x82, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Immediate),
OpCode::new(0x83, Instruction::SAX, "SAX", 6, AddressingMode::Indirect_X),
OpCode::new(0x84, Instruction::STY, "STY", 3, AddressingMode::ZeroPage),
OpCode::new(0x85, Instruction::STA, "STA", 3, AddressingMode::ZeroPage),
OpCode::new(0x86, Instruction::STX, "STX", 3, AddressingMode::ZeroPage),
OpCode::new(0x87, Instruction::SAX, "SAX", 3, AddressingMode::ZeroPage),
OpCode::new(0x88, Instruction::DEY, "DEY", 2, AddressingMode::Implicit),
OpCode::new(0x89, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Immediate),
OpCode::new(0x8A, Instruction::TXA, "TXA", 2, AddressingMode::Implicit),
OpCode::new(0x8B, Instruction::XAA, "XAA", 2, AddressingMode::Immediate),
OpCode::new(0x8C, Instruction::STY, "STY", 4, AddressingMode::Absolute),
OpCode::new(0x8D, Instruction::STA, "STA", 4, AddressingMode::Absolute),
OpCode::new(0x8E, Instruction::STX, "STX", 4, AddressingMode::Absolute),
OpCode::new(0x8F, Instruction::SAX, "SAX", 4, AddressingMode::Absolute),

OpCode::new(0x90, Instruction::BCC, "BCC", 2, AddressingMode::Relative),
OpCode::new(0x91, Instruction::STA, "STA", 6, AddressingMode::Indirect_Y),
OpCode::new(0x92, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0x93, Instruction::AHX, "AHX", 6, AddressingMode::Indirect_Y),
OpCode::new(0x94, Instruction::STY, "STY", 4, AddressingMode::ZeroPage_Y),
OpCode::new(0x95, Instruction::STA, "STA", 4, AddressingMode::ZeroPage_X),
OpCode::new(0x96, Instruction::STX, "STX", 4, AddressingMode::ZeroPage_Y),
OpCode::new(0x97, Instruction::SAX, "SAX", 4, AddressingMode::ZeroPage_Y),
OpCode::new(0x98, Instruction::TYA, "TYA", 2, AddressingMode::Implicit),
OpCode::new(0x99, Instruction::STA, "STA", 5, AddressingMode::Absolute_Y),
OpCode::new(0x9A, Instruction::TXS, "TXS", 2, AddressingMode::Implicit),
OpCode::new(0x9B, Instruction::TAS, "TAS", 5, AddressingMode::Absolute_Y),
OpCode::new(0x9C, Instruction::SHY, "SHY", 5, AddressingMode::Absolute_X),
OpCode::new(0x9D, Instruction::STA, "STA", 5, AddressingMode::Absolute_X),
OpCode::new(0x9E, Instruction::SHX, "SHX", 5, AddressingMode::Absolute_Y),
OpCode::new(0x9F, Instruction::AHX, "AHX", 5, AddressingMode::Absolute_Y),

OpCode::new(0xA0, Instruction::LDY, "LDY", 2, AddressingMode::Immediate),
OpCode::new(0xA1, Instruction::LDA, "LDA", 6, AddressingMode::Indirect_X),
OpCode::new(0xA2, Instruction::LDX, "LDX", 2, AddressingMode::Immediate),
OpCode::new(0xA3, Instruction::LAX, "LAX", 6, AddressingMode::Indirect_X),
OpCode::new(0xA4, Instruction::LDY, "LDY", 3, AddressingMode::ZeroPage),
OpCode::new(0xA5, Instruction::LDA, "LDA", 3, AddressingMode::ZeroPage),
OpCode::new(0xA6, Instruction::LDX, "LDX", 3, AddressingMode::ZeroPage),
OpCode::new(0xA7, Instruction::LAX, "LAX", 3, AddressingMode::ZeroPage),
OpCode::new(0xA8, Instruction::TAY, "TAY", 2, AddressingMode::Implicit),
OpCode::new(0xA9, Instruction::LDA, "LDA", 2, AddressingMode::Immediate),
OpCode::new(0xAA, Instruction::TAX, "TAX", 2, AddressingMode::Implicit),
OpCode::new(0xAB, Instruction::LAX, "LAX", 2, AddressingMode::Immediate),
OpCode::new(0xAC, Instruction::LDY, "LDY", 4, AddressingMode::Absolute),
OpCode::new(0xAD, Instruction::LDA, "LDA", 4, AddressingMode::Absolute),
OpCode::new(0xAE, Instruction::LDX, "LDX", 4, AddressingMode::Absolute),
OpCode::new(0xAF, Instruction::LAX, "LAX", 4, AddressingMode::Absolute),

OpCode::new(0xB0, Instruction::BCS, "BCS", 2, AddressingMode::Relative),
OpCode::new(0xB1, Instruction::LDA, "LDA", 5, AddressingMode::Indirect_Y),
OpCode::new(0xB2, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0xB3, Instruction::LAX, "LAX", 5, AddressingMode::Indirect_Y),
OpCode::new(0xB4, Instruction::LDY, "LDY", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xB5, Instruction::LDA, "LDA", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xB6, Instruction::LDX, "LDX", 4, AddressingMode::ZeroPage_Y),
OpCode::new(0xB7, Instruction::LAX, "LAX", 4, AddressingMode::ZeroPage_Y),
OpCode::new(0xB8, Instruction::CLV, "CLV", 2, AddressingMode::Implicit),
OpCode::new(0xB9, Instruction::LDA, "LDA", 4, AddressingMode::Absolute_Y),
OpCode::new(0xBA, Instruction::TSX, "TSX", 2, AddressingMode::Implicit),
OpCode::new(0xBB, Instruction::LAS, "LAS", 4, AddressingMode::Absolute_Y),
OpCode::new(0xBC, Instruction::LDY, "LDY", 4, AddressingMode::Absolute_X),
OpCode::new(0xBD, Instruction::LDA, "LDA", 4, AddressingMode::Absolute_X),
OpCode::new(0xBE, Instruction::LDX, "LDX", 4, AddressingMode::Absolute_Y),
OpCode::new(0xBF, Instruction::LAX, "LAX", 4, AddressingMode::Absolute_Y),

OpCode::new(0xC0, Instruction::CPY, "CPY", 2, AddressingMode::Immediate),
OpCode::new(0xC1, Instruction::CMP, "CMP", 6, AddressingMode::Indirect_X),
OpCode::new(0xC2, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Immediate),
OpCode::new(0xC3, Instruction::DCP, "DCP", 8, AddressingMode::Indirect_X),
OpCode::new(0xC4, Instruction::CPY, "CPY", 3, AddressingMode::ZeroPage),
OpCode::new(0xC5, Instruction::CMP, "CMP", 3, AddressingMode::ZeroPage),
OpCode::new(0xC6, Instruction::DEC, "DEC", 5, AddressingMode::ZeroPage),
OpCode::new(0xC7, Instruction::DCP, "DCP", 5, AddressingMode::ZeroPage),
OpCode::new(0xC8, Instruction::INY, "INY", 2, AddressingMode::Implicit),
OpCode::new(0xC9, Instruction::CMP, "CMP", 2, AddressingMode::Immediate),
OpCode::new(0xCA, Instruction::DEX, "DEX", 2, AddressingMode::Implicit),
OpCode::new(0xCB, Instruction::AXS, "AXS", 2, AddressingMode::Immediate),
OpCode::new(0xCC, Instruction::CPY, "CPY", 4, AddressingMode::Absolute),
OpCode::new(0xCD, Instruction::CMP, "CMP", 4, AddressingMode::Absolute),
OpCode::new(0xCE, Instruction::DEC, "DEC", 6, AddressingMode::Absolute),
OpCode::new(0xCF, Instruction::DCP, "DCP", 6, AddressingMode::Absolute),

OpCode::new(0xD0, Instruction::BNE, "BNE", 2, AddressingMode::Relative),
OpCode::new(0xD1, Instruction::CMP, "CMP", 5, AddressingMode::Indirect_Y),
OpCode::new(0xD2, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0xD3, Instruction::DCP, "DCP", 8, AddressingMode::Indirect_Y),
OpCode::new(0xD4, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xD5, Instruction::CMP, "CMP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xD6, Instruction::DEC, "DEC", 6, AddressingMode::ZeroPage_X),
OpCode::new(0xD7, Instruction::DCP, "DCP", 6, AddressingMode::ZeroPage_X),
OpCode::new(0xD8, Instruction::CLD, "CLD", 2, AddressingMode::Implicit),
OpCode::new(0xD9, Instruction::CMP, "CMP", 4, AddressingMode::Absolute_Y),
OpCode::new(0xDA, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0xDB, Instruction::DCP, "DCP", 7, AddressingMode::Absolute_Y),
OpCode::new(0xDC, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0xDD, Instruction::CMP, "CMP", 4, AddressingMode::Absolute_X),
OpCode::new(0xDE, Instruction::DEC, "DEC", 7, AddressingMode::Absolute_X),
OpCode::new(0xDF, Instruction::DCP, "DCP", 7, AddressingMode::Absolute_X),

OpCode::new(0xE0, Instruction::CPX, "CPX", 2, AddressingMode::Immediate),
OpCode::new(0xE1, Instruction::SBC, "SBC", 6, AddressingMode::Indirect_X),
OpCode::new(0xE2, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Immediate),
OpCode::new(0xE3, Instruction::ISC, "ISC", 8, AddressingMode::Indirect_X),
OpCode::new(0xE4, Instruction::CPX, "CPX", 3, AddressingMode::ZeroPage),
OpCode::new(0xE5, Instruction::SBC, "SBC", 3, AddressingMode::ZeroPage),
OpCode::new(0xE6, Instruction::INC, "INC", 5, AddressingMode::ZeroPage),
OpCode::new(0xE7, Instruction::ISC, "ISC", 5, AddressingMode::ZeroPage),
OpCode::new(0xE8, Instruction::INX, "INX", 2, AddressingMode::Implicit),
OpCode::new(0xE9, Instruction::SBC, "SBC", 2, AddressingMode::Immediate),
OpCode::new(0xEA, Instruction::NOP, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0xEB, Instruction::SBC_NOP, "SBC", 2, AddressingMode::Immediate),
OpCode::new(0xEC, Instruction::CPX, "CPX", 4, AddressingMode::Absolute),
OpCode::new(0xED, Instruction::SBC, "SBC", 4, AddressingMode::Absolute),
OpCode::new(0xEE, Instruction::INC, "INC", 6, AddressingMode::Absolute),
OpCode::new(0xEF, Instruction::ISC, "ISC", 6, AddressingMode::Absolute),

OpCode::new(0xF0, Instruction::BEQ, "BEQ", 2, AddressingMode::Relative),
OpCode::new(0xF1, Instruction::SBC, "SBC", 5, AddressingMode::Indirect_Y),
OpCode::new(0xF2, Instruction::KIL, "KIL", 0, AddressingMode::Implicit),
OpCode::new(0xF3, Instruction::ISC, "ISC", 8, AddressingMode::Indirect_Y),
OpCode::new(0xF4, Instruction::NOP_ALT, "NOP", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xF5, Instruction::SBC, "SBC", 4, AddressingMode::ZeroPage_X),
OpCode::new(0xF6, Instruction::INC, "INC", 6, AddressingMode::ZeroPage_X),
OpCode::new(0xF7, Instruction::ISC, "ISC", 6, AddressingMode::ZeroPage_X),
OpCode::new(0xF8, Instruction::SED, "SED", 2, AddressingMode::Implicit),
OpCode::new(0xF9, Instruction::SBC, "SBC", 4, AddressingMode::Absolute_Y),
OpCode::new(0xFA, Instruction::NOP_ALT, "NOP", 2, AddressingMode::Implicit),
OpCode::new(0xFB, Instruction::ISC, "ISC", 7, AddressingMode::Absolute_Y),
OpCode::new(0xFC, Instruction::NOP_ALT, "NOP", 4, AddressingMode::Absolute_X),
OpCode::new(0xFD, Instruction::SBC, "SBC", 4, AddressingMode::Absolute_X),
OpCode::new(0xFE, Instruction::INC, "INC", 7, AddressingMode::Absolute_X),
OpCode::new(0xFF, Instruction::ISC, "ISC", 7, AddressingMode::Absolute_X),"""

old_instr = """// ===== Load/Store Operations =====
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
// https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes
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
/// ***WARNING:*** Highly Unstable
XAA,
/// Sets `X` to `A & X - #{imm}`
AXS,
/// Equivalent to `SBC #i` then `NOP`
SBC_NOP,
/// An incorrectly-implemented version of `SAX value`
/// **WARNING:** Unstable in certain situations
AHX,
/// An incorrectly-implemented version of `STY a,X`
/// **WARNING:** Unstable in certain situations
SHY,
/// An incorrectly-implemented version of `STX a,Y`
/// **WARNING:** Unstable in certain situations
SHX,
/// Stores `A & X` into `S` then `AHX a,Y`
/// **WARNING:** Unstable in certain situations
TAS,
/// Stores `{adr} & S` into `A`, `X`, and `S`
LAS,
/// Traps the CPU indefinitely with $FF on the bus, requires a reset to fix
KIL,"""



import re, copy

instructions = {}

regex_match = re.findall(
    r"(\/{2,3} .+)|(?:([A-Za-z]{3}|[A-Za-z_]{7}),)",
    old_instr,
)
docs = ""
for match in regex_match:
    docum = match[0]
    instr = match[1]

    if docum != "":
        docs += f"{"\n" if docs != "" else ""}{docum}"

    if instr != "":
        instructions[instr] = {"docs": docs, "opcodes": {}}
        docs = ""

regex_match = re.findall(
    r"(?:OpCode::new\((0x[0-9A-F]{2}), Instruction::([A-Za-z]{3}|[A-Za-z_]{7}), \"\w+\", (\d), AddressingMode::(\w+)\),)",
    old_codes,
)
for match in regex_match:
    code = match[0]
    instr = match[1]
    cycles = match[2]
    mode = match[3]

    instructions[instr]["opcodes"][code] = {"mode": mode, "cycles": cycles}

instructions_copy = copy.deepcopy(instructions)
for instr in instructions:
    modes = {
        "Implicit": {},
        "Accumulator": {},
        "Immediate": {},
        "ZeroPage": {},
        "ZeroPage_X": {},
        "ZeroPage_Y": {},
        "Relative": {},
        "Absolute": {},
        "Absolute_X": {},
        "Absolute_Y": {},
        "Indirect": {},
        "Indirect_X": {},
        "Indirect_Y": {},
    }
    for code in instructions[instr]["opcodes"]:
        modes[instructions[instr]["opcodes"][code]["mode"]][code] = instructions[instr]["opcodes"][code]["cycles"]
    instructions_copy[instr]["opcodes"] = {}
    for mode in modes:
        for code in modes[mode]:
            instructions_copy[instr]["opcodes"][code] = {"mode": mode, "cycles": modes[mode][code]}
instructions = instructions_copy

for instr in instructions:
    print(f"{instructions[instr]["docs"]}")
    print(f"{instr} {{")
    for code in instructions[instr]["opcodes"]:
        print(f"    {code} => {instructions[instr]["opcodes"][code]["cycles"]}, {instructions[instr]["opcodes"][code]["mode"]},")
    print(f"}},")
