use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use crate::program::Cartridge;

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    ppu_registers: [u8; 8],
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_ram: [0; 0x0800],
            ppu_registers: [0; 8],
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
        println!("cartridge insert");
    }

    pub fn read_cpu_ram(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            return self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            return self.ppu_registers[(address & 0x0007) as usize];
        } else if address >= 0x4020 {
           return self.cartridge.as_ref().unwrap().deref().borrow_mut().read_prg_rom(address);
        } else {
            0
        }
    }

    pub fn write_cpu_ram(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            self.cpu_ram[(address & 0x07FF) as usize] = data;
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu_registers[(address & 0x0007) as usize] = data;
        } else if address >= 0x4020 {
            // cartrige space, maybe need access to mapper
        }
    }
}
