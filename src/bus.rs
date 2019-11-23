use std::cell::RefCell;
use crate::program::Cartridge;

pub struct Bus {
    cpu_ram: [u8; 0x07FF],
    ppu_registers: [u8; 8],
    cartridge: Option<RefCell<Cartridge>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_ram: [0; 0x07FF],
            ppu_registers: [0; 8],
            cartridge: None,
        }
    }

    pub fn read_cpu_ram(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            return self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            return self.ppu_registers[(address & 0x0007) as usize];
        } else if address >= 0x4020 && address <= 0xFFFF {
            // cartrige space
            0
        } else {
            0
        }
    }

    pub fn write_cpu_ram(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            self.cpu_ram[(address & 0x07FF) as usize] = data;
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu_registers[(address & 0x0007) as usize] = data;
        } else if address >= 0x4020 && address <= 0xFFFF {
            // cartrige space, maybe need access to mapper
        }
    }
}
