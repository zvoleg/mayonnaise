use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    _size_prg: usize,
    _size_chr: usize,
    mirroring: u8,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut file = File::open(file_name).unwrap();
        let mut memory: Vec<u8> = Vec::new();
        file.read_to_end(&mut memory).unwrap();

        let size_prg = memory[4] as usize;
        let size_chr = memory[5] as usize;
        println!("size_prg: {} | size_chr: {}", size_prg, size_chr);
        let mirroring = memory[6] & 0x01;
        let low = (memory[6] & 0xF0) >> 4;
        let high = memory[7] & 0xF0;
        let mapper_id = high | low;

        let trainer = (memory[6] & 0x04) != 0;
        let mut idx = 16;
        if trainer {
            idx += 512;
        }

        let mut prg_rom: Vec<u8> = vec![0; 16384 * size_prg];
        let mut chr_rom: Vec<u8> = vec![0; 8192 * size_chr];
        prg_rom.clone_from_slice(&memory[idx .. idx + (16384 * size_prg)]);
        idx += (16384 * size_prg as u32) as usize;
        chr_rom.clone_from_slice(&memory[idx .. idx + (8192 * size_chr)]);
        let mapper = Cartridge::create_mapper(size_prg, size_chr, mapper_id);

        Cartridge {
            prg_rom,
            chr_rom,
            _size_prg: size_prg,
            _size_chr: size_chr,
            mirroring,
            mapper
        }
    }

    fn create_mapper(size_prg: usize, size_chr: usize, mapper_id: u8) -> Box<dyn Mapper> {
        let mapper = match mapper_id {
            000 => Mapper000 { size_prg, _size_chr: size_chr },
            _   => panic!("unknown mapper: {}", mapper_id),
        };
        Box::new(mapper)
    }

    pub fn get_mirroring(&self) -> u8 {
        self.mirroring
    }

    pub fn read_prg_rom(&self, address: u16, data: &mut u8) {
        let mut cartridge_addr = 0;
        if self.mapper.prg_addr(address, &mut cartridge_addr) {
            *data = self.prg_rom[cartridge_addr];
        }
    }

    pub fn write_to_prg_rom(&mut self, address: u16, data: u8) {
        let mut cartridge_addr = 0;
        if self.mapper.prg_addr(address, &mut cartridge_addr) {
            self.prg_rom[cartridge_addr] = data;
        }
    }

    pub fn read_chr_rom(&self, address: u16, data: &mut u8) {
        let mut cartridge_addr = 0;
        if self.mapper.chr_addr(address, &mut cartridge_addr) {
            *data = self.chr_rom[cartridge_addr];
        }
    }
}

trait Mapper {
    fn prg_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool;
    fn chr_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool;
}

struct Mapper000 {
    size_prg: usize,
    _size_chr: usize
}

impl Mapper for Mapper000 {
    fn prg_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address >= 0x8000 {
            *cartridge_addr = match self.size_prg {
            1 => address & 0x3FFF,
            2 => address & 0x7FFF,
            _ => panic!("wrong size of size_prg: {}", self.size_prg),
            } as usize;
            return true;
        }
        false
    }

    fn chr_addr(&self, address: u16, cartridge_addr: &mut usize) -> bool {
        if address < 0x2000 {
            *cartridge_addr = address as usize;
            return true;
        }
        false
    }
}
