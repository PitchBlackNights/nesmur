#[derive(PartialEq, Eq)]
pub enum InterruptType {
    NMI,
}

#[derive(PartialEq, Eq)]
pub struct Interrupt {
    pub itype: InterruptType,
    pub vector_addr: u16,
    pub b_flag_mask: u8,
    pub cpu_cycles: usize,
}

pub const NMI: Interrupt = Interrupt {
    itype: InterruptType::NMI,
    vector_addr: 0xfffA,
    b_flag_mask: 0b00100000,
    cpu_cycles: 2,
};
