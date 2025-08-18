use crate::bus::Bus;
use crate::cartridge::ROM;
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::prelude::*;
use crate::{BoxMapper, BoxNESDevice};
use crate::{apu::APU, memory::Memory};
use once_cell::sync::Lazy;
use std::cell::{Ref, RefMut};

pub static NON_READABLE_ADDR: Lazy<Vec<u16>> = Lazy::new(|| {
    vec![
        0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x4016, 0x4017,
    ]
});

#[rustfmt::skip]
pub trait NESAccess<'a> {
    fn bus(&self) -> Ref<Bus<'a>> { panic!("Access to `Bus` is prohibited") }
    fn bus_mut(&self) -> RefMut<Bus<'a>> { panic!("Mutable access to `Bus` is prohibited") }
    fn apu(&self) -> Ref<APU> { panic!("Access to `APU` is prohibited") }
    fn apu_mut(&self) -> RefMut<APU> { panic!("Mutable access to `APU` is prohibited") }
    fn ppu(&self) -> Ref<PPU> { panic!("Access to `PPU` is prohibited") }
    fn ppu_mut(&self) -> RefMut<PPU> { panic!("Mutable access to `PPU` is prohibited") }
    fn rom(&self) -> Ref<ROM> { panic!("Access to `Rom` is prohibited") }
    fn rom_mut(&self) -> RefMut<ROM> { panic!("Mutable access to `Rom` is prohibited") }
    fn mapper(&self) -> Ref<BoxMapper> { panic!("Access to `Mapper` is prohibited") }
    fn mapper_mut(&self) -> RefMut<BoxMapper> { panic!("Mutable access to `Mapper` is prohibited") }
    fn memory(&self) -> Ref<Memory> { panic!("Access to `Memory` is prohibited") }
    fn memory_mut(&self) -> RefMut<Memory> { panic!("Mutable access to `Memory` is prohibited") }
    fn device1(&self) -> Ref<BoxNESDevice> { panic!("Access to `Device 1` is prohibited") }
    fn device1_mut(&self) -> RefMut<BoxNESDevice> { panic!("Mutable access to `Device 1` is prohibited") }
    fn device2(&self) -> Ref<BoxNESDevice> { panic!("Access to `Device 2` is prohibited") }
    fn device2_mut(&self) -> RefMut<BoxNESDevice> { panic!("Mutable access to `Device 2` is prohibited") }
}

pub fn u16_to_bytes(value: u16) -> [u8; 2] {
    [(value & 0x00FF) as u8, (value >> 8) as u8]
}

pub fn bytes_to_u16(bytes: &[u8; 2]) -> u16 {
    ((bytes[1] as u16) << 8) | (bytes[0] as u16)
}

pub fn vec_to_u16(bytes: &Vec<u8>) -> u16 {
    let bytes: [u8; 2] = bytes.to_owned().try_into().unwrap();
    ((bytes[1] as u16) << 8) | (bytes[0] as u16)
}

pub fn page_cross(addr1: u16, addr2: u16) -> bool {
    addr1 & 0xFF00 != addr2 & 0xFF00
}

pub fn format_byte_size(bytes: usize) -> String {
    let display_scale: f32 = match bytes {
        ..512 => 0.0,                        // 0 Bytes -> 511 Bytes
        512..524_288 => 1024.0,              // 0.5 KiB -> 511.99 KiB
        524_288..536_870_912 => 1_048_576.0, // 0.5 MiB -> 511.99 MiB
        536_870_912.. => 1_073_741_824.0,    // 0.5 GiB -> 9999999+ GiB
    };
    let size_unit: &str = match display_scale {
        0.0 => "B",
        1024.0 => "KiB",
        1_048_576.0 => "MiB",
        1_073_741_824.0 => "GiB",
        _ => panic!("This shouldn't happen!"),
    };
    match display_scale {
        0.0 => format!("{} {}", bytes, size_unit),
        _ => format!("{:.2} {}", bytes as f32 / display_scale, size_unit),
    }
}

pub fn trace(cpu: &CPU) -> String {
    use crate::cpu::opcode::AddressingMode::*;
    use crate::cpu::opcode::Instruction::*;
    use crate::cpu::opcode::{OpCode, decode_opcode};

    let prev_bus_quiet_log: bool = crate::bus::get_quiet_log();
    crate::bus::set_quiet_log(true);

    let opbyte: u8 = cpu.bus_mut().read(cpu.program_counter);
    let opcode: &'static OpCode = decode_opcode(opbyte);

    let begin: u16 = cpu.program_counter;
    let mut hex_dump: Vec<u8> = vec![];
    hex_dump.push(opbyte);

    let (mem_addr, stored_value): (u16, u8) = match opcode.mode {
        Implicit | Accumulator | Immediate => (0, 0),
        _ => {
            let (addr, _): (u16, bool) = opcode.get_absolute_address(cpu, begin + 1);

            if !NON_READABLE_ADDR.contains(&addr) {
                (addr, cpu.bus_mut().read(addr))
            } else {
                (addr, 0)
            }
        }
    };

    let tmp: String = match opcode.len {
        1 => match opcode.byte {
            0x0A | 0x2A | 0x4A | 0x6A => String::from("A "),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.bus_mut().read(begin + 1);
            hex_dump.push(address);

            match opcode.mode {
                Immediate => format!("#${:02X}", address),
                ZeroPage => format!("${:02X} = {:02X}", mem_addr, stored_value),
                ZeroPage_X => format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    address, mem_addr, stored_value
                ),
                ZeroPage_Y => format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    address, mem_addr, stored_value
                ),
                Indirect_X => format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    address,
                    address.wrapping_add(cpu.index_x),
                    mem_addr,
                    stored_value
                ),
                Indirect_Y => format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    address,
                    mem_addr.wrapping_sub(cpu.index_y as u16),
                    mem_addr,
                    stored_value
                ),
                Relative => {
                    // assuming local jumps: BNE, BVS, etc....
                    let address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04X}", address)
                }
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02X}",
                    opcode.mode, opcode.byte
                ),
            }
        }
        3 => {
            let address_lo: u8 = cpu.bus_mut().read(begin + 1);
            let address_hi: u8 = cpu.bus_mut().read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address: u16 = cpu.bus_mut().read_u16(begin + 1);

            match opcode.mode {
                Indirect => {
                    if opcode.byte == 0x6C {
                        // jmp indirect
                        let jmp_addr: u16 = if address & 0x00FF == 0x00FF {
                            let mut temp_bus: RefMut<'_, Bus<'_>> = cpu.bus_mut();
                            tools::bytes_to_u16(&[
                                temp_bus.read(address),
                                temp_bus.read(address & 0xFF00),
                            ])
                        } else {
                            cpu.bus_mut().read_u16(address)
                        };
                        format!("(${:04X}) = {:04X}", address, jmp_addr)
                    } else {
                        format!("${:04X}", address)
                    }
                }
                Absolute => match opcode.byte {
                    0x4C | 0x20 => {
                        format!("${:04X}", mem_addr)
                    }
                    _ => {
                        format!("${:04X} = {:02X}", mem_addr, stored_value)
                    }
                },
                Absolute_X => format!(
                    "${:04X},X @ {:04X} = {:02X}",
                    address, mem_addr, stored_value
                ),
                Absolute_Y => format!(
                    "${:04X},Y @ {:04X} = {:02X}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02X}",
                    opcode.mode, opcode.byte
                ),
            }
        }
        _ => String::from(""),
    };

    let undoc_marker: &'static str = match opcode.instruction {
        NOP_ALT | SLO | RLA | SRE | RRA | SAX | LAX | DCP | ISC | ANC | ALR | ARR | XAA | AXS
        | SBC_NOP | AHX | SHY | SHX | TAS | LAS | KIL => "*",
        _ => " ",
    };
    let hex_str: String = hex_dump
        .iter()
        .map(|z| format!("{:02X}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str: String = format!(
        "{:04X}  {:8} {}{} {}",
        begin, hex_str, undoc_marker, opcode.mnemonic, tmp
    )
    .trim()
    .to_string();

    crate::bus::set_quiet_log(prev_bus_quiet_log);
    format!(
        "{:47} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:>3},{:>3} CYC:{}",
        asm_str,
        cpu.accumulator,
        cpu.index_x,
        cpu.index_y,
        cpu.status,
        cpu.stack_pointer,
        cpu.bus().ppu().scanline,
        cpu.bus().ppu().cycles,
        cpu.bus().cpu_cycles
    )
    .to_ascii_uppercase()
}

pub fn format_mem(memory: &[u8], start_addr: u16, end_addr: u16) -> String {
    let mut buffer: String = String::from(
        "ADDR   00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F  ASCII\n-------------------------------------------------------------------------\n",
    );
    for line_num in (start_addr >> 4usize)..(end_addr >> 4usize) + 1 {
        let shift_line_num: u16 = line_num << 4usize;
        let mut line: String = format!("{:04X}:  ", line_num << 4usize);

        for byte_num in 0x00..0x07 + 1 {
            if shift_line_num + byte_num < start_addr || shift_line_num + byte_num > end_addr {
                line += "   ";
            } else {
                line += format!("{:02X} ", memory[(shift_line_num + byte_num) as usize]).as_str();
            }
        }
        line += " ";

        for byte_num in 0x08..0x0F + 1 {
            if shift_line_num + byte_num < start_addr || shift_line_num + byte_num > end_addr {
                line += "   ";
            } else {
                line += format!("{:02X} ", memory[(shift_line_num + byte_num) as usize]).as_str();
            }
        }
        line += " ";

        for byte_num in 0x00..0x0F + 1 {
            if shift_line_num + byte_num < start_addr || shift_line_num + byte_num > end_addr {
                line += " ";
            } else {
                let ascii_byte: u8 = memory[(shift_line_num + byte_num) as usize];
                let ascii_char: String = if 0x20 >= ascii_byte || ascii_byte >= 0x7E {
                    String::from(0x2E as char) // Char: `.`
                } else {
                    String::from(ascii_byte as char)
                };

                line += ascii_char.as_str();
            }
        }
        buffer += (line + "\n").as_str();
    }

    buffer
}
