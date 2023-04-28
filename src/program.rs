use std::fs::File;
use std::io::prelude::*;

mod mapper;

use mapper::Mapper;
use mapper::mapper000::Mapper000;
use mapper::mapper001::Mapper001;

const PRG_BLOCK_SIZE: usize = 16384;
const CHR_BLOCK_SIZE: usize = 8192;

pub enum Mirroring {
    HORISONTAL,
    VERTICAL,
    UNDEFINED,
}

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mirroring: u8,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut file = File::open(file_name).unwrap();
        let mut memory: Vec<u8> = Vec::new();
        file.read_to_end(&mut memory).unwrap();

        let prg_amount = memory[4] as usize;
        let chr_amount = memory[5] as usize;
        let prg_size = prg_amount * PRG_BLOCK_SIZE;
        let chr_size = chr_amount * CHR_BLOCK_SIZE;
        info!("size_prg: {} | size_chr: {}", prg_amount, chr_amount);
        let mirroring = memory[6] & 0x01;
        let low = (memory[6] & 0xF0) >> 4;
        let high = memory[7] & 0xF0;
        let mapper_id = high | low;

        let trainer = (memory[6] & 0x04) != 0;
        let mut idx = 16;
        if trainer {
            idx += 512;
        }

        let mut prg_rom: Vec<u8> = vec![0; prg_size];
        let mut chr_rom: Vec<u8> = vec![0; chr_size];
        prg_rom.clone_from_slice(&memory[idx .. idx + prg_size]);
        idx += prg_size;
        chr_rom.clone_from_slice(&memory[idx .. idx + chr_size]);
        let mapper = Cartridge::create_mapper(prg_amount, chr_amount, mapper_id);

        Cartridge {
            prg_rom,
            chr_rom,
            mirroring,
            mapper
        }
    }

    fn create_mapper(prg_amount: usize, chr_amount: usize, mapper_id: u8) -> Box<dyn Mapper> {
        let mapper: Box<dyn Mapper> = match mapper_id {
            000 => Box::new(Mapper000::new(prg_amount)),
            001 => Box::new(Mapper001::new(prg_amount, chr_amount)),
            _   => panic!("unknown mapper: {}", mapper_id),
        };
        mapper
    }

    pub fn get_mirroring(&self) -> u8 {
        self.mirroring
    }

    pub fn read_prg_rom(&self, address: u16, data: &mut u8) {
        let mut cartridge_addr = 0;
        if self.mapper.prg_read_addr(address, &mut cartridge_addr) {
            *data = self.prg_rom[cartridge_addr];
        }
    }

    pub fn read_chr_rom(&self, address: u16, data: &mut u8) {
        let mut cartridge_addr = 0;
        if self.mapper.chr_read_addr(address, &mut cartridge_addr) {
            *data = self.chr_rom[cartridge_addr];
        }
    }

    pub fn write_prg_rom(&mut self, address: u16, data: u8) {
        self.mapper.prg_write_addr(address, data);
    }

    pub fn write_chr_rom(&mut self, address: u16, data: u8) {
        self.mapper.chr_write_addr(address, data);
    }
}
