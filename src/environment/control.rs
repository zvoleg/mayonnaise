use spriter::Key;

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
        if_holded!(Key::Up, { register |= 0x10 });
        if_holded!(Key::Down, { register |= 0x20 });
        if_holded!(Key::Left, { register |= 0x40 });
        if_holded!(Key::Right, { register |= 0x80 });
        if_holded!(Key::Z, { register |= 0x01 });
        if_holded!(Key::X, { register |= 0x02 });
        if_holded!(Key::LControl, { register |= 0x04 });
        if_holded!(Key::Space, { register |= 0x08 });
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