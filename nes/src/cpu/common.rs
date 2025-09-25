use super::{CPU, Flags, STACK, opcode::OpCode};
use crate::prelude::*;

// ============================
//   Stack Operations
// ============================
pub fn stack_push(cpu: &mut CPU, data: u8) {
    cpu.bus_mut().write(STACK + cpu.stack_pointer as u16, data);
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1)
}

pub fn stack_push_u16(cpu: &mut CPU, data: u16) {
    let data: [u8; 2] = tools::u16_to_bytes(data);
    stack_push(cpu, data[1]);
    stack_push(cpu, data[0]);
}

pub fn stack_pop(cpu: &mut CPU) -> u8 {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    cpu.bus_mut().read(STACK + cpu.stack_pointer as u16)
}

pub fn stack_pop_u16(cpu: &mut CPU) -> u16 {
    let lo: u8 = stack_pop(cpu);
    let hi: u8 = stack_pop(cpu);
    tools::bytes_to_u16(&[lo, hi])
}

// ============================
//   Instruction Logic
// ============================
pub fn branch(cpu: &mut CPU, opcode: &OpCode, condition: bool) {
    if condition {
        cpu.bus_mut().tick(1);
        let (addr, page_cross) = opcode.get_operand_address(cpu);
        if page_cross {
            cpu.bus_mut().tick(1);
        }
        cpu.program_counter = addr;
    }
}

pub fn compare(cpu: &mut CPU, opcode: &OpCode, compare_with: u8) {
    let (addr, page_cross) = opcode.get_operand_address(cpu);
    let data: u8 = cpu.bus_mut().read(addr);
    if data <= compare_with {
        cpu.status.insert(Flags::CARRY);
    } else {
        cpu.status.remove(Flags::CARRY);
    }
    update_flags_z(cpu, compare_with.wrapping_sub(data));
    update_flags_n(cpu, compare_with.wrapping_sub(data));
    if page_cross {
        cpu.bus_mut().tick(1);
    }
}

// ============================
//   Modifying Registers
// ============================
pub fn set_index_x(cpu: &mut CPU, value: u8) {
    cpu.index_x = value;
    update_flags_z(cpu, cpu.index_x);
    update_flags_n(cpu, cpu.index_x);
}

pub fn set_index_y(cpu: &mut CPU, value: u8) {
    cpu.index_y = value;
    update_flags_z(cpu, cpu.index_y);
    update_flags_n(cpu, cpu.index_y);
}

pub fn set_accumulator(cpu: &mut CPU, value: u8) {
    cpu.accumulator = value;
    update_flags_z(cpu, cpu.accumulator);
    update_flags_n(cpu, cpu.accumulator);
}

pub fn add_to_accumulator(cpu: &mut CPU, data: u8) {
    let sum: u16 = cpu.accumulator as u16
        + data as u16
        + (if cpu.status.contains(Flags::CARRY) {
            1
        } else {
            0
        }) as u16;

    let carry: bool = sum > 0x00FF;

    if carry {
        cpu.status.insert(Flags::CARRY);
    } else {
        cpu.status.remove(Flags::CARRY);
    }

    let result: u8 = sum as u8;

    if (data ^ result) & (result ^ cpu.accumulator) & 0x80 != 0 {
        cpu.status.insert(Flags::OVERFLOW);
    } else {
        cpu.status.remove(Flags::OVERFLOW)
    }

    set_accumulator(cpu, result);
}

pub fn sub_from_accumulator(cpu: &mut CPU, data: u8) {
    add_to_accumulator(cpu, ((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
}

// ============================
//   Updating CPU Status
// ============================
pub fn update_flag_if(cpu: &mut CPU, flag: Flags, condition: bool) {
    if condition {
        cpu.status.insert(flag);
    } else {
        cpu.status.remove(flag);
    }
}

pub fn update_flags_zn(cpu: &mut CPU, value: u8) {
    update_flags_z(cpu, value);
    update_flags_n(cpu, value);
}

pub fn update_flags_z(cpu: &mut CPU, value: u8) {
    if value == 0 {
        cpu.status.insert(Flags::ZERO);
    } else {
        cpu.status.remove(Flags::ZERO);
    }
}

pub fn update_flags_n(cpu: &mut CPU, value: u8) {
    if value >> 7 == 1 {
        cpu.status.insert(Flags::NEGATIVE);
    } else {
        cpu.status.remove(Flags::NEGATIVE);
    }
}
