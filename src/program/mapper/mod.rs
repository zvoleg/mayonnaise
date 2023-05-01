use super::Mirroring;

pub mod mapper000;
pub mod mapper001;

pub trait Mapper {
    fn prg_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool;
    fn prg_write_addr(&mut self, address: u16, data: u8);
    fn chr_read_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool;
    fn chr_write_addr(&mut self, address: u16, data: u8);
    fn mirroring(&self) -> Mirroring;
}
