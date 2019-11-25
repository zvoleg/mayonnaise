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
    Emu6502::init(Rc::clone(&bus));
    let cart = Cartridge::new("Test.nes");
    (*bus).borrow_mut().insert_cartridge(Rc::new(RefCell::new(cart)));
    
    (*bus).borrow_mut().clock();

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
