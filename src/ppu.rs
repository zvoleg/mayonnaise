extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;
use crate::program::Cartridge;

enum Mirroring {
    HORISONTAL,
    VERTICAL,
    UNDEFINED,
}

struct Control {
    data: u8,
}

impl Control {
    fn new(data: u8) -> Control {
        Control { data }
    }

    fn set(&mut self, data: u8) {
        self.data = data;
    }

    fn nmi_flag(&self) -> bool {
        self.data & 0x80 != 0
    }

    fn get_increment(&self) -> u16 {
        match self.data & 0x04 != 0 {
            true => 32,
            false => 1
        }
    }

    fn background_table_address(&self) -> u16 {
        match self.data & 0x10 != 0 {
            true => 0x1000,
            false => 0x0000
        }
    }

    fn base_name_table_address(&self) -> u16 {
        match self.data & 0x03 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("undefined base address for name table"),
        }
    }
}

struct Status {
    data: u8,
}

impl Status {
    fn new(data: u8) -> Status {
        Status { data }
    }

    fn in_vblank(&self) -> bool {
        self.data & 0x80 != 0
    }

    fn set_vblank(&mut self, set: bool) {
        match set {
            true => self.data |= 0x80,
            false => self.data &= !0x80
        }
    }
}

#[derive(Copy, Clone)]
struct Loopy {
    data: u16,
}

impl Loopy {
    fn new() -> Loopy {
        Loopy {
            data: 0,
        }
    }

    fn set_name_table(&mut self, name_table: u8) {
        let name_table = (name_table & 0x03) as u16;
        self.data &= 0x73FF; // clear bits 11 and 12
        self.data |= name_table << 10;
    }

    fn set_coarse_x(&mut self, full_x: u8) {
        let coarse_x = full_x >> 3;
        self.data &= 0x7FE0; // clear bits 1, 2, 3, 4, 5
        self.data |= coarse_x as u16;
    }

    fn set_coarse_y(&mut self, full_y: u8) {
        let coarse_y = (full_y >> 3) as u16;
        self.data &= 0x7C1F; // clear bits 6, 7, 8, 9, 10
        self.data |= coarse_y << 5;
    }

    fn set_fine_y(&mut self, full_y: u8) {
        let fine_y = (full_y & 0x07) as u16;
        self.data &= 0x0FFF; // clear bits 13, 14, 15
        self.data |= fine_y << 12;
    }

    fn get_name_table_address(&self) -> usize {
        (self.data & 0x0FFF) as usize
    }

    fn set_high_address(&mut self, data: u8) {
        let high_address = (data & 0x3f) as u16;
        self.data &= 0x00FF; // clear high 8 bits
        self.data |= high_address << 8;
    }

    fn set_low_address(&mut self, data: u8) {
        let low_address = data as u16;
        self.data &= 0xFF00; //clear low 8 bits
        self.data |= low_address;
    }

    fn add_increment(&mut self, increment: u16) {
        self.data = self.data.overflowing_add(increment).0;
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

    // OAM:
    // spraits memory not include in address space of ppu (256 bytes or 0x0100)
    // it can contains 64 sprites by 4 byte by each sprite
    // bytes assignment:
    // 1. y coordinate of sprite (top-left corner)
    // 2. sprite address in patterns array
    // 3. sprite attributes:
    //    7 - vertical mirroring of sprite (1 - mirror, 0 - normal)
    //    6 - horizontal mirroring of sprite (1 - mirror, 0 - normal)
    //    5 - priority of sprite (1 - over the background, 0 - under the background)
    //    4, 3, 2 - unused
    //    1, 0 - higher bits of color
    // 4. x coordinate of sprite (top-left corner)

    cartridge: Option<Rc<RefCell<Cartridge>>>,
    mirroring: Mirroring,

    pub skanline: u16,
    pub cycle:    u16,

    frame_complete: bool,
    vblank:         bool,
    nmi_require:    bool,
    // Registers
    control:     Control,  // 0x2000
    mask:        u8, // 0x2001
    status:      Status,   // 0x2002
    oam_address: u8, // 0x2003
    oam_data:    u8, // 0x2004
                     // 0x2005 -> scroll register logic is hiden in loopy register
                     // 0x2006 -> address register logic is hiden in loopy register
                     // 0x2007 -> read/write directly from ppu memory

    cur_vram_addr:     Loopy,
    tmp_vram_addr:     Loopy,
    fine_x_scroll:     u8,
    latch:             bool,
    data_buffer:       u8,
}

impl<'a> Ppu {
    pub fn new() -> Ppu {
        let pallette_colors = [
            0x545454, 0x001E74, 0x081090, 0x300088, 0x440064, 0x5C0030, 0x540400, 0x3C1800, 0x202A00, 0x083A00, 0x004000, 0x003C00, 0x00323C, 0x000000, 0x000000, 0x000000, 
            0x989698, 0x084CC4, 0x3032EC, 0x5C1EE4, 0x8814B0, 0xA01464, 0x982220, 0x783C00, 0x545A00, 0x287200, 0x087C00, 0x007628, 0x006678, 0x000000, 0x000000, 0x000000,
            0xECEEEC, 0x4C9AEC, 0x787CEC, 0xB062EC, 0xE454EC, 0xEC58B4, 0xEC6A64, 0xD48820, 0xA0AA00, 0x74C400, 0x4CD020, 0x38CC6C, 0x38B4CC, 0x3C3C3C, 0x000000, 0x000000,
            0xECEEEC, 0xA8CCEC, 0xBCBCEC, 0xD4B2EC, 0xECAEEC, 0xECAED4, 0xECB4B0, 0xE4C490, 0xCCD278, 0xB4DE78, 0xA8E290, 0x98E2B4, 0xA0D6E4, 0xA0A2A0, 0x000000, 0x000000
        ];
        Ppu {
            pallette_colors,
            patterns:   [[0; 0x4000]; 2],
            name_table: [0; 0x0800],
            pallette:   [0; 0x0020],

            cartridge: None,
            mirroring: Mirroring::UNDEFINED,

            skanline:          0,
            cycle:             0,

            frame_complete:    false,
            vblank:            false,
            nmi_require:       false,

            control:           Control::new(0),
            mask:              0,
            status:            Status::new(0),
            oam_address:       0,
            oam_data:          0,

            cur_vram_addr:     Loopy::new(),
            tmp_vram_addr:     Loopy::new(),
            fine_x_scroll:     0,
            latch:             false,
            data_buffer:       0,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.mirroring = match cartridge.borrow().get_mirroring() {
            0 => Mirroring::HORISONTAL,
            1 => Mirroring::VERTICAL,
            _ => Mirroring::UNDEFINED,
        };
        self.cartridge = Some(cartridge);
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut data = 0;
        match address {
            0x0000 => (),
            0x0001 => (),
            0x0002 => {
                data = self.status.data;
                self.status.set_vblank(false);
                self.latch = false;
            },
            0x0003 => (),
            0x0004 => (),
            0x0005 => (),
            0x0006 => (),
            0x0007 => {
                data = self.data_buffer;
                self.data_buffer = self.read_ppu(self.cur_vram_addr.data);
                if self.cur_vram_addr.data >= 0x3F00 {
                    data = self.data_buffer;
                }
                let increment = self.control.get_increment();
                self.cur_vram_addr.add_increment(increment);
            },
            _ => panic!("wrong addres when cpu try read ppu registers"),
        };
        data
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000 => {
                let old_nmi_status = self.control.nmi_flag();
                self.control.set(data);
                self.tmp_vram_addr.set_name_table(data);
                if self.vblank &&
                    self.status.in_vblank() &&
                    !old_nmi_status &&
                    self.control.nmi_flag() {
                    self.nmi_require = true;
                }
            },
            0x0001 => {
                self.mask = data;
            },
            0x0002 => (),
            0x0003 => {
                self.oam_address = data;
            },
            0x0004 => (),
            0x0005 => {
                if !self.latch {
                    self.tmp_vram_addr.set_coarse_x(data);
                    self.fine_x_scroll = data & 0x07;
                    self.latch = !self.latch;
                } else {
                    self.tmp_vram_addr.set_coarse_y(data);
                    self.tmp_vram_addr.set_fine_y(data);
                    self.latch = !self.latch;
                }
            },
            0x0006 => {
                if !self.latch {
                    self.tmp_vram_addr.set_high_address(data);
                    self.latch = !self.latch;
                } else {
                    self.tmp_vram_addr.set_low_address(data);
                    self.cur_vram_addr = self.tmp_vram_addr;
                    self.latch = !self.latch;
                }
            },
            0x0007 => {
                self.write_ppu(self.cur_vram_addr.data, data);
                let increment = self.control.get_increment();
                self.cur_vram_addr.add_increment(increment);
            },
            _ => panic!("wrong addres when cpu try wryte ppu registers"),
        };
    }

    pub fn read_ppu(&self, address: u16) -> u8 {
        let mut data = 0;
        if address < 0x2000 {
            data = self.read_from_cartridge(address);
        } else if address >= 0x2000 && address < 0x3F00 {
            let address = (address & 0x0FFF) as usize;
            match self.mirroring {
                Mirroring::HORISONTAL => {
                    if address < 0x400 {
                        data = self.name_table[address];
                    } else if address >= 0x400 && address < 0x800 {
                        data = self.name_table[address & 0x3FF];
                    } else if address >= 0x800 && address < 0xC00 {
                        data = self.name_table[address & 0x7FF];
                    } else if address >= 0xC00 && address < 0x1000 {
                        data = self.name_table[address & 0x7FF];
                    }
                },
                Mirroring::VERTICAL => {
                    if address < 0x400 {
                        data = self.name_table[address];
                    } else if address >= 0x400 && address < 0x800 {
                        data = self.name_table[address];
                    } else if address >= 0x800 && address < 0xC00 {
                        data = self.name_table[address & 0x3FF];
                    } else if address >= 0xC00 && address < 0x1000 {
                        data = self.name_table[address & 0x7FF];
                    }
                },
                _ => (),
            }
        } else if address >= 0x3F00 && address < 0x3FFF {
            let address = address & 0x001F;
            data = self.pallette[address as usize];
        }
        data
    }

    pub fn write_ppu(&mut self, address: u16, data: u8) {
        // let mut data = 0;
        if address < 0x2000 {
            // data = self.bus.as_ref().borrow().read_chr_from_cartridge(address);
        } else if address >= 0x2000 && address < 0x3F00 {
            let address = (address & 0x0FFF) as usize;
            match self.mirroring {
                Mirroring::HORISONTAL => {
                    if address < 0x400 {
                        self.name_table[address] = data;
                    } else if address >= 0x400 && address < 0x800 {
                        self.name_table[address & 0x3FF] = data;
                    } else if address >= 0x800 && address < 0xC00 {
                        self.name_table[address & 0x7FF] = data;
                    } else if address >= 0xC00 && address < 0x1000 {
                        self.name_table[address & 0x7FF] = data;
                    }
                },
                Mirroring::VERTICAL => {
                    if address < 0x400 {
                        self.name_table[address] = data;
                    } else if address >= 0x400 && address < 0x800 {
                        self.name_table[address] = data;
                    } else if address >= 0x800 && address < 0xC00 {
                        self.name_table[address & 0x3FF] = data;
                    } else if address >= 0xC00 && address < 0x1000 {
                        self.name_table[address & 0x7FF] = data;
                    }
                },
                _ => (),
            }
        } else if address >= 0x3F00 && address < 0x3FFF {
            let address = address & 0x001F;
            self.pallette[address as usize] = data;
        }
    }

    fn read_from_cartridge(&self, address: u16) -> u8 {
        let mut data = 0;
        self.cartridge.as_ref().unwrap().borrow().read_chr_rom(address, &mut data);
        data
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

    pub fn nmi_require(&self) -> bool {
        self.nmi_require
    }

    pub fn reset_nmi_require(&mut self) {
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

    pub fn read_name_table(&self, name_table: u16) {
        let base_addr = 0x0400 * name_table;
        for row in 0 .. 32 {
            for column in 0 .. 32 {
                let offset = row * 32 + column;
                let value = self.name_table[(base_addr + offset) as usize];
                print!("{:02X} ", value);
            }
            println!("");
        }
    }

    pub fn clock(&mut self) -> Option<u32> {
        if self.skanline == 241 && self.cycle == 1 {
            self.status.set_vblank(true);
            self.vblank = true;
            if self.control.nmi_flag() {
                self.nmi_require = true;
            }
        }
        if self.skanline == 261 && self.cycle == 1 {
            self.status.set_vblank(false);
            self.vblank = false;
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
            if self.skanline > 261 {
                self.skanline = 0;
                self.frame_complete = true;
            }
        }
        color
    }
}
