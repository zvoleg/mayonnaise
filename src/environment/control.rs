extern crate spriter;

use spriter::{Key, handler};

pub struct Controller {
    register: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            register: 0,
        }
    }

    pub fn update_register_by_input(&mut self) {
        let mut register = 0;
        for key in handler::get_pressed_keys().iter() {
            match key {
                Key::Up => register |= 0x10,
                Key::Down => register |= 0x20,
                Key::Left => register |= 0x40,
                Key::Right => register |= 0x80,
                Key::Z => register |= 0x01,
                Key::X => register |= 0x02,
                Key::LControl => register |= 0x04,
                Key::Space => register |= 0x08,
                _ => ()
            }
        }
        self.register = register;
    }

    pub fn update_register(&mut self, data: u8) {
        self.register |= data;
    }

    pub fn read_bit(&mut self) -> u8 {
        let bit = self.register & 0x01;
        self.register >>= 1;
        bit
    }

    pub fn read_register(&self) -> u8 {
        self.register
    }
}