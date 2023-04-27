use super::Mapper;

pub struct Mapper000 {
    prg_amount: usize,
}

impl Mapper000 {
    pub fn new(prg_amount: usize) -> Self {
        Mapper000 { prg_amount: prg_amount }
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

    fn prg_write_addr(&self, address: u16, data: u8) {
        
    }

    fn chr_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address < 0x2000 {
            *cartridge_addr = address as usize;
            return true;
        }
        false
    }

    fn chr_write_addr(&self, address: u16, data: u8) {
        
    }
}
