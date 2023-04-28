#[macro_use]
extern crate spriter;
#[macro_use]
extern crate log;
extern crate env_logger;

use spriter::Key;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write};
use std::time::Duration;

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
            let mut pointer_sign = "   ";
            if i == address {
                pointer_sign = " > ";
            }
            info!("{}{:04X} - {:02X}", pointer_sign, i, self.bus.borrow().read_only_data(i));
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
            if !self.bus.borrow().dma_enable() {
                self.cpu.clock();
            } else {
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

        self.update_pallettes();
    }

    fn update_pallettes(&mut self) {
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

    fn handle_keys(&mut self) {
        if_pressed!(Key::Escape, {spriter::program_stop()});
        if_pressed!(Key::C, {self.clock_type = ClockType::Manual});
        if_pressed!(Key::A, {
            let clock_type = self.clock_type;
            self.clock_type = match clock_type {
                ClockType::Auto => ClockType::Undefined,
                _ => ClockType::Auto,
            };
            info!("clock type: {:?}", self.clock_type);
        });
        if_pressed!(Key::F, {self.clock_type = ClockType::Frame});
        if_pressed!(Key::N, {
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<u32>() {
                Ok(num) => self.clock_type = ClockType::Amount(num),
                Err(_) => self.clock_type = ClockType::Undefined,
            }
        });
        if_pressed!(Key::R, {
            self.ppu.borrow_mut().reset();
            self.cpu.reset();
        });
        if_pressed!(Key::D, {self.cpu.debug = !self.cpu.debug});
        if_pressed!(Key::E, {
            let debug = self.ppu.borrow().debug;
            self.ppu.borrow_mut().debug = !debug;
        });
        if_pressed!(Key::V, {
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let parse_result = u16::from_str_radix(input.trim(), 16);
            match parse_result {
                Ok(idx) => self.print_memory_by_address(idx, 3),
                Err(_)  => info!("index must be in hex format"),
            }
        });
        if_pressed!(Key::S, {
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let mut input_parts = input.split_whitespace();
            let address = u16::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
            let data = u8::from_str_radix(input_parts.next().unwrap().trim(), 16).unwrap();
            self.bus.borrow_mut().write_cpu_ram(address, data);
            info!("write: {:04X} {:02X}", address, data);
        });
        if_pressed!(Key::P, {
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let address = match u16::from_str_radix(input.trim(), 16) {
                Ok(value) => value,
                Err(_) => 0,
            };
            self.cpu.set_programm_counter(address);
        });
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
    env_logger::init();

    let pixel_size = 3;
    let width = 522 * pixel_size;
    let height = 242 * pixel_size;
    let (runner, mut window) = spriter::init("mayonnaise", width, height);
    let screen = Screen::new(&mut window, pixel_size);

    let cart = Cartridge::new("af.nes");
    let mut device = Device::new(screen);
    info!("device created");
    device.insert_cartridge(cart);
    
    for table in 0 .. 2 {
        for idx in 0 .. 128 * 128 {
            let pixel = device.read_pixel_pattern_table(idx, table);
            device.screen.set_point_at_sprite_area(pixel, table);
        }
    }

    runner.run(window, move |_duration| {
        device.handle_keys();
        let clock_type = device.clock_type;
        let mut update_screen = false;
        match clock_type {
            ClockType::Manual => {
                while !device.cpu.clock_complete {
                    device.clock();
                }
                update_screen = true;
                device.clock_type = ClockType::Undefined;
            },
            ClockType::Auto => {
                while !update_screen {
                    device.clock();
                    update_screen = device.ppu.borrow().frame_complete;
                }
                std::thread::sleep(Duration::from_secs_f64(1.0/120.0)); // simple approach for a fiting framerate
            },
            ClockType::Frame => {
                while !device.ppu.borrow().frame_complete {
                    device.clock();
                }
                update_screen = true;
                device.clock_type = ClockType::Undefined;
            },
            ClockType::Amount(num) => {
                let current_clock = device.clock_counter;
                while device.clock_counter < current_clock + num {
                    device.clock();
                }
                update_screen = true;
                device.clock_type = ClockType::Undefined;
            },
            ClockType::Undefined => (),
        }
        
        device.ppu.borrow_mut().frame_complete = false;
        device.cpu.clock_complete = false;
        update_screen
    });
}
