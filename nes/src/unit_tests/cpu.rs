#![allow(non_snake_case)]

use super::*;
use crate::cpu::opcode::{Instruction, OPCODES, OpCode};

#[test]
fn test_for_missing_opcodes() {
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
fn test_for_no_cycle_opcodes() {
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

#[test]
fn test_0xA9_LDA_immediate_load_data() {
    // LDA #$05
    // HLT
    let mut nes: NES = setup_nes_with_rom(vec![0xA9, 0x05, 0x00]);
    nes.cpu.run();

    assert_eq!(nes.cpu.accumulator, 5);
    assert!(nes.cpu.status.bits() & 0b0000_0010 == 0b0000_0000);
    assert!(nes.cpu.status.bits() & 0b1000_0000 == 0b0000_0000);
}

#[test]
fn test_0xAA_TAX_move_A_to_X() {
    // TAX
    // HLT
    let mut nes: NES = setup_nes_with_rom(vec![0xAA, 0x00]);
    nes.cpu.accumulator = 0x0A;
    nes.cpu.run();

    assert_eq!(nes.cpu.index_x, 0x0A);
}

#[test]
fn test_5_ops_working_together() {
    // LDA #$C0
    // TAX
    // INX
    // HLT
    let mut nes: NES = setup_nes_with_rom(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
    nes.cpu.run();

    assert_eq!(nes.cpu.index_x, 0xC1);
}

#[test]
fn test_INX_overflow() {
    // INX
    // INX
    // HLT
    let mut nes: NES = setup_nes_with_rom(vec![0xE8, 0xE8, 0x00]);
    nes.cpu.index_x = 0xFF;
    nes.cpu.run();

    assert_eq!(nes.cpu.index_x, 0x01);
}

#[test]
fn test_LDA_from_memory() {
    // LDA $10
    // HLT
    let mut nes: NES = setup_nes_with_rom(vec![0xA5, 0x10, 0x00]);
    nes.bus_mut().write(0x0010, 0x55);
    nes.cpu.run();

    assert_eq!(nes.cpu.accumulator, 0x55);
}
