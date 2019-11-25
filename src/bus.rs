use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::Deref;

use crate::program::Cartridge;
use crate::emu6502::Emu6502;

pub struct Bus {
    cpu_ram: [u8; 0x07FF],
    ppu_registers: [u8; 8],
    cpu:       Option<Rc<RefCell<Emu6502>>>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_ram: [0; 0x07FF],
            ppu_registers: [0; 8],
            cpu: None,
            cartridge: None,
        }
    }

    pub fn set_cpu(&mut self, cpu: Rc<RefCell<Emu6502>>) {
        // cpu.deref().borrow_mut().irq();
        // r_cpu.irq(); //already borrowed: BorrowMutError
        self.cpu = Some(cpu);
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn read_cpu_ram(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            return self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            return self.ppu_registers[(address & 0x0007) as usize];
        } else if address >= 0x4020 {
//            return self.cartridge.unwrap().read_prg_rom(address);
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
        } else if address >= 0x4020 {
            // cartrige space, maybe need access to mapper
        }
    }

    pub fn clock(&self) {
       self.cpu.as_ref().unwrap().deref().borrow_mut().clock();
    }
}
