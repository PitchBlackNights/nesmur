mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}

// pub mod apu;
pub mod bus;
pub mod cpu;
// pub mod ppu;

// use crate::apu::APU;
use crate::bus::Bus;
use crate::cpu::CPU;
// use crate::ppu::PPU;


struct NES {
   // pub apu: APU,
   pub bus: Bus,
   pub cpu: CPU,
   // pub ppu: PPU,
}