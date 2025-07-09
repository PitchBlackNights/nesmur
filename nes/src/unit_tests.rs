mod cpu_opcodes {
    use crate::cpu::opcode::{Instruction, OPCODES, OpCode};

    #[test]
    pub fn missing() {
        let mut missing_opcodes: Vec<u8> = Vec::new();
        for byte in 0..256 {
            match OPCODES.get(&(byte as u8)) {
                Some(_) => {}
                None => missing_opcodes.push(byte as u8),
            }
        }

        if missing_opcodes.len() > 0 {
            panic!(
                "{} missing opcodes\n[{}]",
                missing_opcodes.len(),
                missing_opcodes
                    .iter()
                    .map(|b| format!("{:#04X}", b))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }

    #[test]
    pub fn no_cycle() {
        let mut no_cycle_opcodes: Vec<OpCode> = Vec::new();
        for byte in 0..256 {
            match OPCODES.get(&(byte as u8)) {
                Some(opcode) => {
                    if (opcode.cycles <= 0) && (opcode.instruction != Instruction::KIL) {
                        no_cycle_opcodes.push(*opcode);
                    }
                }
                None => {}
            }
        }

        if no_cycle_opcodes.len() > 0 {
            panic!(
                "{} no cycle opcodes\n[{}]",
                no_cycle_opcodes.len(),
                no_cycle_opcodes
                    .iter()
                    .map(|op| format!("{:?} ({:#04X})", op.instruction, op.byte))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
}
