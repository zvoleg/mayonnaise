extern crate spriter;

use spriter::Program;
use spriter::Key;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write};

use emu::emu6502::Emu6502;
use emu::ppu::Ppu;
use emu::bus::Bus;
use emu::program::Cartridge;
use emu::environment::screen::Screen;
use emu::environment::control::Controller;

struct Device {
    screen: Screen,
    cpu: Emu6502,
    ppu: Rc<RefCell<Ppu>>,
    bus: Rc<RefCell<Bus>>,
    clock_type: ClockType,
    clock_counter: u32,
    is_run: bool,
}

impl Device {
    fn new (screen: Screen) -> Device {
        let controller_a = Controller::new();
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let bus = Rc::new(RefCell::new(Bus::new(controller_a, ppu.clone())));
        let cpu = Emu6502::new(bus.clone());
        Device {
            screen,
            cpu,
            ppu,
            bus,
            clock_type: ClockType::Undefined,
            clock_counter: 0,
            is_run: true,
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
        let pattern = &self.ppu.borrow().patterns[table as usize];
        match pattern[idx] {
            0 => 0x222222,
            1 => 0x5555AA,
            2 => 0xDDCCAA,
            3 => 0x55AA99,
            _ => 0
        }
    }

    fn clock(&mut self) {
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
        self.clock_counter = self.clock_counter.wrapping_add(1);

        match color {
            Some(color) => self.screen.set_point_at_main_area(color),
            None => ()
        }

        if self.ppu.borrow().update_pallettes {
            let main_color_addr = self.ppu.borrow().read_ppu(0x3F00);
            let main_color = self.ppu.borrow().pallette_colors[main_color_addr as usize];
            self.screen.set_point_at_main_color_area(main_color);

            (0..16).for_each(|x| {
                let color_addr = self.ppu.borrow().read_ppu(0x3F00 + x + 1);
                let color = self.ppu.borrow().pallette_colors[color_addr as usize];
                self.screen.set_point_at_background_color_area((x / 4) as usize, color);
            });
            (16..32).for_each(|x| {
                let color_addr = self.ppu.borrow().read_ppu(0x3F00 + x + 1);
                let color = self.ppu.borrow().pallette_colors[color_addr as usize];
                self.screen.set_point_at_sprite_color_area(((x - 16) / 4) as usize, color);
            });
            self.ppu.borrow_mut().update_pallettes = false;
        }
    }
}

impl Program for Device {
    fn run(&mut self) {
        let clock_type = self.clock_type;
        match clock_type {
            ClockType::Manual => {
                while !self.cpu.clock_complete {
                    self.clock();
                }
                self.clock_type = ClockType::Undefined;
            },
            ClockType::Auto => {
                while !self.ppu.borrow().frame_complete {
                    self.clock();
                }
                self.ppu.borrow_mut().frame_complete = false;
            },
            ClockType::Frame => {
                while !self.ppu.borrow().frame_complete {
                    self.clock();
                }
                self.clock_type = ClockType::Undefined;
            },
            ClockType::Amount(num) => {
                let current_clock = self.clock_counter;
                while self.clock_counter < current_clock + num {
                    self.clock();
                }
                self.clock_type = ClockType::Undefined;
            },
            ClockType::Undefined => (),
        }
        
        self.ppu.borrow_mut().frame_complete = false;
        self.cpu.clock_complete = false;
        self.screen.window.borrow_mut().swap_buffers();
    }
    
    fn is_execute(&self) -> bool {
        self.is_run
    }

    fn handle_key_input(&mut self, key: Key) {
        match key {
            Key::Escape => self.is_run = false,
            Key::C => self.clock_type = ClockType::Manual,
            Key::A => {
                let clock_type = self.clock_type;
                self.clock_type = match clock_type {
                    ClockType::Auto => ClockType::Undefined,
                    _ => ClockType::Auto,
                };
                println!("clock type: {:?}", self.clock_type);
            },
            Key::F => self.clock_type = ClockType::Frame,
            Key::N => {
                let mut input = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<u32>() {
                    Ok(num) => self.clock_type = ClockType::Amount(num),
                    Err(_) => self.clock_type = ClockType::Undefined,
                }
            },
            Key::R => {
                self.ppu.borrow_mut().reset();
                self.cpu.reset();
            },
            Key::D => self.cpu.debug = !self.cpu.debug,
            Key::E => {
                let debug = self.ppu.borrow().debug;
                self.ppu.borrow_mut().debug = !debug;
            },
            Key::V => {
                let mut input = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let parse_result = u16::from_str_radix(input.trim(), 16);
                match parse_result {
                    Ok(idx) => self.print_memory_by_address(idx, 3),
                    Err(_)  => println!("index must be in hex format"),
                }
            },
            Key::S => {
                let mut input = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let mut input_parts = input.split_whitespace();
                let address = u16::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
                let data = u8::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
                self.bus.borrow_mut().write_cpu_ram(address, data);
                println!("write: {:04X} {:02X}", address, data);
            },
            Key::P => {
                let mut input = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let address = match u16::from_str_radix(input.trim(), 16) {
                    Ok(value) => value,
                    Err(_) => 0,
                };
                self.cpu.set_programm_counter(address);
            },
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ClockType {
    Manual,
    Auto,
    Frame,
    Amount(u32),
    Undefined,
}

fn main() {
    let pixel_size = 3;
    let width = 522 * pixel_size;
    let height = 242 * pixel_size;
    let (window, handler) = spriter::init("mayonnaise", width, height, false);
    let window = Rc::new(RefCell::new(window));
    let screen = Screen::new(window.clone(), pixel_size);

    let cart = Cartridge::new("smb.nes");
    let device = Rc::new(RefCell::new(Device::new(screen)));
    device.borrow_mut().insert_cartridge(cart);
    
    for table in 0 .. 2 {
        for idx in 0 .. 128 * 128 {
            let pixel = device.borrow().read_pixel_pattern_table(idx, table);
            device.borrow_mut().screen.set_point_at_sprite_area(pixel, table);
        }
    }

    handler.run(window.clone(), Some(device));
}
