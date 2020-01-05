extern crate sdl2;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{stdin, stdout, Write};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let controller_a = Rc::new(RefCell::new(Controller::new()));
        let bus = Rc::new(RefCell::new(Bus::new(ppu.clone(), controller_a.clone())));
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
        let max = address.saturating_add(offset);
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

    fn clock(&mut self) -> Option<u32> {
        let color = self.ppu.borrow_mut().clock();
        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }
        if self.ppu.borrow().nmi_require() {
            self.cpu.nmi();
            self.ppu.borrow_mut().reset_nmi_require();
        }
        self.clock_counter = self.clock_counter.overflowing_add(1).0;
        color
    }
}

fn main() {
    let pixel_size = 2;
    let  (mut recource_holder, canvas) = RecourceHolder::init(pixel_size);
    let mut screen = Screen::new(&mut recource_holder, canvas, pixel_size);

    let cart = Cartridge::new("dk.nes");
    let mut device = Device::new();
    device.insert_cartridge(cart);
    
    for table in 0 .. 2 {
        for idx in 0 .. 128 * 128 {
            let pixel = device.read_pixel_pattern_table(idx, table);
            screen.set_point_at_sprite_area(pixel, table);
        }
    }

    screen.update();

    let mut clock_num_offset = 0;
    let mut clock_by_num = false;
    let mut auto = false;
    let mut clock_by_frame = false;
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
                        clock_by_num = false;
                        clock_by_frame = false;
                        println!("auto mode: {}", auto);
                    }
                    if keycode.unwrap() == Keycode::F {
                        clock_by_frame = true;
                        clock_by_num = false;
                        auto = false;
                    }
                    if keycode.unwrap() == Keycode::R {
                        device.ppu.borrow_mut().reset();
                        device.cpu.reset();
                    }
                    if keycode.unwrap() == Keycode::D {
                        let debug = device.cpu.get_debug();
                        device.cpu.set_debug(!debug);
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
                    if keycode.unwrap() == Keycode::S {
                        let mut input = String::new();
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let mut address = 0;
                        let mut data = 0;
                        for (i, part) in input.split_whitespace().enumerate() {
                            match i {
                                0 => address = u16::from_str_radix(part.trim(), 16).unwrap(),
                                1 => data = u8::from_str_radix(part.trim(), 16).unwrap(),
                                _ => ()
                            }
                        }
                        device.bus.borrow_mut().write_cpu_ram(address, data);
                    }
                    if keycode.unwrap() == Keycode::N {
                        clock_by_num = true;
                        clock_by_frame = false;
                        auto = false;
                        let mut input = String::new();
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        clock_num_offset = match input.trim().parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                clock_by_num = false;
                                0
                            },
                        }
                    }
                    if keycode.unwrap() == Keycode::I {
                        let mut input = String::new();
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input_value = match u8::from_str_radix(input.trim(), 16) {
                            Ok(value) => value,
                            Err(_) => 0,
                        };
                        device.bus.borrow_mut().write_input_value(input_value);
                    }
                    if keycode.unwrap() == Keycode::P {
                        let mut input = String::new();
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let address = match u16::from_str_radix(input.trim(), 16) {
                            Ok(value) => value,
                            Err(_) => 0,
                        };
                        device.cpu.set_programm_counter(address);
                    }
                    if keycode.unwrap() == Keycode::Num1 {
                        device.ppu.borrow().read_name_table(0);
                    }
                    if keycode.unwrap() == Keycode::Num2 {
                        device.ppu.borrow().read_name_table(1);
                    }
                    if keycode.unwrap() == Keycode::Up {
                        device.controller_a.as_ref().borrow_mut().update_register(0x10);
                    }
                    if keycode.unwrap() == Keycode::Down {
                        device.controller_a.as_ref().borrow_mut().update_register(0x20);
                    }
                    if keycode.unwrap() == Keycode::Left {
                        device.controller_a.as_ref().borrow_mut().update_register(0x40);
                    }
                    if keycode.unwrap() == Keycode::Right {
                        device.controller_a.as_ref().borrow_mut().update_register(0x80);
                    }
                    if keycode.unwrap() == Keycode::Z { // button A
                        device.controller_a.as_ref().borrow_mut().update_register(0x01);
                    }
                    if keycode.unwrap() == Keycode::X { // button B
                        device.controller_a.as_ref().borrow_mut().update_register(0x02);
                    }
                    if keycode.unwrap() == Keycode::LCtrl { // button SELECT
                        device.controller_a.as_ref().borrow_mut().update_register(0x04);
                    }
                    if keycode.unwrap() == Keycode::Space { // button START
                        device.controller_a.as_ref().borrow_mut().update_register(0x08);
                    }
                },
                _ => ()
            }
        }

        if auto {
            while !device.ppu.borrow().frame_complete() {
                match device.clock() {
                    Some(color) => screen.set_point_at_main_area(color),
                    None => (),
                }
                if device.controller_a.borrow().input_access() {
                    break;
                }
            }
            if device.ppu.borrow().frame_complete() {
                screen.update();
                device.ppu.borrow_mut().reset_frame_complete_status();
                device.cpu.reset_complete_status();
            }
        } else if clock_by_frame {
            while !device.ppu.borrow().frame_complete() {
                match device.clock() {
                    Some(color) => screen.set_point_at_main_area(color),
                    None => (),
                }
                if device.controller_a.borrow().input_access() {
                    break;
                }
            }
            if device.ppu.borrow().frame_complete() {
                clock_by_frame = false;
                screen.update();
                device.ppu.borrow_mut().reset_frame_complete_status();
                device.cpu.reset_complete_status();
            }
        } else if clock_by_num {
            let current_clock = device.clock_counter;
            while device.clock_counter < current_clock + clock_num_offset {
                match device.clock() {
                    Some(color) => screen.set_point_at_main_area(color),
                    None => (),
                }
            }
            clock_by_num = false;
            clock_num_offset = 0;
            screen.update();
            device.cpu.reset_complete_status();
            device.ppu.borrow_mut().reset_frame_complete_status();
        } else if manual_clock {
            while !device.cpu.clock_is_complete() {
                match device.clock() {
                    Some(color) => screen.set_point_at_main_area(color),
                    None => (),
                }
            }
            screen.update();
            device.cpu.reset_complete_status();
            manual_clock = false;
        }
    }
}
