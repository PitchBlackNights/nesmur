use crate::cpu::CPU;

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

    pub fn read(self, cpu: &CPU, operands: Option<Vec<u8>>) -> (u8, bool) {
        todo!()
    }

    pub fn write(self, cpu: &mut CPU, operands: Option<Vec<u8>>, value: u8) -> bool {
        todo!()
    }
}
