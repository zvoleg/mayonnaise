extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::cell::RefCell;

use emu::emu6502::Emu6502;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::Screen;

fn main() {
    let mut screen = Screen::new();

    let bus = Rc::new(RefCell::new(Bus::new()));
    let mut emu6502 = Emu6502::new(Rc::clone(&bus));
    let cart = Cartridge::new("Donkey_Kong.nes");
    (*bus).borrow_mut().set_cpu(Box::new(emu6502));
    (*bus).borrow_mut().insert_cartridge(Box::new(cart));

    let i: u8 = 1 << 7;
    println!("{:#010b}", i);
    println!("Hello, world!");

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
