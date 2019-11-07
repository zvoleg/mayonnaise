macro_rules! op {
    ($addr:ident, $instr:ident) => {
        Op { a: &Emu6502::$addr, i: &Emu6502::$instr }
    };
}

const OPCODES: [Op<'static>; 2] = [
    op!(IMM, XEP), op!(IMM, XEP)
];

pub struct Emu6502 {
    acc: u8,
    x: u8,
    y: u8,

    status: u8,
    stack_ptr: u8,
    prog_counter: u16,

    address: u16,
    addr_offset: i8,
    fetched_data: u8,

    opcode: u8,
    cycle_counter: u8,
}

struct Op<'a> {
    a: &'a dyn Fn(&mut Emu6502),
    i: &'a dyn Fn(&mut Emu6502),
}

#[allow(non_snake_case)]
impl Emu6502 {
    pub fn new() -> Emu6502 {
        Emu6502 {
            acc: 0,
            x: 0,
            y: 0,

            status: 32,
            stack_ptr: 0,
            prog_counter: 0,

            address: 0,
            addr_offset: 0,
            fetched_data: 0,

            opcode: 0,
            cycle_counter: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cycle_counter == 0 {
            self.opcode = Emu6502::read_data(self.prog_counter);
            self.prog_counter += 1;
            let addressing = OPCODES[self.opcode as usize].a;
            addressing(self);
            let instruction = OPCODES[self.opcode as usize].i;
            instruction(self);
        }
        self.cycle_counter -= 1;
    }

    fn read_data(address: u16) -> u8 {
        0
    }

    // Addressing modes

    fn IMM(&mut self) {
        self.address = self.prog_counter;
        self.prog_counter += 1;
    }

    fn ACC(&mut self) {
        self.fetched_data = self.acc;
    }

    fn IMP(&mut self) {}

    fn ABS(&mut self) {
        let low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn ABX(&mut self) {
        let low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(self.x as u16);
        self.address = result_address;
    }

    fn ABY(&mut self) {
        let low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(self.y as u16);
        self.address = result_address;
    }

    fn ZP0(&mut self) {
        let low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        self.address = low as u16;
    }

    fn ZPX(&mut self) {
        let base_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(self.x);
        self.address = low as u16;
    }

    fn ZPY(&mut self) {
        let base_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(self.y);
        self.address = low as u16;
    }

    fn IND(&mut self) {
        let indirect_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_high = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        let low = Emu6502::read_data(indirect_address);
        let high = Emu6502::read_data(indirect_address + 1);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn INX(&mut self) {
        let base_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let (indirect_low, _overflow) = base_low.overflowing_add(self.x);
        let low = Emu6502::read_data(indirect_low as u16);
        let (indirect_high, _overflow) = indirect_low.overflowing_add(1);
        let high = Emu6502::read_data(indirect_high as u16);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn INY(&mut self) {
        let indirect_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let (next_byte, _overflow) = indirect_low.overflowing_add(1);
        let indirect_high = Emu6502::read_data(next_byte as u16);
        let mut result_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        result_address += self.y as u16;
        self.address = result_address;
    }

    fn REL(&mut self) {
        let offset = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        self.addr_offset = offset as i8;
    }

    // Instructions set

    fn XEP(&mut self) {
        panic!("undefinded opcode: {}");
    }
}
