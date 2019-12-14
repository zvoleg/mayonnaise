extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;
use crate::bus::Bus;

pub struct Ppu {
    pallette_colors: [u32; 0x40],
    patterns: [[u8; 0x4000]; 2],
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

impl<'a> Ppu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Ppu {
        let pallette_colors = [
            0x545454, 0x001E74, 0x081090, 0x300088, 0x440064, 0x5C0030, 0x540400, 0x3C1800, 0x202A00, 0x083A00, 0x004000, 0x003C00, 0x00323C, 0x000000, 0x000000, 0x000000, 
            0x989698, 0x084CC4, 0x3032EC, 0x5C1EE4, 0x8814B0, 0xA01464, 0x982220, 0x783C00, 0x545A00, 0x287200, 0x087C00, 0x007628, 0x006678, 0x000000, 0x000000, 0x000000,
            0xECEEEC, 0x4C9AEC, 0x787CEC, 0xB062EC, 0xE454EC, 0xEC58B4, 0xEC6A64, 0xD48820, 0xA0AA00, 0x74C400, 0x4CD020, 0x38CC6C, 0x38B4CC, 0x3C3C3C, 0x000000, 0x000000,
            0xECEEEC, 0xA8CCEC, 0xBCBCEC, 0xD4B2EC, 0xECAEEC, 0xECAED4, 0xECB4B0, 0xE4C490, 0xCCD278, 0xB4DE78, 0xA8E290, 0x98E2B4, 0xA0D6E4, 0xA0A2A0, 0x000000, 0x000000
        ];
        Ppu {
            pallette_colors,
            patterns: [[0; 0x4000]; 2],
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

    pub fn read_all_sprites(&mut self, table: u8) {
        let start = 0x1000 * table as u16;
        for y in 0..16 {
            for x in 0..16 {
                let offset = y * 256 + x * 16;
                for row in 0..8 {
                    let mut low = self.read_from_cartridge(start + (offset + row) as u16);
                    let mut high = self.read_from_cartridge(start + (offset + row) as u16 + 8);
                    for column in 0..8 {
                        let pixel = ((high & 0x01) << 1) | (low & 0x01);
                        low >>= 1;
                        high >>= 1;
                        self.patterns[table as usize][y * 1024 + row * 128 + x * 8 + 7 - column] = pixel;
                    }
                }
            }
        }
    }

    pub fn get_pattern_table(&self, table: u8) -> [u8; 0x4000] {
        self.patterns[table as usize]
    }

    pub fn nmi_require(&self) -> bool {
        self.nmi_require
    }

    pub fn reset_nmi(&mut self) {
        self.nmi_require = false;
    }

    fn read_from_cartridge(&self, address: u16) -> u8 {
        self.bus.as_ref().borrow().read_ppu(address)
    }
}
