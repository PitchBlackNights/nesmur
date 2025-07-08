use crate::cpu::CPU;
use crate::cpu::opcode::Instruction::*;
use crate::cpu::opcode::OpCode;
use crate::prelude::*;

pub fn execute_instruction(cpu: &mut CPU, opcode: &OpCode, operands: Vec<u8>) -> u64 {
    debug!("==== Executing Operation ====");
    debug!("  Address: {:#06X},", cpu.program_counter - 1);
    debug!("  Byte: {:#04X},", opcode.byte);
    debug!("  Instruction: {:?},", opcode.instruction);
    debug!("  Mnemonic: \"{}\"", opcode.mnemonic);
    debug!("  Len: {}", opcode.len);
    debug!("  Mode: {:?}", opcode.mode);
    debug!(
        "  Operands: [{}]",
        operands
            .iter()
            .map(|b| format!("{:#04X}", b))
            .collect::<Vec<_>>()
            .join(", ")
    );

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
        NOP => {}
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
        KIL => panic!("The `KIL` instruction was executed!"),
    }

    0
}
