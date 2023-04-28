use std::rc::Rc;
use std::cell::RefCell;

use crate::program::Cartridge;
use crate::ppu::Ppu;
use crate::environment::control::Controller;

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    ppu: Rc<RefCell<Ppu>>,
    controller_a: Controller,
    cartridge: Option<Rc<RefCell<Cartridge>>>,

    previous_data: u8,

    dma_enable: bool,
    dma_wait_clock: bool,
    oam_page: u16,
    oam_addr: u8,
    oam_data: u8,
}

impl Bus {
    pub fn new(controller_a: Controller, ppu: Rc<RefCell<Ppu>>) -> Bus {
        Bus {
            cpu_ram: [0; 0x0800],
            ppu,
            controller_a,
            cartridge: None,

            previous_data: 0,

            dma_enable: false,
            dma_wait_clock: true,
            oam_page: 0,
            oam_addr: 0,
            oam_data: 0,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
        info!("cartridge insert");
    }

    pub fn read_only_data(&self, address: u16) -> u8 {
        let mut data = 0;
        if address <= 0x1FFF {
            data = self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            data = self.ppu.borrow().
                cpu_read_only(address & 0x0007);
        } else if address == 0x4016 {
            data = self.controller_a.read_register();
        } else if address == 0x4017 {
            data = 0;
        } else if address >= 0x4020 {
            self.cartridge.as_ref().unwrap().borrow_mut().
                read_prg_rom(address, &mut data);
        }
        data
    }

    pub fn read_cpu_ram(&mut self, address: u16) -> u8 {
        let mut data = self.previous_data;
        if address <= 0x1FFF {
            data = self.cpu_ram[(address & 0x07FF) as usize];
        } else if address >= 0x2000 && address <= 0x3FFF {
            data = self.ppu.borrow_mut().
                cpu_read(address & 0x0007);
        } else if address == 0x4016 {
            data = self.controller_a.read_bit();
        } else if address == 0x4017 {
            data = 0;
        } else if address >= 0x4020 {
            self.cartridge.as_ref().unwrap().borrow_mut().
                read_prg_rom(address, &mut data);
        }
        self.previous_data = data;
        data
    }

    pub fn write_cpu_ram(&mut self, address: u16, data: u8) {
        self.previous_data = data;
        if address <= 0x1FFF {
            self.cpu_ram[(address & 0x07FF) as usize] = data;
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu.borrow_mut().
                cpu_write(address & 0x0007, data);
        } else if address == 0x4014 {
            self.dma_enable = true;
            self.dma_wait_clock = true;
            self.oam_page = (data as u16) << 8;
        } else if address == 0x4016 {
            self.controller_a.update_register_by_input();
        } else if address >= 0x4020 {
            info!("cpu: try write to cartridge {:04X} -> {:02X}", address, data);
            self.cartridge.as_ref().unwrap().borrow_mut().
                write_prg_rom(address, data);
        }
    }

    pub fn write_input_value(&mut self, input_value: u8) {
        self.controller_a.update_register(input_value);
    }

    pub fn dma_enable(&self) -> bool {
        self.dma_enable
    }

    pub fn disable_dma(&mut self) {
        self.dma_enable = false;
    }

    pub fn dma_wait_clock(&self) -> bool {
        self.dma_wait_clock
    }

    pub fn set_dma_wait_clock(&mut self, wait: bool) {
        self.dma_wait_clock = wait;
    }

    pub fn read_dma_byte(&mut self) {
        let address = self.oam_page | self.oam_addr as u16;
        self.oam_data = self.read_cpu_ram(address);
    }

    pub fn write_dma_byte(&mut self) {
        self.ppu.borrow_mut().write_oam_byte(self.oam_addr, self.oam_data);
        self.oam_addr = self.oam_addr.wrapping_add(1);
        if self.oam_addr == 0x00 {
            self.dma_enable = false;
        }
    }
}
