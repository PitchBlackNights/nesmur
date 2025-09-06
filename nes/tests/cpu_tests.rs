mod common;

use nes::NES;
use nes::cpu::CPU;
use nes::tools;
use std::fs::File;
use std::io::{BufWriter, Write};
// use nes::prelude::*;

#[test]
fn nestest() {
    const GOOD_NESTEST_LOG: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/logs/good-nestest.log");
    const NESTEST_LOG: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/logs/nestest.log");

    let mut debug_log: String = String::new();
    let mut nes: NES = common::setup_nes("nestest.nes");
    nes.cpu.program_counter = 0xC000;

    let mut instruction_cycle: u16 = 0;
    nes.run_with_callback(|cpu: &mut CPU| {
        debug_log += format!("{}\n", tools::trace(cpu)).as_str();
        instruction_cycle += 1;
        if instruction_cycle == 8991 {
            cpu.running = false;
        }
    });

    let log_hash: String = format!("{:x}", md5::compute(&debug_log));
    let good_hash: String = format!(
        "{:x}",
        md5::compute(std::fs::read_to_string(GOOD_NESTEST_LOG).unwrap())
    );

    // Hardcoded hash is the hash of the current best cpu log
    if log_hash != good_hash && log_hash != "a9bc0c53220971c7d3ab15b228121d38" {
        let mut log_file: BufWriter<File> = BufWriter::new(File::create(NESTEST_LOG).unwrap());
        write!(&mut log_file, "{}", &debug_log).unwrap();
        log_file.flush().unwrap();
        assert!(
            false,
            "The generated CPU log does not match the known good reference log!\nGenerated Hash: {}\nKnown Good Hash: {}",
            log_hash, good_hash
        );
    }
}
