use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
pub struct FlagArgs {
    pub negative_result: bool,
    pub overflow: bool,
    pub unused: bool,
    pub break_command: bool,
    pub decimal_mode: bool,
    pub interrupt_disable: bool,
    pub zero_result: bool,
    pub carry: bool,
}

impl FlagArgs {
    pub const fn none() -> FlagArgs {
        FlagArgs {
            negative_result: false,
            overflow: false,
            unused: false,
            break_command: false,
            decimal_mode: false,
            interrupt_disable: false,
            zero_result: false,
            carry: false,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u8 {
        const NEGATIVE_RESULT   = 0b1000_0000;
        const OVERFLOW          = 0b0100_0000;
        const UNUSED            = 0b0010_0000;
        const BREAK_COMMAND     = 0b0001_0000;
        const DECIMAL_MODE      = 0b0000_1000;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const ZERO_RESULT       = 0b0000_0010;
        const CARRY             = 0b0000_0001;
    }
}

impl Flags {
    pub fn new(
        FlagArgs {
            negative_result,
            overflow,
            unused,
            break_command,
            decimal_mode,
            interrupt_disable,
            zero_result,
            carry,
        }: FlagArgs,
    ) -> Flags {
        let mut out = Flags::empty();

        if negative_result {
            out |= Flags::NEGATIVE_RESULT;
        }
        if overflow {
            out |= Flags::OVERFLOW;
        }
        if unused {
            out |= Flags::UNUSED;
        }
        if break_command {
            out |= Flags::BREAK_COMMAND;
        }
        if decimal_mode {
            out |= Flags::DECIMAL_MODE;
        }
        if interrupt_disable {
            out |= Flags::INTERRUPT_DISABLE;
        }
        if zero_result {
            out |= Flags::ZERO_RESULT;
        }
        if carry {
            out |= Flags::CARRY;
        }

        out
    }

    pub fn and(&mut self, rhs: Flags) {
        *self &= rhs;
    }

    pub fn or(&mut self, rhs: Flags) {
        *self |= rhs;
    }

    pub fn set_with_mask(&mut self, mask: Flags, rhs: Flags) {
        *self = (*self & !mask) | rhs;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::new(FlagArgs {
            negative_result: false,
            overflow: false,
            unused: true,
            break_command: false,
            decimal_mode: false,
            interrupt_disable: true,
            zero_result: false,
            carry: false,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Registers {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub flags: Flags,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            stack_pointer: 0x00,
            program_counter: 0,
            flags: Flags::default(),
        }
    }
}
