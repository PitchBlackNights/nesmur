#[allow(unused_imports)]
pub mod prelude {
    pub use log::{debug, error, info, trace, warn};
    pub use nes::bus::Mem;
    pub use nes::tools;
    pub use nes::tools::NESAccess;
}
use nes::NES;
use nes::cartridge::Rom;
use prelude::*;

pub fn setup_nes(rom_path: &str) -> NES {
    let path: String = format!("tests/roms/{}", rom_path);
    let rom_bytes: Vec<u8> = std::fs::read(path).unwrap();
    let rom: Rom = Rom::new(&rom_bytes).unwrap();
    NES::new(rom)
}

pub fn trace(cpu: &nes::cpu::CPU) -> String {
    use nes::cpu::opcode::AddressingMode::*;
    use nes::cpu::opcode::Instruction::*;
    use nes::cpu::opcode::{OpCode, decode_opcode};
    nes::bus::set_quiet_log(true);

    let opbyte: u8 = cpu.bus().read(cpu.program_counter);
    let opcode: &'static OpCode = decode_opcode(opbyte);

    let begin: u16 = cpu.program_counter;
    let mut hex_dump: Vec<u8> = vec![];
    hex_dump.push(opbyte);

    let (mem_addr, stored_value): (u16, u8) = match opcode.mode {
        Implicit | Accumulator | Immediate => (0, 0),
        _ => {
            let (addr, _): (u16, bool) = opcode.get_absolute_address(cpu, begin + 1);
            (addr, cpu.bus().read(addr))
        }
    };

    let tmp: String = match opcode.len {
        1 => match opcode.byte {
            0x0a | 0x2a | 0x4a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.bus().read(begin + 1);
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
            let address_lo: u8 = cpu.bus().read(begin + 1);
            let address_hi: u8 = cpu.bus().read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address: u16 = cpu.bus().read_u16(begin + 1);

            match opcode.mode {
                Indirect => {
                    if opcode.byte == 0x6C {
                        // jmp indirect
                        let jmp_addr: u16 = if address & 0x00FF == 0x00FF {
                            tools::bytes_to_u16(&[
                                cpu.bus().read(address),
                                cpu.bus().read(address & 0xFF00),
                            ])
                        } else {
                            cpu.bus().read_u16(address)
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

    nes::bus::set_quiet_log(false);
    format!(
        "{:47} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        asm_str, cpu.accumulator, cpu.index_x, cpu.index_y, cpu.status, cpu.stack_pointer,
    )
    .to_ascii_uppercase()
}
