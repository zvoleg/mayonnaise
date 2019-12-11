extern crate sdl2;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::io::{stdin, stdout, Write};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use emu::emu6502::Emu6502;
use emu::ppu::Ppu;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::Screen;

struct Device {
    cpu: Emu6502,
    ppu: Ppu,
    bus: Rc<RefCell<Bus>>,
    clock_counter: u32,
}

impl Device {
    fn new () -> Device {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = Emu6502::new(bus.clone());
        let ppu = Ppu::new(bus.clone());
        Device {
            cpu,
            ppu,
            bus,
            clock_counter: 0
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
        let min = address.saturating_sub(offset);
        let max = address.saturating_add(offset);
        for i in min..max {
            if i == address {
                print!(" > ");
            } else {
                print!("   ");
            }
            println!("{:04X} - {:02X}", i, self.bus.deref().borrow_mut().read_cpu_ram(i));
        }
    }

    fn clock(&mut self) -> u32 {
        let color = self.ppu.clock();
        let (res, _) = self.clock_counter.overflowing_add(1);
        self.clock_counter = res;
        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }
        color.unwrap()
    }
}

fn main() {
    let pixel_size = 1;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("mayonnaise", 256 * pixel_size + 20 + 128, 240 * pixel_size).
        position_centered().
        opengl().
        build().unwrap();
    let canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut screen = Screen::new(sdl, &texture_creator, canvas, pixel_size);

    let cart = Cartridge::new("Donkey_Kong.nes");
    let mut device = Device::new();
    device.insert_cartridge(cart);
    
    let mut auto = false;
    let mut manual_clock = false;
    let mut event_pump = screen.get_events();
    'lock: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == Keycode::Escape {
                        break 'lock;
                    }
                    if keycode.unwrap() == Keycode::C {
                        manual_clock = true;
                    }
                    if keycode.unwrap() == Keycode::A {
                        auto = !auto;
                        println!("auto mode: {}", auto);
                    }
                    if keycode.unwrap() == Keycode::R {
                        device.cpu.reset();
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
        if device.ppu.nmi_require() {
            screen.update();
            device.cpu.nmi();
            device.ppu.reset_nmi();
        }
        if auto || manual_clock {
            screen.set_point_at_main_area(device.clock());
            manual_clock = false;
        }
    }
}
