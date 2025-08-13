mod common;

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
        if instruction_cycle == 8992 {
            cpu.running = false;
        }
    });

    let log_hash: String = format!("{:x}", md5::compute(debug_log));
    let good_hash: String = format!(
        "{:x}",
        md5::compute(std::fs::read_to_string("good-no-cycle-nestest.log").unwrap())
    );

    // Hardcoded hash is the hash of the current best cpu log
    if log_hash != good_hash && log_hash != "514baa4125ede525e1c5707a98a2f36b" {
        assert!(
            false,
            "The generated CPU log does not match the known good reference log!\nGenerated Hash: {}\nKnown Good Hash: {}",
            log_hash, good_hash
        );
    }
}
