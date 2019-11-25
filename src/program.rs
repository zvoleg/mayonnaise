use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    size_prg: u8,
    size_chr: u8,
    mapper_id: u8,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut file = File::open(file_name).unwrap();
        let mut memory: Vec<u8> = Vec::new();
        file.read_to_end(&mut memory).unwrap();

        let size_prg = memory[4];
        let size_chr = memory[5];
        let low = (memory[6] & 0xF0) >> 4;
        let high = memory[7] & 0xF0;
        let mapper_id = high | low;

        let trainer = (memory[6] & 0x04) != 0;
        let mut idx = 16;
        if trainer {
            idx += 512;
        }
        println!("memory len = {}", memory.len());
        println!("trainer = {}", trainer);
        println!("size_prg = {}", size_prg);
        println!("size_chr = {}", size_chr);

        let mut prg_rom: Vec<u8> = vec![0; (16384 * size_prg as u16) as usize];
        let mut chr_rom: Vec<u8> = vec![0; (8192 * size_chr as u16) as usize];
        prg_rom.clone_from_slice(&memory[idx..idx + (16384 * size_prg as u16) as usize]);
        idx += (16384 * size_prg as u16) as usize;
        chr_rom.clone_from_slice(&memory[idx..idx + (8192 * size_chr as u16) as usize]);

        Cartridge {
            prg_rom,
            chr_rom,
            size_prg,
            size_chr,
            mapper_id
        }
    }

    pub fn read_prg_rom(&self, address: u16) -> u8 {
        match self.mapper_id {
            000 => Mapper000{size_prg: self.size_prg, size_chr: self.size_chr}.read_prg_rom(&self.prg_rom, address),
            _   => 0
        }
    }
}

trait Mapper {
    fn read_prg_rom(&self, memory: &Vec<u8>, address: u16) -> u8;
    fn read_chr_rom(&self, memory: &Vec<u8>, address: u16) -> u8;
}

struct Mapper000 {
    size_prg: u8,
    size_chr: u8
}

impl Mapper for Mapper000 {
    fn read_prg_rom(&self, memory: &Vec<u8>, address: u16) -> u8 {
        let mut idx = address & 0x3FFF;
        if self.size_prg == 2 && address >= 0xC000 {
            idx += 16384;
        }
        memory[idx as usize]
    }

    fn read_chr_rom(&self, memory: &Vec<u8>, address: u16) -> u8 {
        0
    }
}
