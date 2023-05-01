use core::panic;

use super::Mapper;
use crate::program::Mirroring;

const PRG_BLOCK_SIZE: usize = 16384;
const CHR_BLOCK_SIZE: usize = 8192;

enum PRG_MODE {
    SWITCH_32,
    FIX_FIRST_16,
    FIX_LAST_16,
}

enum CHR_MODE {
    K4,
    K8,
}

pub struct Mapper001 {
    prg_amount: usize,
    chr_amount: usize,

    chr_bank_0: usize,
    chr_bank_1: usize,

    low_bank_offset: usize,  // 0x8000
    high_bank_offset: usize, // 0xC000

    shift_reg: u8,
    prg_wrt_counter: u8,

    mirroring: Mirroring,
    prg_bank_mode: PRG_MODE,
    chr_bank_mode: CHR_MODE,
}
impl Mapper001 {
    pub fn new(prg_amount: usize, chr_amount: usize) -> Self {
        Mapper001 {
            prg_amount: prg_amount,
            chr_amount: chr_amount,

            chr_bank_0: 0,
            chr_bank_1: 0,

            low_bank_offset: 0,
            high_bank_offset: (prg_amount - 1) * PRG_BLOCK_SIZE,

            shift_reg: 0,
            prg_wrt_counter: 0,

            mirroring: Mirroring::UNDEFINED,
            prg_bank_mode: PRG_MODE::FIX_LAST_16,
            chr_bank_mode: CHR_MODE::K8,
        }
    }
}

impl Mapper001 {
    fn reset(&mut self) {
        self.low_bank_offset = 0;
        self.high_bank_offset = (self.prg_amount - 1) * PRG_BLOCK_SIZE;

        self.shift_reg = 0;
        self.prg_wrt_counter = 0;
    } 
}

impl Mapper for Mapper001 {
    fn prg_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address >= 0x8000 && address < 0xC000 {
            *cartridge_addr = self.low_bank_offset + (address & 0x3FFF) as usize;
            return true;
        }
        if address >= 0xC000 {
            *cartridge_addr = self.high_bank_offset + (address & 0x3FFF) as usize;
            return true;
        }
        false
    }

    fn prg_write_addr(&mut self, address: u16, data: u8) {
        if address < 0x8000 {
            return;
        }
        self.prg_wrt_counter += 1;
        if address >= 0x8000 {
            if data & 0x80 != 0 {
                self.reset();
                return
            }
            let bit = (data & 0x1) << 4;
            self.shift_reg = (self.shift_reg >> 1) | bit;
        }

        if self.prg_wrt_counter == 5 {
            self.prg_wrt_counter = 0;

            self.shift_reg = self.shift_reg;
            let reg_selector = address;
            if reg_selector >= 0x8000 && reg_selector < 0xA000 { // control reg
                self.mirroring = match self.shift_reg & 0x3 {
                    0 => Mirroring::UNDEFINED,
                    1 => Mirroring::UNDEFINED,
                    2 => Mirroring::VERTICAL,
                    3 => Mirroring::HORISONTAL,
                    _ => Mirroring::UNDEFINED,
                };
                self.prg_bank_mode = match (self.shift_reg >> 2) & 0x3 {
                    0 | 1 => PRG_MODE::SWITCH_32,
                    2 => {
                        self.low_bank_offset = 0;
                        PRG_MODE::FIX_FIRST_16
                    },
                    3 => {
                        self.high_bank_offset = (self.prg_amount - 1) * PRG_BLOCK_SIZE;
                        PRG_MODE::FIX_LAST_16
                    }, 
                    _ => panic!("unexpected value for prg bank mode"),
                };
                self.chr_bank_mode = match self.shift_reg & 0x10 == 0 {
                    true => CHR_MODE::K8,
                    false => CHR_MODE::K4,
                }
            } else if reg_selector >= 0xA000 && reg_selector < 0xC000 { // chr bank 0
                match self.chr_bank_mode {
                    CHR_MODE::K4 => self.chr_bank_0 = self.shift_reg as usize,
                    CHR_MODE::K8 => {
                        self.chr_bank_0 = (self.shift_reg >> 1) as usize;
                        self.chr_bank_1 = self.chr_bank_0 + 1;
                    },
                };
            } else if reg_selector >= 0xC000 && reg_selector < 0xE000 { // chr bank 1
                self.chr_bank_1 = match self.chr_bank_mode {
                    CHR_MODE::K4 => self.shift_reg as usize,
                    CHR_MODE::K8 => self.chr_bank_1,
                };
            } else if reg_selector >= 0xE000 { // prg bank
                info!("{:04X}", self.shift_reg);
                match self.prg_bank_mode {
                    PRG_MODE::FIX_FIRST_16 => {
                        self.high_bank_offset = PRG_BLOCK_SIZE * (self.shift_reg & 0xFF) as usize;
                    },
                    PRG_MODE::FIX_LAST_16 => {
                        self.low_bank_offset = PRG_BLOCK_SIZE * (self.shift_reg & 0xFF) as usize;
                    },
                    PRG_MODE::SWITCH_32 => {
                        let bank = ((self.shift_reg & 0xFF) >> 1) as usize;
                        self.low_bank_offset = bank * PRG_BLOCK_SIZE;
                        self.high_bank_offset = (bank + 1) * PRG_BLOCK_SIZE;
                    },
                };
            }
        }
    }

    fn chr_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address < 0x0FFF {
            *cartridge_addr = self.chr_bank_0 * (CHR_BLOCK_SIZE / 2) + address as usize;
            return true;
        }
        if address >= 0x1000 && address < 0x2000 {
            *cartridge_addr = self.chr_bank_1 * (CHR_BLOCK_SIZE / 2) + (address & 0x0FFF) as usize;
            return true;
        }
        false
    }

    fn chr_write_addr(&mut self, address: u16, data: u8) {
        
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring
    }
}