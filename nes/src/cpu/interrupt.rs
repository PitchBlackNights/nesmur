#[derive(PartialEq, Eq)]
pub enum InterruptType {
    NMI,
    IRQ,
    BRK,
    PHP,
}

#[derive(PartialEq, Eq)]
pub struct Interrupt {
    pub itype: InterruptType,
    pub vector_addr: u16,
    pub cpu_cycles: usize,
}

pub const NMI: Interrupt = Interrupt {
    itype: InterruptType::NMI,
    vector_addr: 0xFFFA
    cpu_cycles: 2,
};

pub const IRQ: Interrupt = Interrupt {
    itype: InterruptType::IRQ,
    vector_addr: 0xFFFE
    cpu_cycles: 1,
};

pub const BRK: Interrupt = Interrupt {
    itype: InterruptType::BRK,
    vector_addr: 0xFFFA
    cpu_cycles: 0,
};

pub const PHP: Interrupt = Interrupt {
    itype: InterruptType::PHP,
    vector_addr: 0xFFFA
    cpu_cycles: 0,
};