extern crate sdl2;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write};
use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

use emu::emu6502::Emu6502;
use emu::ppu::Ppu;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::screen::{RecourceHolder, Screen};
use emu::environment::control::Controller;

struct Device {
    cpu: Emu6502,
    ppu: Rc<RefCell<Ppu>>,
    controller_a: Rc<RefCell<Controller>>,
    bus: Rc<RefCell<Bus>>,
    clock_counter: u32,
}

impl Device {
    fn new () -> Device {
        let controller_a = Rc::new(RefCell::new(Controller::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let bus = Rc::new(RefCell::new(Bus::new(controller_a.clone(), ppu.clone())));
        let cpu = Emu6502::new(bus.clone());
        Device {
            cpu,
            ppu,
            controller_a,
            bus,
            clock_counter: 0
        }
    }

    fn insert_cartridge(&mut self, cartridge: Cartridge) {
        let cartridge = Rc::new(RefCell::new(cartridge));
        self.bus.borrow_mut().insert_cartridge(cartridge.clone());
        self.ppu.borrow_mut().insert_cartridge(cartridge.clone());
        self.cpu.reset();
        self.ppu.borrow_mut().read_all_sprites(0);
        self.ppu.borrow_mut().read_all_sprites(1);
    }

    fn print_memory_by_address(&self, address: u16, offset: u16) {
        let min = address.saturating_sub(offset);
        let max = address.saturating_add(offset + 1);
        for i in min..max {
            if i == address {
                print!(" > ");
            } else {
                print!("   ");
            }
            println!("{:04X} - {:02X}", i, self.bus.borrow().read_only_data(i));
        }
    }

    fn read_pixel_pattern_table(&self, idx: usize, table: u8) -> u32 {
        let pattern = &self.ppu.borrow().get_pattern_table(table);
        match pattern[idx] {
            0 => 0x222222,
            1 => 0x5555AA,
            2 => 0xDDCCAA,
            3 => 0x55AA99,
            _ => 0
        }
    }

    fn clock(&mut self, screen: &mut Screen) {
        let color = self.ppu.borrow_mut().clock();
        if self.clock_counter % 3 == 0 {
            if self.bus.borrow().dma_enable() {
                if self.bus.borrow().dma_wait_clock() {
                    if self.clock_counter % 2 == 1 {
                        self.bus.borrow_mut().set_dma_wait_clock(false);
                    }
                } else {
                    if self.clock_counter % 2 == 0 {
                        self.bus.borrow_mut().read_dma_byte();
                    } else {
                        self.bus.borrow_mut().write_dma_byte();
                    }
                }
            } else {
                self.cpu.clock();
            }
        }
        if self.ppu.borrow().nmi_require() {
            self.cpu.nmi();
            self.ppu.borrow_mut().reset_nmi_require();
        }
        self.clock_counter = self.clock_counter.overflowing_add(1).0;

        match color {
            Some(color) => screen.set_point_at_main_area(color),
            None => ()
        }

        if self.ppu.borrow().update_pallettes {
            let main_color_addr = self.ppu.borrow().read_ppu(0x3F00);
            let main_color = self.ppu.borrow().pallette_colors[main_color_addr as usize];
            screen.set_point_at_main_color_area(main_color);

            (0..16).for_each(|x| {
                let color_addr = self.ppu.borrow().read_ppu(0x3F00 + x +1);
                let color = self.ppu.borrow().pallette_colors[color_addr as usize];
                screen.set_point_at_background_color_area((x / 4) as usize, color);
            });
            (16..32).for_each(|x| {
                let color_addr = self.ppu.borrow().read_ppu(0x3F00 + x + 1);
                let color = self.ppu.borrow().pallette_colors[color_addr as usize];
                screen.set_point_at_sprite_color_area(((x - 16) / 4) as usize, color);
            });
            self.ppu.borrow_mut().update_pallettes = false;
        }
    }
}

#[derive(Debug)]
enum ClockType {
    Manual,
    Auto,
    Frame,
    Amount(u32),
    Undefined,
}

fn main() {
    let pixel_size = 2;
    let  (mut recource_holder, canvas) = RecourceHolder::init(pixel_size);
    let mut screen = Screen::new(&mut recource_holder, canvas, pixel_size);

    let cart = Cartridge::new("smb.nes");
    let mut device = Device::new();
    device.insert_cartridge(cart);
    
    for table in 0 .. 2 {
        for idx in 0 .. 128 * 128 {
            let pixel = device.read_pixel_pattern_table(idx, table);
            screen.set_point_at_sprite_area(pixel, table);
        }
    }

    screen.update();

    let mut clock_type = ClockType::Undefined;
    let mut event_pump = screen.get_events();
    'lock: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'lock,
                Event::KeyDown { keycode: Some(Keycode::C), .. } => clock_type = ClockType::Manual,
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    clock_type = match clock_type {
                        ClockType::Auto => ClockType::Undefined,
                        _ => ClockType::Auto,
                    };
                    println!("clock type: {:?}", clock_type);
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => clock_type = ClockType::Frame,
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    match input.trim().parse::<u32>() {
                        Ok(num) => clock_type = ClockType::Amount(num),
                        Err(_) => clock_type = ClockType::Undefined,
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    device.ppu.borrow_mut().reset();
                    device.cpu.reset();
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    device.cpu.debug = !device.cpu.debug;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    let debug = device.ppu.borrow().debug;
                    device.ppu.borrow_mut().debug = !debug;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    let parse_result = u16::from_str_radix(input.trim(), 16);
                    match parse_result {
                        Ok(idx) => device.print_memory_by_address(idx, 3),
                        Err(_)  => println!("index must be in hex format"),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    let mut input_parts = input.split_whitespace();
                    let address = u16::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
                    let data = u8::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
                    device.bus.borrow_mut().write_cpu_ram(address, data);
                },
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    let input_value = match u8::from_str_radix(input.trim(), 16) {
                        Ok(value) => value,
                        Err(_) => 0,
                    };
                    device.bus.borrow_mut().write_input_value(input_value);
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    let address = match u16::from_str_radix(input.trim(), 16) {
                        Ok(value) => value,
                        Err(_) => 0,
                    };
                    device.cpu.set_programm_counter(address);
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => device.ppu.borrow().read_name_table(0),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => device.ppu.borrow().read_name_table(1),
                _ => (),
            }
        }

        let keys: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();
        for key in keys {
            match key {
                Scancode::Up => device.controller_a.as_ref().borrow_mut().update_register(0x10),
                Scancode::Down => device.controller_a.as_ref().borrow_mut().update_register(0x20),
                Scancode::Left => device.controller_a.as_ref().borrow_mut().update_register(0x40),
                Scancode::Right => device.controller_a.as_ref().borrow_mut().update_register(0x80),
                Scancode::Z => device.controller_a.as_ref().borrow_mut().update_register(0x01),
                Scancode::X => device.controller_a.as_ref().borrow_mut().update_register(0x02),
                Scancode::LCtrl => device.controller_a.as_ref().borrow_mut().update_register(0x04),
                Scancode::Space => device.controller_a.as_ref().borrow_mut().update_register(0x08),
                _ => ()
            }
        }

        match clock_type {
            ClockType::Manual => {
                while !device.cpu.clock_complete {
                    device.clock(&mut screen);
                }
                clock_type = ClockType::Undefined;
            },
            ClockType::Auto => {
                while !device.ppu.borrow().frame_complete {
                    device.clock(&mut screen);
                    if device.controller_a.borrow().input_access() {
                        break;
                    }
                }
            },
            ClockType::Frame => {
                while !device.ppu.borrow().frame_complete {
                    device.clock(&mut screen);
                }
                clock_type = ClockType::Undefined;
            },
            ClockType::Amount(num) => {
                let current_clock = device.clock_counter;
                while device.clock_counter < current_clock + num {
                    device.clock(&mut screen);
                }
                clock_type = ClockType::Undefined;
            },
            ClockType::Undefined => (),
        }
        if device.ppu.borrow().frame_complete || device.cpu.clock_complete {
            screen.update();
            device.ppu.borrow_mut().frame_complete = false;
            device.cpu.clock_complete = false;
        }
    }
}
