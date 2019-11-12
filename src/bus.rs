pub struct Bus {
    memory: [u8; 65536],
}

impl Bus {
    pub fn new() -> Bus {
        Bus { memory: [0; 65536] }
    }

    pub fn read_data(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_data(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}