extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::cell::RefCell;

use emu::emu6502::Emu6502;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::Screen;

fn main() {
    let mut screen = Screen::new();

    let mut bus = Bus::new();
    let mut reff_bus = RefCell::new(bus);
    let mut emu6502 = Emu6502::new(reff_bus);
    emu6502.clock();
    let i: u8 = 1 << 7;
    println!("{:#010b}", i);
    println!("Hello, world!");
    let mut cart = Cartridge::new("Test.nes");

    screen.print_text();
    let mut event_pump = screen.get_events();
    'lock: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'lock,
                _ => ()
            }
        }
    }
}
