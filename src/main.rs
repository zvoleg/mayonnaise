use std::cell::RefCell;

use emu::emu6502::Emu6502;
use emu::bus::Bus;

fn main() {
    let mut bus = Bus::new();
    let mut reff_bus = RefCell::new(bus);
    let mut emu6502 = Emu6502::new(reff_bus);
    emu6502.clock();
    let i: u8 = 1 << 7;
    println!("{:#010b}", i);
    println!("Hello, world!");
}
