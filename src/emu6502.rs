use std::ops::DerefMut;

pub struct Emu6502 {
    registers: Registers,
    opcodes: Vec<Opcode>,

    opcode: u8,
    cycle_counter: u8,
}

struct Registers {
    acc: u8,
    x: u8,
    y: u8,

    status: u8,
    stack_ptr: u8,
    prog_counter: u16,

    address: u16,
    addr_offset: i8,
    fetched_data: u8,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            acc: 0,
            x: 0,
            y: 0,

            status: 0,
            stack_ptr: 0,
            prog_counter: 0,

            address: 0,
            addr_offset: 0,
            fetched_data: 0,
        }
    }
}

struct Opcode {
    addressing: Box<dyn FnMut(&mut Registers)>,
    instruction: Box<dyn FnMut(&mut Registers)>,
}

#[allow(non_snake_case)]
impl Opcode {
    fn new(
        addressing: Box<dyn FnMut(&mut Registers)>,
        instruction: Box<dyn FnMut(&mut Registers)>,
    ) -> Opcode {
        Opcode {
            addressing,
            instruction,
        }
    }

    // Addressing modes

    fn IMM(registers: &mut Registers) {
        registers.address = registers.prog_counter;
        registers.prog_counter += 1;
    }

    fn ACC(registers: &mut Registers) {
        registers.fetched_data = registers.acc;
    }

    fn IMP(registers: &mut Registers) {}

    fn ABS(registers: &mut Registers) {
        let low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let high = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        registers.address = ((high as u16) << 8) | low as u16;
    }

    fn ABX(registers: &mut Registers) {
        let low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let high = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(registers.x as u16);
        registers.address = result_address;
    }

    fn ABY(registers: &mut Registers) {
        let low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let high = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(registers.y as u16);
        registers.address = result_address;
    }

    fn ZP0(registers: &mut Registers) {
        let low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        registers.address = low as u16;
    }

    fn ZPX(registers: &mut Registers) {
        let base_low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(registers.x);
        registers.address = low as u16;
    }

    fn ZPY(registers: &mut Registers) {
        let base_low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(registers.y);
        registers.address = low as u16;
    }

    fn IND(registers: &mut Registers) {
        let indirect_low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let indirect_high = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let indirect_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        let low = Emu6502::read_data(indirect_address);
        let high = Emu6502::read_data(indirect_address + 1);
        registers.address = ((high as u16) << 8) | low as u16;
    }

    fn INX(registers: &mut Registers) {
        let base_low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let (indirect_low, _overflow) = base_low.overflowing_add(registers.x);
        let low = Emu6502::read_data(indirect_low as u16);
        let (indirect_high, _overflow) = indirect_low.overflowing_add(1);
        let high = Emu6502::read_data(indirect_high as u16);
        registers.address = ((high as u16) << 8) | low as u16;
    }

    fn INY(registers: &mut Registers) {
        let indirect_low = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        let (next_byte, _overflow) = indirect_low.overflowing_add(1);
        let indirect_high = Emu6502::read_data(next_byte as u16);
        let mut result_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        result_address += registers.y as u16;
        registers.address = result_address;
    }

    fn REL(registers: &mut Registers) {
        let offset = Emu6502::read_data(registers.prog_counter);
        registers.prog_counter += 1;
        registers.addr_offset = offset as i8;
    }

    // Instructions set

    fn XEP(registers: &mut Registers) {
        panic!("undefinded opcode: {}");
    }
}

impl Emu6502 {
    pub fn new() -> Emu6502 {
        let opcodes = vec![Opcode::new(Box::new(Opcode::IMP), Box::new(Opcode::XEP))];
        let registers = Registers::new();
        Emu6502 {
            registers,
            opcodes,

            opcode: 0,
            cycle_counter: 0,
        }
    }

    fn clock(&mut self) {
        if self.cycle_counter == 0 {
            self.opcode = Emu6502::read_data(self.registers.prog_counter);
            self.registers.prog_counter += 1;
            let addressing = self.opcodes[self.opcode as usize].addressing.deref_mut();
            addressing(&mut self.registers);
            let instruction = self.opcodes[self.opcode as usize].instruction.deref_mut();
            instruction(&mut self.registers);
        }
        self.cycle_counter -= 1;
    }

    fn read_data(address: u16) -> u8 {
        0
    }
}
