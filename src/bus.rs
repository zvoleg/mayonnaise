use std::rc::Rc;
use std::cell::RefCell;

use crate::program::Cartridge;
use crate::ppu::Ppu;

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    ppu: Rc<RefCell<Ppu>>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Bus {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Bus {
        Bus {
            cpu_ram: [0; 0x0800],
            ppu: ppu,
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
        println!("cartridge insert");
    }

    pub fn read_cpu_ram(&self, address: u16) -> u8 {
        let mut data = 0;
        if address <= 0x1FFF {
            data = self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            data = self.ppu.borrow_mut().
                cpu_read(address & 0x0007);
        } else if address >= 0x4020 {
            self.cartridge.as_ref().unwrap().borrow_mut().
                read_prg_rom(address, &mut data);
        }
        data
    }

    pub fn write_cpu_ram(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            self.cpu_ram[(address & 0x07FF) as usize] = data;
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu.borrow_mut().
                cpu_write(address & 0x0007, data);
        } else if address >= 0x4020 {
            // cartrige space, maybe need access to mapper
        }
    }
}
