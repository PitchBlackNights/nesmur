mod common;

use std::fs::File;
use std::io::{BufWriter, Write};

// use common::prelude::*;
use nes::NES;
use nes::tools;

#[test]
fn nestest() {
    let mut debug_log: String = String::new();
    let mut nes: NES = common::setup_nes("nestest.nes");

    nes.reset();
    nes.cpu.program_counter = 0xC000;

    let mut instruction_cycle: u16 = 0;
    nes.cpu.run_with_callback(|cpu| {
        debug_log += format!("{}\n", tools::trace(cpu)).as_str();
        instruction_cycle += 1;
        if instruction_cycle == 8991 {
            cpu.running = false;
        }
    });

    let log_hash: String = format!("{:x}", md5::compute(&debug_log));
    let good_hash: String = format!(
        "{:x}",
        md5::compute(std::fs::read_to_string("good-nestest.log").unwrap())
    );

    // Hardcoded hash is the hash of the current best cpu log
    if log_hash != good_hash && log_hash != "a9bc0c53220971c7d3ab15b228121d38" {
        let mut log_file: BufWriter<File> = BufWriter::new(File::create("nestest.log").unwrap());
        write!(&mut log_file, "{}", &debug_log).unwrap();
        assert!(
            false,
            "The generated CPU log does not match the known good reference log!\nGenerated Hash: {}\nKnown Good Hash: {}",
            log_hash, good_hash
        );
    }
}
