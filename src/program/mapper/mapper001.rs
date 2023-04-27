use super::Mapper;

pub struct Mapper001 {
    prg_amount: usize,
    chr_amount: usize,
    load_register: u8,
    ctrl_register: u8,
}
impl Mapper001 {
    pub fn new(prg_amount: usize, chr_amount: usize) -> Self {
        Mapper001 {
            prg_amount: prg_amount,
            chr_amount: chr_amount,
            load_register: 0,
            ctrl_register: 0,
        }
    }
}

impl Mapper for Mapper001 {
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