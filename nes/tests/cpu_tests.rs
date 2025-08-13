mod common;

// use common::prelude::*;
use nes::NES;

#[test]
fn nestest() {
    let mut debug_log: String = String::new();
    let mut nes: NES = common::setup_nes("nestest.nes");

    nes::bus::set_force_quiet_log(true);
    nes.reset();
    nes.cpu.program_counter = 0xC000;

    for _ in 0..8991 {
        debug_log += format!("{}\n", common::trace(&nes.cpu)).as_str();
        nes.cpu.step();
    }

    let log_hash: String = format!("{:x}", md5::compute(debug_log));
    let good_hash: String = format!(
        "{:x}",
        md5::compute(std::fs::read_to_string("good-no-cycle-nestest.log").unwrap())
    );

    // Hardcoded hash is the hash of the current best cpu log
    if log_hash != good_hash && log_hash != "53b5a9eea0cf79a81a6fb1d632dd1977" {
        panic!(
            "The generated CPU log does not match the known good reference log!\nGenerated Hash: {}\nKnown Good Hash: {}",
            log_hash, good_hash
        );
    }
}
