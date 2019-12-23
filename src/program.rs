use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    size_prg: u8,
    size_chr: u8,
    mirroring: u8,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut file = File::open(file_name).unwrap();
        let mut memory: Vec<u8> = Vec::new();
        file.read_to_end(&mut memory).unwrap();

        let size_prg = memory[4];
        let size_chr = memory[5];
        let mirroring = memory[6] & 0x01;
        let low = (memory[6] & 0xF0) >> 4;
        let high = memory[7] & 0xF0;
        let mapper_id = high | low;

        let trainer = (memory[6] & 0x04) != 0;
        let mut idx = 16;
        if trainer {
            idx += 512;
        }

        let mut prg_rom: Vec<u8> = vec![0; (16384 * size_prg as u16) as usize];
        let mut chr_rom: Vec<u8> = vec![0; (8192 * size_chr as u16) as usize];
        prg_rom.clone_from_slice(&memory[idx..idx + (16384 * size_prg as usize)]);
        idx += (16384 * size_prg as u16) as usize;
        chr_rom.clone_from_slice(&memory[idx..idx + (8192 * size_chr as usize)]);
        let mapper = Cartridge::create_mapper(size_prg, size_chr, mapper_id);

        Cartridge {
            prg_rom,
            chr_rom,
            size_prg,
            size_chr,
            mirroring,
            mapper
        }
    }

    fn create_mapper(size_prg: u8, size_chr: u8, mapper_id: u8) -> Box<dyn Mapper> {
        let mapper = match mapper_id {
            000 => Mapper000 { size_prg, size_chr },
            _   => panic!("unknown mapper: {}", mapper_id),
        };
        Box::new(mapper)
    }

    pub fn get_mirroring(&self) -> u8 {
        self.mirroring
    }

    pub fn read_prg_rom(&self, address: u16, data: &mut u8) {
        let idx = self.mapper.prg_addr(address);
        *data = self.prg_rom[idx];
    }

    pub fn read_chr_rom(&self, address: u16, data: &mut u8) {
        let idx = self.mapper.chr_addr(address);
        *data = self.chr_rom[idx];
    }
}

trait Mapper {
    fn prg_addr(&self, address: u16) -> usize;
    fn chr_addr(&self, address: u16) -> usize;
}

struct Mapper000 {
    size_prg: u8,
    size_chr: u8
}

impl Mapper for Mapper000 {
    fn prg_addr(&self, address: u16) -> usize {
        let mut idx = address & 0x3FFF;
        if self.size_prg == 2 && address >= 0xC000 {
            idx += 16384;
        }
        idx as usize
    }

    fn chr_addr(&self, address: u16) -> usize {
        address as usize
    }
}
