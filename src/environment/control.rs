extern crate spriter;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

use spriter::Key;

pub struct Controller {
    register: u8,
    pressed_keys: Rc<RefCell<HashSet<Key>>>,
}

impl Controller {
    pub fn new(pressed_keys: Rc<RefCell<HashSet<Key>>>) -> Controller {
        Controller {
            register: 0,
            pressed_keys,
        }
    }

    pub fn update_register_by_input(&mut self) {
        let mut register = 0;
        for key in self.pressed_keys.borrow().iter() {
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