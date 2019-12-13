extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;
use crate::bus::Bus;

pub struct Ppu {
    sprites: [u8; 0x2000], // 0x0000 - 0x1FFF <- it stored on cartridge, access through mapper
    name_table: [u8; 0x1000], // 0x2000 - 0x2FFF
    pallette: [u8; 0x0020], // 0x3F00 - 0x3F1F

    bus: Rc<RefCell<Bus>>,
    pub skanline: u16,
    pub cycle: u16,
    nmi_require: bool,
    // Registers
    controller:  u8, // 0x2000
    mask:        u8, // 0x2001
    status:      u8, // 0x2002
    oam_address: u8, // 0x2003
    oam_data:    u8, // 0x2004
    scroll:      u8, // 0x2005
    address:     u8, // 0x2006
    data:        u8, // 0x2007
}

impl Ppu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Ppu {
        Ppu {
            sprites: [0; 0x2000],
            name_table: [0; 0x1000],
            pallette: [0; 0x0020],
            bus,
            skanline:    0,
            cycle:       0,
            nmi_require: false,
            controller:  0,
            mask:        0,
            status:      0,
            oam_address: 0,
            oam_data:    0,
            scroll:      0,
            address:     0,
            data:        0,
        }
    }

    fn update_registers(&mut self) {
        self.controller = self.bus.borrow().read_cpu_ram(0x2000);
        self.mask = self.bus.borrow().read_cpu_ram(0x2001);
        self.status = self.bus.borrow().read_cpu_ram(0x2002);
        self.oam_address = self.bus.borrow().read_cpu_ram(0x2003);
        self.oam_data = self.bus.borrow().read_cpu_ram(0x2004);
        self.scroll = self.bus.borrow().read_cpu_ram(0x2005);
        self.address = self.bus.borrow().read_cpu_ram(0x2006);
        self.data = self.bus.borrow().read_cpu_ram(0x2007);
    }

    pub fn clock(&mut self) -> Option<u32> {
        self.cycle += 1;
        if self.cycle == 257 {
            self.skanline += 1;
            self.cycle = 0;
        }
        if self.skanline == 241 && self.cycle == 1 {
            self.nmi_require = true;
        }
        if self.skanline == 242 {
            self.skanline = 0;
        }
        if rand::random() {
            Some(0xFFFFFF)
        } else {
            Some(0)
        }
    }

    pub fn nmi_require(&self) -> bool {
        self.nmi_require
    }

    pub fn reset_nmi(&mut self) {
        self.nmi_require = false;
    }
}
