use crate::cpu::CPU;
use crate::prelude::*;
use crate::tools::{bytes_to_u16, page_cross};

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
    pub const fn bytes(self) -> u8 {
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

    pub fn get_operand_address(self, cpu: &CPU) -> (u16, bool) {
        match self {
            AddressingMode::Immediate => (cpu.program_counter, false),
            _ => self.get_absolute_address(cpu, cpu.program_counter),
        }
    }

    pub fn get_absolute_address(self, cpu: &CPU, addr: u16) -> (u16, bool) {
        match self {
            AddressingMode::ZeroPage => (cpu.bus().read(addr) as u16, false),
            AddressingMode::ZeroPage_X => {
                let pos: u8 = cpu.bus().read(addr);
                let addr: u16 = pos.wrapping_add(cpu.index_x) as u16;
                (addr, false)
            }
            AddressingMode::ZeroPage_Y => {
                let pos: u8 = cpu.bus().read(addr);
                let addr: u16 = pos.wrapping_add(cpu.index_y) as u16;
                (addr, false)
            }
            AddressingMode::Relative => {
                let jump: i8 = cpu.bus().read(addr) as i8;
                let jump_addr: u16 = addr.wrapping_add(1).wrapping_add(jump as u16);
                (jump_addr, page_cross(addr.wrapping_add(1), jump_addr))
            }
            AddressingMode::Absolute => (cpu.bus().read_u16(addr), false),
            AddressingMode::Absolute_X => {
                let base: u16 = cpu.bus().read_u16(addr);
                let addr: u16 = base.wrapping_add(cpu.index_x as u16);
                (addr, page_cross(base, addr))
            }
            AddressingMode::Absolute_Y => {
                let base: u16 = cpu.bus().read_u16(addr);
                let addr: u16 = base.wrapping_add(cpu.index_y as u16);
                (addr, page_cross(base, addr))
            }
            AddressingMode::Indirect => {
                // JMP ($xxyy), or JMP indirect, does not advance pages if the
                // lower eight bits of the specified address is $FF; the upper
                // eight bits are fetched from $xx00, 255 bytes earlier,
                // instead of the expected following byte.
                let base: u16 = cpu.bus().read_u16(addr);
                let addr: u16 = if base & 0x00FF == 0x00FF {
                    bytes_to_u16(&[cpu.bus().read(base), cpu.bus().read(base & 0xFF00)])
                } else {
                    cpu.bus().read_u16(base)
                };
                (addr, false)
            }
            AddressingMode::Indirect_X => {
                let base: u8 = cpu.bus().read(addr).wrapping_add(cpu.index_x);
                let ptr: u16 = cpu.bus().read_u16(base as u16);
                (ptr, false)
            }
            AddressingMode::Indirect_Y => {
                let base: u8 = cpu.bus().read(addr);
                let deref_base: u16 = cpu.bus().read_u16(base as u16);
                let deref: u16 = deref_base.wrapping_add(cpu.index_y as u16);
                (deref, page_cross(deref, deref_base))
            }
            _ => {
                panic!(
                    "Getting the operand address for Addressing Mode {:?} is not supported",
                    self
                );
            }
        }
    }
}
