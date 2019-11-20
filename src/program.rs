use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    memory: Vec<u8>,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut file = File::open(file_name)?;
        let mut memory: Vec<u8> = Vec::new();
        file.read_to_end(&mut memory);
        Cartridge { memory }
    }
}