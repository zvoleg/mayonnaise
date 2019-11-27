extern crate sdl2;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::io::{stdin, stdout, Write};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
        self.cpu.reset();
    }

    fn print_memory_dump(&self) {
        self.print_memory_by_address(self.cpu.get_program_counter(), 10);
    }

    fn print_memory_by_address(&self, address: u16, offset: u16) {
        let min = address - offset;
        let max = address + offset;
        for i in min..max {
            if i == address {
                print!(" > ");
            } else {
                print!("   ");
            }
            println!("{:04X} - {:02X}", i, self.bus.deref().borrow_mut().read_cpu_ram(i));
        }
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
                        device.print_memory_dump();
                    }
                    if keycode.unwrap() == Keycode::V {
                        let mut input = String::new();
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let parse_result = u16::from_str_radix(input.trim(), 16);
                        match parse_result {
                            Ok(idx) => device.print_memory_by_address(idx, 2),
                            Err(_)  => println!("index must be in hex format"),
                        }
                    }
                },
                _ => ()
            }
        }
    }
}
