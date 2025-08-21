mod common;
pub mod interrupt;
pub mod opcode;

use crate::RcRef;
use crate::bus::Bus;
use crate::cpu::interrupt::Interrupt;
use crate::cpu::opcode::AddressingMode::*;
use crate::cpu::opcode::Instruction::*;
use crate::cpu::opcode::OpCode;
use crate::prelude::*;
// use crate::tools;

bitflags! {
    /// Status Register (P) - http://wiki.nesdev.com/w/index.php/Status_flags
    /// ```plaintext
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +---- Carry
    ///  | |   | | | +------ Zero
    ///  | |   | | +-------- Interrupt Disable
    ///  | |   | +---------- Decimal Mode (not used on NES)
    ///  | |   +------------ Break
    ///  | +---------------- Overflow
    ///  +------------------ Negative
    /// ```
    #[derive(Clone, Debug)]
    pub struct Flags: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAK             = 0b0001_0000;
        const UNUSED            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIVE          = 0b1000_0000;
    }
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

#[rustfmt::skip]
impl NESAccess for CPU {
    fn bus(&self) -> Ref<Bus> { self.bus.borrow() }
    fn bus_mut(&self) -> RefMut<Bus> { self.bus.borrow_mut() }
}

pub struct CPU {
    pub running: bool,
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status: Flags,
    pub fresh: bool,
    pub bus: RcRef<Bus>,
}

impl CPU {
    pub fn new(bus: RcRef<Bus>) -> Self {
        // Hack to build OPCODES hashmap now instead of in `cpu::step()`
        let _ = &opcode::OPCODES.get(&0u8);

        let pc: u16 = bus.borrow_mut().read_u16(0xFFFC);
        CPU {
            running: false,
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            stack_pointer: STACK_RESET,
            program_counter: pc,
            status: Flags::from_bits_truncate(0b0010_0100),
            fresh: true,
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.stack_pointer -= 3;
        self.status.insert(Flags::UNUSED);
        self.status.insert(Flags::INTERRUPT_DISABLE);
        let pc: u16 = self.bus_mut().read_u16(0xFFFC);
        self.program_counter = pc;
        self.fresh = false;
        self.bus_mut().tick(7);
    }

    fn interrupt(&mut self, interrupt: Interrupt) {
        common::stack_push_u16(self, self.program_counter);

        let mut flag: Flags = self.status.clone();
        flag.set(Flags::BREAK, interrupt == interrupt::BRK);
        flag.insert(Flags::UNUSED);
        common::stack_push(self, flag.bits());

        self.status.insert(Flags::INTERRUPT_DISABLE);
        self.bus_mut().tick(interrupt.cpu_cycles);
        let interrupt_vector: u16 = self.bus_mut().read_u16(interrupt.vector_addr);
        self.program_counter = interrupt_vector;
    }

    pub fn run_with_callback(&mut self, mut callback: impl FnMut(&mut CPU)) {
        self.running = true;
        if self.fresh {
            self.bus_mut().tick(7);
            self.fresh = false;
        }

        info!("Running CPU...");
        while self.running {
            let pending_interrupt: Option<Interrupt> = self.bus_mut().poll_interrupts();
            if let Some(interrupt) = pending_interrupt {
                self.interrupt(interrupt);
            }

            callback(self);

            let opbyte: u8 = self.bus_mut().read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state: u16 = self.program_counter;
            let opcode: &'static OpCode = opcode::decode_opcode(opbyte);

            self.execute_instruction(opcode);
            self.bus_mut().tick(opcode.cycles);

            if program_counter_state == self.program_counter {
                self.program_counter += opcode.len as u16 - 1
            }
        }
        info!("Stopping CPU...");
    }

    fn execute_instruction(&mut self, opcode: &OpCode) {
        match opcode.instruction {
            LDA => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let value: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, value);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            LDX => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let value: u8 = self.bus_mut().read(addr);
                common::set_index_x(self, value);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            LDY => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let value: u8 = self.bus_mut().read(addr);
                common::set_index_y(self, value);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            STA => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                self.bus_mut().write(addr, self.accumulator);
            }
            STX => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                self.bus_mut().write(addr, self.index_x);
            }
            STY => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                self.bus_mut().write(addr, self.index_y);
            }
            TAX => {
                common::set_index_x(self, self.accumulator);
            }
            TAY => {
                common::set_index_y(self, self.accumulator);
            }
            TXA => {
                common::set_accumulator(self, self.index_x);
            }
            TYA => {
                common::set_accumulator(self, self.index_y);
            }
            TSX => {
                common::set_index_x(self, self.stack_pointer);
            }
            TXS => {
                self.stack_pointer = self.index_x;
            }
            PHA => common::stack_push(self, self.accumulator),
            PHP => {
                let mut flags: Flags = self.status.clone();
                flags.insert(Flags::BREAK);
                flags.insert(Flags::UNUSED);
                common::stack_push(self, flags.bits());
            }
            PLA => {
                let value: u8 = common::stack_pop(self);
                common::set_accumulator(self, value);
            }
            PLP => {
                self.status = Flags::from_bits_truncate(common::stack_pop(self));
                self.status.remove(Flags::BREAK);
                self.status.insert(Flags::UNUSED);
            }
            AND => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data & self.accumulator);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            EOR => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data ^ self.accumulator);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            ORA => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data | self.accumulator);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            BIT => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::update_flags_z(self, self.accumulator & data);
                self.status.set(Flags::NEGATIVE, data & 0b1000_0000 > 0);
                self.status.set(Flags::OVERFLOW, data & 0b0100_0000 > 0);
            }
            ADC => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let value: u8 = self.bus_mut().read(addr);
                common::add_to_accumulator(self, value);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            SBC => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let value: u8 = self.bus_mut().read(addr);
                common::sub_from_accumulator(self, value);
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            CMP => {
                common::compare(self, opcode, self.accumulator);
            }
            CPX => {
                common::compare(self, opcode, self.index_x);
            }
            CPY => {
                common::compare(self, opcode, self.index_y);
            }
            INC => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr).wrapping_add(1);
                self.bus_mut().write(addr, data);
                common::update_flags_zn(self, data);
            }
            INX => {
                common::set_index_x(self, self.index_x.wrapping_add(1));
            }
            INY => {
                common::set_index_y(self, self.index_y.wrapping_add(1));
            }
            DEC => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr).wrapping_sub(1);
                self.bus_mut().write(addr, data);
                common::update_flags_zn(self, data);
            }
            DEX => {
                common::set_index_x(self, self.index_x.wrapping_sub(1));
            }
            DEY => {
                common::set_index_y(self, self.index_y.wrapping_sub(1));
            }
            ASL => match opcode.mode {
                Accumulator => {
                    let data: u8 = self.accumulator;
                    common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                    common::set_accumulator(self, data << 1);
                }
                _ => {
                    let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                    let mut data: u8 = self.bus_mut().read(addr);
                    common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                    data <<= 1;
                    self.bus_mut().write(addr, data);
                    common::update_flags_zn(self, data);
                }
            },
            LSR => match opcode.mode {
                Accumulator => {
                    let data: u8 = self.accumulator;
                    common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                    common::set_accumulator(self, data >> 1);
                }
                _ => {
                    let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                    let mut data: u8 = self.bus_mut().read(addr);
                    common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                    data >>= 1;
                    self.bus_mut().write(addr, data);
                    common::update_flags_zn(self, data);
                }
            },
            ROL => match opcode.mode {
                Accumulator => {
                    let mut data: u8 = self.accumulator;
                    let old_carry: bool = self.status.contains(Flags::CARRY);
                    common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                    data <<= 1;
                    if old_carry {
                        data |= 1;
                    }
                    common::set_accumulator(self, data);
                }
                _ => {
                    let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                    let mut data: u8 = self.bus_mut().read(addr);
                    let old_carry: bool = self.status.contains(Flags::CARRY);
                    common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                    data <<= 1;
                    if old_carry {
                        data |= 1;
                    }
                    self.bus_mut().write(addr, data);
                    common::update_flags_n(self, data);
                }
            },
            ROR => match opcode.mode {
                Accumulator => {
                    let mut data: u8 = self.accumulator;
                    let old_carry: bool = self.status.contains(Flags::CARRY);
                    common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                    data >>= 1;
                    if old_carry {
                        data |= 0b1000_0000;
                    }
                    common::set_accumulator(self, data);
                }
                _ => {
                    let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                    let mut data: u8 = self.bus_mut().read(addr);
                    let old_carry: bool = self.status.contains(Flags::CARRY);
                    common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                    data >>= 1;
                    if old_carry {
                        data |= 0b1000_0000;
                    }
                    self.bus_mut().write(addr, data);
                    common::update_flags_n(self, data);
                }
            },
            JMP => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                self.program_counter = addr;
            }
            JSR => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                common::stack_push_u16(self, self.program_counter + 1);
                self.program_counter = addr;
            }
            RTS => {
                self.program_counter = common::stack_pop_u16(self) + 1;
            }
            BCC => {
                common::branch(self, opcode, !self.status.contains(Flags::CARRY));
            }
            BCS => {
                common::branch(self, opcode, self.status.contains(Flags::CARRY));
            }
            BEQ => {
                common::branch(self, opcode, self.status.contains(Flags::ZERO));
            }
            BMI => {
                common::branch(self, opcode, self.status.contains(Flags::NEGATIVE));
            }
            BNE => {
                common::branch(self, opcode, !self.status.contains(Flags::ZERO));
            }
            BPL => {
                common::branch(self, opcode, !self.status.contains(Flags::NEGATIVE));
            }
            BVC => {
                common::branch(self, opcode, !self.status.contains(Flags::OVERFLOW));
            }
            BVS => {
                common::branch(self, opcode, self.status.contains(Flags::OVERFLOW));
            }
            CLC => self.status.remove(Flags::CARRY),
            CLD => self.status.remove(Flags::DECIMAL_MODE),
            CLI => self.status.remove(Flags::INTERRUPT_DISABLE),
            CLV => self.status.remove(Flags::OVERFLOW),
            SEC => self.status.insert(Flags::CARRY),
            SED => self.status.insert(Flags::DECIMAL_MODE),
            SEI => self.status.insert(Flags::INTERRUPT_DISABLE),
            BRK => {
                if !self.status.contains(Flags::INTERRUPT_DISABLE) {
                    self.interrupt(interrupt::BRK);
                }
            }
            NOP => {}
            RTI => {
                self.status = Flags::from_bits_truncate(common::stack_pop(self));
                self.status.remove(Flags::BREAK);
                self.status.insert(Flags::UNUSED);
                self.program_counter = common::stack_pop_u16(self);
            }
            NOP_ALT => match opcode.mode {
                Implicit => {}
                _ => {
                    let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                    let _data: u8 = self.bus_mut().read(addr);
                    if page_cross {
                        self.bus_mut().tick(1);
                    }
                }
            },
            SLO => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let mut data: u8 = self.bus_mut().read(addr);
                common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                data <<= 1;
                self.bus_mut().write(addr, data);
                common::update_flags_zn(self, data);
                common::set_accumulator(self, data | self.accumulator);
            }
            RLA => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let mut data: u8 = self.bus_mut().read(addr);
                let old_carry: bool = self.status.contains(Flags::CARRY);
                common::update_flag_if(self, Flags::CARRY, data >> 7 == 1);
                data <<= 1;
                if old_carry {
                    data |= 1;
                }
                self.bus_mut().write(addr, data);
                common::update_flags_n(self, data);
                common::set_accumulator(self, data & self.accumulator);
            }
            SRE => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let mut data: u8 = self.bus_mut().read(addr);
                common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                data >>= 1;
                self.bus_mut().write(addr, data);
                common::update_flags_zn(self, data);
                common::set_accumulator(self, data ^ self.accumulator);
            }
            RRA => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let mut data: u8 = self.bus_mut().read(addr);
                let old_carry: bool = self.status.contains(Flags::CARRY);
                common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                data >>= 1;
                if old_carry {
                    data |= 0b1000_0000;
                }
                self.bus_mut().write(addr, data);
                common::update_flags_n(self, data);
                common::add_to_accumulator(self, data);
            }
            SAX => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.accumulator & self.index_x;
                self.bus_mut().write(addr, data);
            }
            LAX => {
                let (addr, page_cross): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data);
                self.index_x = self.accumulator;
                if page_cross {
                    self.bus_mut().tick(1);
                }
            }
            DCP => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr).wrapping_sub(1);
                self.bus_mut().write(addr, data);
                common::update_flag_if(self, Flags::CARRY, data <= self.accumulator);
                common::update_flags_zn(self, self.accumulator.wrapping_sub(data));
            }
            ISC => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr).wrapping_add(1);
                self.bus_mut().write(addr, data);
                common::update_flags_zn(self, data);
                common::sub_from_accumulator(self, data);
            }
            ANC => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data & self.accumulator);
                common::update_flag_if(self, Flags::CARRY, self.status.contains(Flags::NEGATIVE));
            }
            ALR => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr) & self.accumulator;
                common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                common::set_accumulator(self, data >> 1);
            }
            ARR => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let mut data: u8 = self.bus_mut().read(addr) & self.accumulator;
                let old_carry: bool = self.status.contains(Flags::CARRY);
                common::update_flag_if(self, Flags::CARRY, data & 1 == 1);
                data >>= 1;
                if old_carry {
                    data |= 0b1000_0000;
                }
                common::set_accumulator(self, data);
                let bit_5: u8 = (self.accumulator >> 5) & 1;
                let bit_6: u8 = (self.accumulator >> 6) & 1;
                common::update_flag_if(self, Flags::CARRY, bit_6 == 1);
                common::update_flag_if(self, Flags::OVERFLOW, bit_5 ^ bit_6 == 1);
            }
            XAA => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::set_accumulator(self, data & self.index_x);
            }
            AXS => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                let result: u8 = (self.index_x & self.accumulator).wrapping_sub(data);
                common::update_flag_if(self, Flags::CARRY, data <= self.index_x & self.accumulator);
                common::update_flags_zn(self, result);
                self.index_x = result
            }
            SBC_NOP => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr);
                common::sub_from_accumulator(self, data);
            }
            AHX => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.accumulator & self.index_x & (addr >> 8) as u8;
                self.bus_mut().write(addr, data);
            }
            SHY => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.index_y & ((addr >> 8) as u8 + 1);
                self.bus_mut().write(addr, data);
            }
            SHX => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.index_x & ((addr >> 8) as u8 + 1);
                self.bus_mut().write(addr, data);
            }
            TAS => {
                self.stack_pointer = self.accumulator & self.index_x;
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = ((addr >> 8) as u8 + 1) & self.stack_pointer;
                self.bus_mut().write(addr, data);
            }
            LAS => {
                let (addr, _): (u16, bool) = opcode.get_operand_address(self);
                let data: u8 = self.bus_mut().read(addr) & self.stack_pointer;
                self.accumulator = data;
                self.index_x = data;
                self.stack_pointer = data;
                common::update_flags_zn(self, data);
            }
            KIL => {
                #[cfg(not(test))]
                {
                    panic!("The `KIL` instruction was executed!");
                }
                #[cfg(test)]
                {
                    self.running = false;
                }
            }
        };
    }
}
