use crate::program::Mirroring;

use super::Mapper;

pub struct Mapper000 {
    prg_amount: usize,
    mirroring: Mirroring,
}

impl Mapper000 {
    pub fn new(prg_amount: usize, mirroring_bit: u8) -> Self {
        let mirroring = match mirroring_bit {
            0 => Mirroring::HORISONTAL,
            1 => Mirroring::VERTICAL,
            _ => Mirroring::UNDEFINED,
        };
        Mapper000 { prg_amount: prg_amount, mirroring: mirroring }
    }
}

impl Mapper for Mapper000 {
    fn prg_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address >= 0x8000 {
            *cartridge_addr = match self.prg_amount {
            1 => address & 0x3FFF,
            2 => address & 0x7FFF,
            _ => panic!("wrong size of size_prg: {}", self.prg_amount),
            } as usize;
            return true;
        }
        false
    }

    fn prg_write_addr(&mut self, _address: u16, _data: u8) {
        
    }

    fn chr_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address < 0x2000 {
            *cartridge_addr = address as usize;
            return true;
        }
        false
    }

    fn chr_write_addr(&mut self, _address: u16, _data: u8) {
        
    }

    fn mirroring(&self) -> Mirroring {
        self.mirroring
    }
}
