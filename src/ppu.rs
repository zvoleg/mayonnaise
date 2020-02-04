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

    fn sprite_size(&self) -> u8 {
        match self.data & 0x20 != 0 {
            true  => 16,
            false => 8
        }
    }

    fn background_table_address(&self) -> u16 {
        match self.data & 0x10 != 0 {
            true  => 0x1000,
            false => 0x0000
        }
    }

    fn sprite_table_address(&self) -> u16 {
        match (self.data & 0x08 != 0) && self.sprite_size() != 16 {
            true  => 0x1000,
            false => 0x0000
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
            true  => self.data |= 0x80,
            false => self.data &= !0x80
        }
    }

    fn set_sprite_overlow(&mut self, set: bool) {
        match set {
            true  => self.data |= 0x20,
            false => self.data &= !0x20
        }
    }

    fn hit_zero_sprite(&self) -> bool {
        self.data & 0x40 != 0
    }

    fn set_hit_zero_sprite(&mut self, set: bool) {
        match set {
            true  => self.data |= 0x40,
            false => self.data &= !0x40
        }
    }
}

struct Mask {
    data: u8
}

impl Mask {
    fn new(data: u8) -> Mask {
        Mask { data }
    }

    fn background_enable(&self) -> bool {
        self.data & 0x08 != 0
    }

    fn sprites_enable(&self) -> bool {
        self.data & 0x10 != 0
    }

    fn grayscale_mode(&self) -> bool {
        self.data & 0x01 != 0
    }

    fn bg_enable_left_column(&self) -> bool {
        self.data & 0x02 != 0
    }

    fn sprite_enable_left_column(&self) -> bool {
        self.data & 0x04 != 0
    }
}

#[derive(Copy, Clone)]
struct AddresRegister {
    data: u16,
}

impl AddresRegister {
    fn new() -> AddresRegister {
        AddresRegister {
            data: 0,
        }
    }

    fn set_name_table(&mut self, name_table: u8) {
        let name_table = (name_table & 0x03) as u16;
        self.data &= 0x73FF; // clear bits 11 and 12
        self.data |= name_table << 10;
    }

    fn set_coarse_x(&mut self, coarse_x: u8) {
        let coarse_x = (coarse_x & 0x1F) as u16;
        self.data &= 0x7FE0; // clear bits 1, 2, 3, 4, 5
        self.data |= coarse_x;
    }

    fn get_coarse_x(&self) -> u8 {
        (self.data & 0x001F) as u8
    }

    fn increment_coarse_x(&mut self) {
        let coarse_x = self.get_coarse_x();
        if coarse_x == 31 {
            self.set_coarse_x(0);
            self.data ^= 0x0400;
        } else {
            self.set_coarse_x(coarse_x + 1);
        }
    }

    fn set_coarse_y(&mut self, coarse_y: u8) {
        let coarse_y = (coarse_y & 0x1F) as u16;
        self.data &= 0x7C1F; // clear bits 6, 7, 8, 9, 10
        self.data |= coarse_y << 5;
    }

    fn get_coarse_y(&self) -> u8 {
        ((self.data >> 5) & 0x001F) as u8
    }

    fn increment_coarse_y(&mut self) {
        let coarse_y = self.get_coarse_y();
        if coarse_y == 29 {
            self.set_coarse_y(0);
            self.data ^= 0x0800;
        } else if coarse_y == 31 {
            self.set_coarse_y(0);
        } else {
            self.set_coarse_y(coarse_y + 1);
        }
    }

    fn set_fine_y(&mut self, fine_y: u8) {
        let fine_y = (fine_y & 0x07) as u16;
        self.data &= 0x0FFF; // clear bits 13, 14, 15
        self.data |= fine_y << 12;
    }

    fn get_fine_y(&self) -> u8 {
        (self.data >> 12) as u8
    }

    fn increment_fine_y(&mut self) {
        let fine_y = self.get_fine_y();
        if fine_y == 7 {
            self.set_fine_y(0);
            self.increment_coarse_y();
        } else {
            self.set_fine_y(fine_y + 1);
        }
    }

    fn get_tile_address(&self) -> u16 {
        0x2000 | self.data & 0x0FFF
    }

    fn get_attribute_address(&self) -> u16 {
        let x_offset = (self.get_coarse_x() >> 2) as u16;
        let y_offset = (self.get_coarse_y() >> 2) as u16;
        0x23C0 | self.data & 0x0C00 | (y_offset << 3) | x_offset
    }

    fn set_high_address(&mut self, data: u8) {
        let high_address = (data & 0x3F) as u16;
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

#[derive(Clone, Copy)]
struct Oam {
    y_position: u8,
    id:  u8,
    attributes: u8,
    x_position: u8,
}

impl Oam {
    fn new () -> Oam {
        Oam {
            y_position: 0,
            id:  0,
            attributes: 0,
            x_position: 0,
        }
    }

    fn get_pallette_id(&self) -> u8 {
        self.attributes & 0x03
    }

    fn horizontal_flip(&self) -> bool {
        self.attributes & 0x40 != 0
    }

    fn vertical_flip(&self) -> bool {
        self.attributes & 0x80 != 0
    }

    fn in_front_of_bg(&self) -> u8 {
        match self.attributes & 0x20 == 0 {
            true  => 0,
            false => 1,
        }
    }
}

pub struct Ppu {
    pub pallette_colors: [u32; 0x40],
    pub patterns: [[u8; 0x4000]; 2], // not necessary for emulation

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
    //    1, 0 - bits of color
    // 4. x coordinate of sprite (top-left corner)
    oam_memory: [Oam; 64],
    oam_tmp:    [Oam; 8],
    oam_buffer: [Oam; 8],

    cartridge: Option<Rc<RefCell<Cartridge>>>,
    mirroring: Mirroring,

    skanline: u16,
    cycle:    u16,

    pub frame_complete: bool,
    vblank:             bool,
    nmi_require:        bool,
    in_visible_range:   bool,
    // Registers
    control:     Control,  // 0x2000
    mask:        Mask,     // 0x2001
    status:      Status,   // 0x2002
    oam_address_reg: u8,   // 0x2003
                           // 0x2004 -> oam data register, write data to oam_data array
                           // 0x2005 -> scroll register logic is hiden in loopy register
                           // 0x2006 -> address register logic is hiden in loopy register
                           // 0x2007 -> read/write directly from ppu memory

    cur_addr:          AddresRegister,
    tmp_addr:          AddresRegister,
    fine_x_scroll:     u8,
    latch:             bool,
    data_buffer:       u8,

    next_background_tile_id:       u8,
    next_background_attribute:     u8,
    next_background_low_pattern:   u8,
    next_background_high_pattern:  u8,

    bg_low_shift_register:            u16,
    bg_high_shift_register:           u16,
    bg_low_attribute_shift_register:  u16,
    bg_high_attribute_shift_register: u16,

    sprite_low_shift_register:            [u8; 8],
    sprite_high_shift_register:           [u8; 8],
    sprite_priority_shift_register:       [u8; 8],
    sprite_attribute_shift_register:      [u8; 8],

    expected_sprite_zero_hit:   bool,

    oam_counter: usize,
    oam_tmp_counter: usize,

    pub update_pallettes: bool,
    pub debug: bool,
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

            oam_memory: [Oam::new(); 64],
            oam_tmp:    [Oam::new(); 8],
            oam_buffer: [Oam::new(); 8],

            cartridge: None,
            mirroring: Mirroring::UNDEFINED,

            skanline:          0,
            cycle:             0,

            frame_complete:    false,
            vblank:            false,
            nmi_require:       false,
            in_visible_range:  false,

            control:           Control::new(0),
            mask:              Mask::new(0),
            status:            Status::new(0),
            oam_address_reg:   0,

            cur_addr:          AddresRegister::new(),
            tmp_addr:          AddresRegister::new(),
            fine_x_scroll:     0,
            latch:             false,
            data_buffer:       0,

            next_background_tile_id:      0,
            next_background_attribute:    0,
            next_background_low_pattern:  0,
            next_background_high_pattern: 0,

            bg_low_shift_register:            0,
            bg_high_shift_register:           0,
            bg_low_attribute_shift_register:  0,
            bg_high_attribute_shift_register: 0,

            sprite_low_shift_register:        [0; 8],
            sprite_high_shift_register:       [0; 8],
            sprite_priority_shift_register:   [0; 8],
            sprite_attribute_shift_register:  [0; 8],

            expected_sprite_zero_hit:   false,

            oam_counter: 0,
            oam_tmp_counter: 0,

            update_pallettes: false,
            debug: false,
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

    pub fn reset(&mut self) {
            self.skanline = 0;
            self.cycle = 0;
            self.frame_complete = false;
            self.vblank = false;
            self.nmi_require = false;
            self.control = Control::new(0);
            self.mask = Mask::new(0);
            self.status = Status::new(0);
            self.oam_address_reg = 0;
            self.cur_addr = AddresRegister::new();
            self.tmp_addr = AddresRegister::new();
            self.fine_x_scroll = 0;
            self.latch = false;
            self.data_buffer = 0;
            self.next_background_tile_id = 0;
            self.next_background_attribute = 0;
            self.next_background_low_pattern = 0;
            self.next_background_high_pattern = 0;
            self.bg_low_shift_register = 0;
            self.bg_high_shift_register = 0;
            self.bg_low_attribute_shift_register = 0;
            self.bg_high_attribute_shift_register = 0;
            self.sprite_low_shift_register = [0; 8];
            self.sprite_high_shift_register = [0; 8];
            self.sprite_attribute_shift_register = [0; 8];
    }

    pub fn cpu_read_only(&self, address: u16) -> u8 {
        let mut data = 0;
        match address {
            0x0000 => data = self.control.data,
            0x0001 => data = self.mask.data,
            0x0002 => data = self.status.data,
            0x0003 => (),
            0x0004 => (),
            0x0005 => (),
            0x0006 => (),
            0x0007 => data = self.data_buffer,
            _ => panic!("wrong addres when cpu try read ppu registers, address: {:04X}", address),
        };
        data
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut data;
        match address {
            // 0x0000 => (),
            // 0x0001 => (),
            0x0002 => {
                data = (self.status.data & 0xE0) | (self.data_buffer & 0x1F);
                self.status.set_vblank(false);
                self.latch = false;
            },
            // 0x0003 => (),
            0x0004 => data = self.read_oam_byte(self.oam_address_reg),
            // 0x0005 => (),
            // 0x0006 => (),
            0x0007 => {
                data = self.data_buffer;
                self.data_buffer = self.read_ppu(self.cur_addr.data);
                if self.cur_addr.data >= 0x3F00 {
                    data = self.data_buffer;
                }
                let increment = self.control.get_increment();
                self.cur_addr.add_increment(increment);
            },
            _ => data = 0, //panic!("wrong addres when cpu try read ppu registers, address: {:04X}", address),
        };
        data
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000 => {
                let old_nmi_status = self.control.nmi_flag();
                self.control.set(data);
                self.tmp_addr.set_name_table(data & 0x03);
                if self.vblank &&
                    self.status.in_vblank() &&
                    !old_nmi_status &&
                    self.control.nmi_flag() {
                    self.nmi_require = true;
                }
                if self.debug {
                    println!("ppu: update CONTROL register: {:02X} | nmi: {} | sprite size: {} | background pattern tabele: {} | sprite pattern table: {} | vram increment: {} | base nametable address: {}",
                        self.control.data,
                        (self.control.data >> 7) & 0x01,
                        (self.control.data >> 5) & 0x01,
                        (self.control.data >> 4) & 0x01,
                        (self.control.data >> 3) & 0x01,
                        (self.control.data >> 2) & 0x01,
                        self.control.data & 0x03
                    );
                }
            },
            0x0001 => {
                self.mask.data = data;
                if self.debug{
                    println!("ppu: update MASK register: {:02X} ({:08b})", self.mask.data, self.mask.data);
                }
            },
            0x0002 => (),
            0x0003 => self.oam_address_reg = data,
            0x0004 => {
               self.write_oam_byte(self.oam_address_reg, data);
               self.oam_address_reg = self.oam_address_reg.overflowing_add(1).0;
            },
            0x0005 => {
                if !self.latch {
                    self.tmp_addr.set_coarse_x(data >> 3);
                    self.fine_x_scroll = data & 0x07;
                    self.latch = !self.latch;
                } else {
                    self.tmp_addr.set_coarse_y(data >> 3);
                    self.tmp_addr.set_fine_y(data & 0x07);
                    self.latch = !self.latch;
                }
                if self.debug{
                    println!("ppu: (scroll) update tmp addr register: {:02X} ({:08b})", self.tmp_addr.data, self.tmp_addr.data);
                }
            },
            0x0006 => {
                if !self.latch {
                    self.tmp_addr.set_high_address(data);
                    self.latch = !self.latch;
                } else {
                    self.tmp_addr.set_low_address(data);
                    self.cur_addr = self.tmp_addr;
                    self.latch = !self.latch;
                }
                if self.debug{
                    println!("ppu: (scroll) update tmp addr register: {:02X} ({:08b})", self.tmp_addr.data, self.tmp_addr.data);
                    println!("ppu: (scroll) update cur addr register: {:02X} ({:08b})", self.cur_addr.data, self.cur_addr.data);
                }
            },
            0x0007 => {
                self.write_ppu(self.cur_addr.data, data);
                let increment = self.control.get_increment();
                self.cur_addr.add_increment(increment);
            },
            _ => panic!("wrong addres when cpu try wryte ppu registers, address: {:04X}", address),
        };
    }

    pub fn read_ppu(&self, address: u16) -> u8 {
        let mut data = 0;
        let address = address & 0x3FFF;
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
            let mut address = (address & 0x001F) as usize;
            match address {
                0x0004 | 0x0008 | 0x000C => address = 0x0000,
                0x0010 => address = 0x0000,
                0x0014 => address = 0x0004,
                0x0018 => address = 0x0008,
                0x001C => address = 0x000C,
                _ => (),
            }
            data = match self.mask.grayscale_mode() {
                true  => self.pallette[address] & 0x30,
                false => self.pallette[address]
            }
        }
        data
    }

    pub fn write_ppu(&mut self, address: u16, data: u8) {
        let address = address & 0x3FFF;
        if address < 0x2000 {
            println!("try to write into pattern tabel by address: {:04X} data: {:02X}", address, data);
        } else if address >= 0x2000 && address < 0x3F00 {
            let address = (address & 0x0FFF) as usize;
            match self.mirroring {
                Mirroring::HORISONTAL => {
                    if address < 0x400 {
                        self.name_table[address] = data;
                    } else if address >= 0x400 && address < 0x0800 {
                        self.name_table[address & 0x3FF] = data;
                    } else if address >= 0x800 && address < 0x0C00 {
                        self.name_table[address & 0x7FF] = data;
                    } else if address >= 0xC00 && address < 0x1000 {
                        self.name_table[address & 0x7FF] = data;
                    }
                },
                Mirroring::VERTICAL => {
                    if address < 0x400 {
                        self.name_table[address] = data;
                    } else if address >= 0x400 && address < 0x0800 {
                        self.name_table[address] = data;
                    } else if address >= 0x800 && address < 0x0C00 {
                        self.name_table[address & 0x3FF] = data;
                    } else if address >= 0xC00 && address < 0x1000 {
                        self.name_table[address & 0x7FF] = data;
                    }
                },
                _ => (),
            }
        } else if address >= 0x3F00 && address < 0x3FFF {
            let address = address & 0x001F;
            match address {
                0x0010 => self.pallette[0x0000] = data,
                0x0014 => self.pallette[0x0004] = data,
                0x0018 => self.pallette[0x0008] = data,
                0x001C => self.pallette[0x000C] = data,
                _ => self.pallette[address as usize] = data,
            }
            self.update_pallettes = true;
        }
    }

    fn read_from_cartridge(&self, address: u16) -> u8 {
        let mut data = 0;
        self.cartridge.as_ref().unwrap().borrow().read_chr_rom(address, &mut data);
        data
    }

    pub fn write_oam_byte(&mut self, address: u8, data: u8) {
        unsafe {
            let first = &mut self.oam_memory[0] as *mut _ as *mut u8;
            *first.offset(address as isize) = data;
        }
    }

    pub fn read_oam_byte(&self, address: u8) -> u8 {
        unsafe {
            let first = &self.oam_memory[0] as *const _ as *const u8;
            *first.offset(address as isize)
        }
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

    fn set_next_data_to_shift_registers(&mut self) {
        self.bg_low_shift_register = (self.bg_low_shift_register & 0x00FF) | (self.next_background_low_pattern as u16) << 8 - self.fine_x_scroll;
        self.bg_high_shift_register = (self.bg_high_shift_register & 0x00FF) | (self.next_background_high_pattern as u16) << 8 - self.fine_x_scroll;

        let attribute_idx_for_tile = (self.cur_addr.get_coarse_y() & 0x02) | ((self.cur_addr.get_coarse_x() >> 1) & 0x01);
        let low_attribute_bit = (self.next_background_attribute >> (attribute_idx_for_tile * 2)) & 0x01;
        self.bg_low_attribute_shift_register = (self.bg_low_attribute_shift_register & 0x00FF) | match low_attribute_bit {
            1 => 0xFF00,
            _ => 0x0000,
        } >> self.fine_x_scroll;
        let high_attribute_bit = (self.next_background_attribute >> (attribute_idx_for_tile * 2) + 1) & 0x01;
        self.bg_high_attribute_shift_register = (self.bg_high_attribute_shift_register & 0x00FF) | match high_attribute_bit {
            1 => 0xFF00,
            _ => 0x0000,
        } >> self.fine_x_scroll;
    }

    fn pop_bg_pixel(&mut self) -> u16 {
        let idx4 = self.bg_high_attribute_shift_register & 0x01;
        let idx3 = self.bg_low_attribute_shift_register & 0x01;
        let idx2 = self.bg_high_shift_register & 0x01;
        let idx1 = self.bg_low_shift_register & 0x01;
        self.bg_high_attribute_shift_register >>= 1;
        self.bg_low_attribute_shift_register >>= 1;
        self.bg_high_shift_register >>= 1;
        self.bg_low_shift_register >>= 1;
        (idx4 << 3) | (idx3  << 2) | (idx2 << 1) | idx1
    }

    fn update_sprite_shift_registers(&mut self) {
        for i in 0..8 {
            let oam = self.oam_buffer[i];
            let y_offset = self.skanline.overflowing_sub(oam.y_position as u16).0;
            if y_offset < self.control.sprite_size() as u16 {
                let sprite_id = oam.id as u16;

                let offset = match oam.vertical_flip() {
                    true  => self.control.sprite_size() as u16 - y_offset,
                    false => y_offset
                };

                let pattern_low_byte_address = self.control.sprite_table_address() + sprite_id * 16 + offset;
                let pattern_high_byte_address = self.control.sprite_table_address() + sprite_id * 16 + 8 + offset;

                let mut low_byte = 0;
                let mut high_byte = 0;

                self.cartridge.as_ref().unwrap().borrow().read_chr_rom(pattern_low_byte_address, &mut low_byte);
                self.cartridge.as_ref().unwrap().borrow().read_chr_rom(pattern_high_byte_address, &mut high_byte);

                // pixel bit reading by 0x01 mask, but first bit to read placed at 0x80 position,
                // that means we should reverse bits for normal rendering of sprite
                if !oam.horizontal_flip() {
                    low_byte = low_byte.reverse_bits();
                    high_byte = high_byte.reverse_bits();
                }
                let attribute = oam.get_pallette_id();
                let priority = oam.in_front_of_bg();

                self.sprite_low_shift_register[i] = low_byte;
                self.sprite_high_shift_register[i] = high_byte;
                self.sprite_attribute_shift_register[i] = attribute;
                self.sprite_priority_shift_register[i] = priority;
            }
        }
    }

    fn pop_sprite_pixel_with_priority(&mut self, bg_pixel: u16) -> u16 {
        let mut bit0;
        let mut bit1;
        let mut bit23;
        let mut bit4;
        let mut pixel = 0;
        for i in 0..8 {
            let oam = &mut self.oam_buffer[i];
            if oam.x_position == 0 {
                bit4 = self.sprite_priority_shift_register[i] as u16;
                bit23 = self.sprite_attribute_shift_register[i] as u16;
                bit1 = (self.sprite_high_shift_register[i] & 0x01) as u16;
                bit0 = (self.sprite_low_shift_register[i] & 0x01) as u16;
                self.sprite_high_shift_register[i] >>= 1;
                self.sprite_low_shift_register[i] >>= 1;
                if pixel == 0 && (bit0 != 0 || bit1 != 0) {
                    pixel = (bit4 << 4) | (bit23 << 2) | (bit1 << 1) | bit0;
                }
                // determine of sprite zero hit
                if i == 0 && self.expected_sprite_zero_hit && pixel & 0x03 > 0 && !self.status.hit_zero_sprite() {
                    if self.mask.bg_enable_left_column() && self.mask.sprite_enable_left_column() {
                        if bg_pixel & 0x03 > 0{
                            self.status.set_hit_zero_sprite(true);
                        }
                    }
                }
            } else {
                oam.x_position -= 1;
            }
        }
        pixel
    }

    fn fetching_data_trough_cycles(&mut self) {
        match self.cycle % 8 {
            1 => {
                self.next_background_tile_id = self.read_ppu(self.cur_addr.get_tile_address());
            },
            3 => {
                self.next_background_attribute = self.read_ppu(self.cur_addr.get_attribute_address());
            },
            5 => {
                let low_byte_address = self.control.background_table_address() + (self.next_background_tile_id as u16 * 16) + self.cur_addr.get_fine_y() as u16;
                self.next_background_low_pattern = self.read_ppu(low_byte_address).reverse_bits();
            }
            7 => {
                let high_byte_address = self.control.background_table_address() + (self.next_background_tile_id as u16 * 16) + 8 + self.cur_addr.get_fine_y() as u16;
                self.next_background_high_pattern = self.read_ppu(high_byte_address).reverse_bits();
            },
            0 => {
                self.set_next_data_to_shift_registers();
                self.cur_addr.increment_coarse_x();
            }
            _ => (),
        }
    }

    pub fn clock(&mut self) -> Option<u32> {
        if self.debug {
            println!(
                "ppu: coarse_x: {:02} | coarse y: {:02} | fine x: {:02} | fine y: {:02} | name_tabel: {:02} | full register: {:015b} ({:04X}) | tmp register: {:015b} ({:04X})",
                self.cur_addr.data & 0x1F,
                (self.cur_addr.data >> 5) & 0x1F,
                self.fine_x_scroll,
                (self.cur_addr.data >> 12) & 0x07,
                (self.cur_addr.data >> 10) & 0x03,
                self.cur_addr.data,
                self.cur_addr.data,
                self.tmp_addr.data,
                self.tmp_addr.data
            );
        }

        let mut color = None;

        if self.cycle >= 1 && self.cycle <= 256 && self.skanline <= 239 {
            color = Some(self.pallette_colors[self.read_ppu(0x3F00) as usize]);
        }

        let mut bg_pixel = 0;
        let mut sprite_pixel = 0;
        let mut sprite_pixel_priority = 0;
        // background pixel
        if self.mask.background_enable() {
            if self.in_visible_range {
                if self.mask.bg_enable_left_column() || self.cycle > 8 {
                    bg_pixel = self.pop_bg_pixel();
                }
                self.fetching_data_trough_cycles();
            }

            if self.cycle >= 321 && self.cycle <= 336 && (self.skanline <= 239 || self.skanline == 261) {
                self.pop_bg_pixel();
                self.fetching_data_trough_cycles();
            }

            if self.skanline == 261 && self.cycle >= 1 && self.cycle <= 256 {
                self.fetching_data_trough_cycles();
            }

            if self.cycle == 256 && (self.skanline <= 239 || self.skanline == 261) {
                self.cur_addr.increment_fine_y();
            }

            if self.cycle == 257 && (self.skanline <= 239 || self.skanline == 261) {
                self.cur_addr.set_coarse_x(self.tmp_addr.get_coarse_x());
                self.cur_addr.data &= !0x0400;
                let x_name_table = (self.tmp_addr.data >> 10) & 0x01;
                self.cur_addr.data |= x_name_table << 10;
            }

            if self.skanline == 261 && self.cycle >= 280 && self.cycle <= 304 {
                self.cur_addr.set_coarse_y(self.tmp_addr.get_coarse_y());
                self.cur_addr.set_fine_y(self.tmp_addr.get_fine_y());
                self.cur_addr.data &= !0x0800;
                let y_name_table = (self.tmp_addr.data >> 11) & 0x01;
                self.cur_addr.data |= y_name_table << 11;
            }

            if self.cycle >= 337 && self.cycle <= 340 && (self.skanline <= 239 || self.skanline == 261) {
                match self.cycle % 8 {
                    1 | 3 => self.next_background_tile_id = self.read_ppu(self.cur_addr.get_tile_address()),
                    _ => (),
                }
            }
        }

        // sprite pixel
        if self.mask.sprites_enable() && self.in_visible_range {
            // sprite evaluation
            if self.cycle <= 64 { // clear tmp oam memory
                if self.cycle % 2 == 0 {
                    unsafe {
                        let p = &mut self.oam_tmp[((self.cycle - 1) / 8) as usize] as *mut _ as *mut u8;
                        *p.offset(((self.cycle / 2) % 4) as isize) = 0xFF;
                    }
                }
            }
            if self.cycle > 64 && self.cycle <= 256 && self.oam_counter < 64 && self.oam_tmp_counter <= 8 {
                let oam_candidate = self.oam_memory[self.oam_counter];
                let offset_by_y = self.skanline as i16 - oam_candidate.y_position as i16;
                if offset_by_y >= 0 && offset_by_y < self.control.sprite_size() as i16 {
                    if self.oam_counter == 0 && !self.expected_sprite_zero_hit && !self.status.hit_zero_sprite() {
                        self.expected_sprite_zero_hit = true;
                    }
                    if self.oam_tmp_counter < 8 {
                        self.oam_tmp[self.oam_tmp_counter] = oam_candidate;
                        self.oam_tmp_counter += 1;
                    } else {
                        self.status.set_sprite_overlow(true);
                    }
                }
                self.oam_counter += 1;
            }
            // sprite rendering
            if self.mask.sprite_enable_left_column() || self.cycle > 8 {
                let sprite_pixel_with_priority = self.pop_sprite_pixel_with_priority(bg_pixel);
                sprite_pixel = sprite_pixel_with_priority & 0x0F;
                sprite_pixel_priority = sprite_pixel_with_priority >> 4;
            }
        }

        if self.in_visible_range {
            let color_address;

            if bg_pixel & 0x03 == 0 && sprite_pixel & 0x03 != 0 {
                color_address = 0x3F10 + sprite_pixel;
            } else if bg_pixel & 0x03 != 0 && sprite_pixel & 0x03 == 0 {
                color_address = 0x3F00 + bg_pixel;
            } else if sprite_pixel_priority == 0 {
                color_address = 0x3F10 + sprite_pixel;
            } else {
                color_address = 0x3F00 + bg_pixel;
            }
            color = Some(self.pallette_colors[self.read_ppu(color_address) as usize]);
        }

        if self.cycle == 257 && self.skanline <= 239 {
            std::mem::swap(&mut self.oam_tmp, &mut self.oam_buffer);
            self.update_sprite_shift_registers();
        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.skanline += 1;
            self.oam_counter = 0;
            self.oam_tmp_counter = 0;
            if self.skanline > 261 {
                self.skanline = 0;
                self.frame_complete = true;
            }
        }

        self.in_visible_range = self.cycle >= 1 && self.cycle <= 256 && self.skanline <= 239;
        if self.skanline == 241 && self.cycle == 1 {
            self.status.set_vblank(true);
            self.vblank = true;
            if self.control.nmi_flag() {
                self.nmi_require = true;
            }
            if self.debug {
            println!("ppu: start vblank\t| status: {:02X} | control: {:02X} | mask: {:02X} | tmp_addr: {:04X} | cur_addr: {:04X}",
                self.status.data, self.control.data, self.mask.data, self.tmp_addr.data, self.cur_addr.data);
            }
        }
        if self.skanline == 261 && self.cycle == 1 {
            self.status.set_vblank(false);
            self.status.set_sprite_overlow(false);
            self.status.set_hit_zero_sprite(false);
            self.expected_sprite_zero_hit = false;
            self.sprite_low_shift_register = [0; 8];
            self.sprite_high_shift_register = [0; 8];
            self.sprite_attribute_shift_register = [0; 8];
            self.sprite_priority_shift_register = [0; 8];
            self.vblank = false;
            if self.debug {
                println!("ppu: end vblank \t| status: {:02X} | control: {:02X} | mask: {:02X} | tmp_addr: {:04X} | cur_addr: {:04X}",
                    self.status.data, self.control.data, self.mask.data, self.tmp_addr.data, self.cur_addr.data);
            }
        }
        color
    }
}
