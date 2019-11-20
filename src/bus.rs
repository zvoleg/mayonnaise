use std::cell::RefCell;
use crate::program::Cartridge;

pub struct Bus {
    cpuRam: [u8; 0x07FF],
    ppuRegister: [u8; 8],
//    cartridge: RefCell<Cartridge>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpuRam: [0; 0x07FF],
            ppuRegister: [0; 8],
//            cartridge: RefCell::default(),
        }
    }

    pub fn read_cpu_ram(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            return self.cpuRam[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            return self.ppuRegister[(address & 0x0007) as usize];
        } else if address >= 0x4020 && address <= 0xFFFF {
            // cartrige space
        }
        } else {
            0
        }
    }

    pub fn write_cpu_ram(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            self.cpuRam[(address & 0x07FF) as usize] = data;
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppuRegister[(address & 0x0007) as usize] = data;
        } else if address >= 0x4020 && address <= 0xFFFF {
            // cartrige space, maybe need access to mapper
        }
    }
}