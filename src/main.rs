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
use emu::environment::{RecourceHolder, Screen};

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
        self.ppu.read_all_sprites(0);
        self.ppu.read_all_sprites(1);
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

    fn read_pixel_pattern_table(&self, idx: usize, table: u8) -> u32 {
        let pattern = &self.ppu.get_pattern_table(table);
        match pattern[idx] {
            0 => 0x222222,
            1 => 0x5555AA,
            2 => 0xDDCCAA,
            3 => 0x55AA99,
            _ => 0
        }
    }

    fn clock(&mut self) -> Option<u32> {
        let color = self.ppu.clock();
        let (res, _) = self.clock_counter.overflowing_add(1);
        self.clock_counter = res;
        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }
        color
    }
}

fn main() {
    let pixel_size = 2;
    let  (mut recource_holder, canvas) = RecourceHolder::init(pixel_size);
    let mut screen = Screen::new(&mut recource_holder, canvas, pixel_size);

    let cart = Cartridge::new("Test.nes");
    let mut device = Device::new();
    device.insert_cartridge(cart);
    
    for table in 0 .. 2 {
        for idx in 0 .. 128 * 128 {
            let pixel = device.read_pixel_pattern_table(idx, table);
            screen.set_point_at_sprite_area(pixel, table);
        }
    }

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

        if auto {
            while !device.ppu.nmi_require() {
                match device.clock() {
                    Some(color) => screen.set_point_at_main_area(color),
                    None => (),
                }
            }
            if device.ppu.nmi_require() {
                screen.update();
                device.cpu.nmi();
                device.ppu.reset_nmi();
            }
        } else if manual_clock {
            match device.clock() {
                Some(color) => screen.set_point_at_main_area(color),
                None => (),
            }
            screen.update();
            if device.ppu.nmi_require() {
                device.cpu.nmi();
                device.ppu.reset_nmi();
            }
            manual_clock = false;
        }
    }
}
