extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;
use crate::bus::Bus;

struct Register {
    address: u16,
    data: u8,
}

impl Register {
    fn new(address: u16) -> Register {
        Register { address, data: 0x00 }
    }
}

pub struct Ppu {
    pallette_colors: [u32; 0x40],
    patterns: [[u8; 0x4000]; 2], // not necessary for emulation

    // 0x2000 - 0x2FFF // (30 line by 32 sprites (960 bytes or 0x03C0) and 2 line with collor) * 4 name-table
    // 2 name-table stores on device and 2 can stores on cartridge
    // each name table take 1kb (0x0400) of memory
    name_table: [u8; 0x0800],
    pallette: [u8; 0x0020], // 0x3F00 - 0x3F1F

    // spraits memory not include in address space of ppu (256 bytes or 0x0100)
    // it can contains 64 sprites by 4 byte by each sprite
    // bytes assignment:
    // 1. y coordinate of sprite (top-left corner)
    // 2. sprite address in patterns array
    // 3. sprite attributes^
    //    7 - vertical mirroring of sprite (1 - mirror, 0 - normal)
    //    6 - horizontal mirroring of sprite (1 - mirror, 0 - normal)
    //    5 - priority of sprite (1 - over the background, 0 - under the background)
    //    4, 3, 2 - unused
    //    1, 0 - higher bits of color
    // 4. x coordinate of sprite (top-left corner)

    bus: Rc<RefCell<Bus>>,
    pub skanline: i16,
    pub cycle: u16,
    frame_complete: bool,
    nmi_require: bool,
    // Registers
    controller:  Register, // 0x2000
    mask:        Register, // 0x2001
    status:      Register, // 0x2002
    oam_address: Register, // 0x2003
    oam_data:    Register, // 0x2004
    scroll:      Register, // 0x2005
    address:     Register, // 0x2006
    data:        Register, // 0x2007
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
            name_table: [0; 0x0800],
            pallette: [0; 0x0020],
            bus,
            skanline:    -1,
            cycle:       0,
            frame_complete: false,
            nmi_require: false,
            controller:  Register::new(0x2000),
            mask:        Register::new(0x2001),
            status:      Register::new(0x2002),
            oam_address: Register::new(0x2003),
            oam_data:    Register::new(0x2004),
            scroll:      Register::new(0x2005),
            address:     Register::new(0x2006),
            data:        Register::new(0x2007),
        }
    }

    fn read_cpu(&self, register: &mut Register) {
        let address = register.address;
        register.data = self.bus.as_ref().borrow().read_cpu_ram(address);
        match address {
            0x2000 => (),
            0x2001 => (),
            0x2002 => (),
            0x2003 => (),
            0x2004 => (),
            0x2005 => (),
            0x2006 => (),
            0x2007 => (),
            _ => panic!("wrong addres when ppu try read registers from cpu ram"),
        };
    }

    fn write_cpu(&mut self, address: u16, data: u8) {
        match address {
            0x2000 => (),
            0x2001 => (),
            0x2002 => {
                self.status.data = data;
                self.bus.as_ref().borrow_mut().write_cpu_ram(self.status.address, data);
            },
            0x2003 => (),
            0x2004 => (),
            0x2005 => (),
            0x2006 => (),
            0x2007 => (),
            _ => panic!("wrong addres when ppu try wryte registers to cpu ram"),
        };
    }

    pub fn read_ppu(&self, address: u16) -> u8 {
        0
    }

    pub fn write_ppu(&mut self, address: u16, data: u8) {

    }

    fn read_from_cartridge(&self, address: u16) -> u8 {
        self.bus.as_ref().borrow().read_chr_from_cartridge(address)
    }

    pub fn get_pattern_table(&self, table: u8) -> [u8; 0x4000] {
        self.patterns[table as usize]
    }

    pub fn reset_frame_complete_status(&mut self) {
        self.frame_complete = false;
    }

    pub fn frame_complete(&self) -> bool {
        self.frame_complete
    }

    pub fn reset_nmi(&mut self) {
        self.nmi_require = false;
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

    pub fn clock(&mut self) -> Option<u32> {
        if self.skanline == 241 && self.cycle == 1 {
            self.write_cpu(self.status.address, self.status.data | 0x80);
        }
        if self.skanline == -1 && self.cycle == 1 {
            self.status.data &= !0x80;
            self.write_cpu(self.status.address, self.status.data & !0x80);
        }
        let color = if (self.cycle == 0 && self.skanline < 240) || (self.cycle < 256 && self.skanline == 239) {
            Some(0)
        } else if (self.cycle > 0 && self.cycle < 256) && self.skanline < 240 {
            if rand::random() {
                Some(0xFFFFFF)
            } else {
                Some(0)
            }
        } else {
            None
        };
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.skanline += 1;
            if self.skanline >= 262 {
                self.skanline = -1;
                self.frame_complete = true;
            }
        }
        color
    }
}
