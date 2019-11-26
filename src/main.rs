extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use emu::emu6502::Emu6502;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::Screen;

struct Device {
    cpu: Emu6502,
    bus: Rc<RefCell<Bus>>,
}

impl Device {
    fn new () -> Device {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = Emu6502::new(bus.clone());
        Device {
            cpu,
            bus
        }
    }

    fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.deref().borrow_mut().insert_cartridge(Rc::new(RefCell::new(cartridge)));
        self.cpu.irq();
    }

    fn clock(&mut self) {
        self.cpu.clock();
    }
}

fn main() {
    let mut screen = Screen::new();

    let cart = Cartridge::new("Donkey_Kong.nes");
    let mut device = Device::new();
    device.insert_cartridge(cart);
    
    let mut event_pump = screen.get_events();
    'lock: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == Keycode::Escape {
                        break 'lock;
                    }
                    if keycode.unwrap() == Keycode::C {
                        device.clock();
                    }
                    if keycode.unwrap() == Keycode::M {
                        // print memory dump
                    }
                },
                _ => ()
            }
        }
    }
}
