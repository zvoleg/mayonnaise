pub struct Controller {
    latch: bool,
    register: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            latch: false,
            register: 0,
        }
    }

    pub fn set_latch(&mut self, latch: bool) {
        match latch {
            true  => {
                self.register = 0;
                self.latch = latch;
            },
            false => self.latch = latch,
        }
    }

    pub fn input_access(&self) -> bool {
        self.latch
    }

    pub fn update_register(&mut self, data: u8) {
        if self.latch {
            self.register |= data;
        }
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