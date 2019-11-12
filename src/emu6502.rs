use std::cell::RefCell;
use super::bus::Bus;

macro_rules! op {
    ($ind: literal, $addr:ident, $instr:ident, $amount: expr) => {
        Op { addressing_mode: &Emu6502::$addr, instruction: &Emu6502::$instr, cycle_amount: $amount }
    };
}

struct Op<'a> {
    addressing_mode: &'a dyn Fn(&mut Emu6502),
    instruction: &'a dyn Fn(&mut Emu6502),
    cycle_amount: u8
}

const OPCODES: [Op<'static>; 256] = [
    op!(0x00, IMP, BRK, 7), op!(0x01, IDX, ORA, 6), op!(0x02, IMM, XEP, 0), op!(0x03, IMM, XEP, 0), op!(0x04, IMM, XEP, 0), op!(0x05, ZP0, ORA, 3), op!(0x06, ZP0, ASL, 5), op!(0x07, IMM, XEP, 0), op!(0x08, IMP, PHP, 3), op!(0x09, IMM, ORA, 2), op!(0x0A, ACC, ASL, 2), op!(0x0B, IMM, XEP, 0), op!(0x0C, IMM, XEP, 0), op!(0x0D, ABS, ORA, 4), op!(0x0E, ABS, ASL, 6), op!(0x0F, IMM, XEP, 0),
    op!(0x10, REL, BPL, 2), op!(0x11, IDY, ORA, 5), op!(0x12, IMM, XEP, 0), op!(0x13, IMM, XEP, 0), op!(0x14, IMM, XEP, 0), op!(0x15, ZPX, ORA, 4), op!(0x16, ZPX, ASL, 6), op!(0x17, IMM, XEP, 0), op!(0x18, IMP, CLC, 2), op!(0x19, ABY, ORA, 4), op!(0x1A, IMM, XEP, 0), op!(0x1B, IMM, XEP, 0), op!(0x1C, IMM, XEP, 0), op!(0x1D, ABX, ORA, 4), op!(0x1E, ABX, ASL, 7), op!(0x1F, IMM, XEP, 0),
    op!(0x20, ABS, JSR, 6), op!(0x21, IDX, AND, 6), op!(0x22, IMM, XEP, 0), op!(0x23, IMM, XEP, 0), op!(0x24, ZP0, BIT, 3), op!(0x25, ZP0, AND, 3), op!(0x26, ZP0, ROL, 5), op!(0x27, IMM, XEP, 0), op!(0x28, IMP, PLP, 4), op!(0x29, IMM, AND, 2), op!(0x2A, ACC, ROL, 2), op!(0x2B, IMM, XEP, 0), op!(0x2C, ABS, BIT, 4), op!(0x2D, ABS, AND, 4), op!(0x2E, ABS, ROL, 6), op!(0x2F, IMM, XEP, 0),
    op!(0x30, REL, BMI, 2), op!(0x31, IDY, AND, 5), op!(0x32, IMM, XEP, 0), op!(0x33, IMM, XEP, 0), op!(0x34, IMM, XEP, 0), op!(0x35, ZPX, AND, 4), op!(0x36, ZPX, ROL, 6), op!(0x37, IMM, XEP, 0), op!(0x38, IMP, SEC, 2), op!(0x39, ABY, AND, 4), op!(0x3A, IMM, XEP, 0), op!(0x3B, IMM, XEP, 0), op!(0x3C, IMM, XEP, 0), op!(0x3D, ABX, AND, 4), op!(0x3E, ABX, ROL, 7), op!(0x3F, IMM, XEP, 0),
    op!(0x40, IMP, RTI, 6), op!(0x41, IDX, EOR, 6), op!(0x42, IMM, XEP, 0), op!(0x43, IMM, XEP, 0), op!(0x44, IMM, XEP, 0), op!(0x45, ZP0, EOR, 3), op!(0x46, ZP0, LSR, 5), op!(0x47, IMM, XEP, 0), op!(0x48, IMP, PHA, 3), op!(0x49, IMM, EOR, 2), op!(0x4A, ACC, LSR, 2), op!(0x4B, IMM, XEP, 0), op!(0x4C, ABS, JMP, 3), op!(0x4D, ABS, EOR, 4), op!(0x4E, ABS, LSR, 6), op!(0x4F, IMM, XEP, 0),
    op!(0x50, REL, BVC, 2), op!(0x51, IDY, EOR, 5), op!(0x52, IMM, XEP, 0), op!(0x53, IMM, XEP, 0), op!(0x54, IMM, XEP, 0), op!(0x55, ZPX, EOR, 4), op!(0x56, ZPX, LSR, 6), op!(0x57, IMM, XEP, 0), op!(0x58, IMP, CLI, 2), op!(0x59, ABY, EOR, 4), op!(0x5A, IMM, XEP, 0), op!(0x5B, IMM, XEP, 0), op!(0x5C, IMM, XEP, 0), op!(0x5D, ABX, EOR, 4), op!(0x5E, ABX, LSR, 7), op!(0x5F, IMM, XEP, 0),
    op!(0x60, IMP, RTS, 6), op!(0x61, IDX, ADC, 6), op!(0x62, IMM, XEP, 0), op!(0x63, IMM, XEP, 0), op!(0x64, IMM, XEP, 0), op!(0x65, ZP0, ADC, 3), op!(0x66, ZP0, ROR, 5), op!(0x67, IMM, XEP, 0), op!(0x68, IMP, PLA, 4), op!(0x69, IMM, ADC, 2), op!(0x6A, ACC, ROR, 2), op!(0x6B, IMM, XEP, 0), op!(0x6C, IND, JMP, 5), op!(0x6D, ABS, ADC, 4), op!(0x6E, ABS, ROR, 6), op!(0x6F, IMM, XEP, 0),
    op!(0x70, REL, BVS, 2), op!(0x71, IDY, ADC, 5), op!(0x72, IMM, XEP, 0), op!(0x73, IMM, XEP, 0), op!(0x74, IMM, XEP, 0), op!(0x75, ZPX, ADC, 4), op!(0x76, ZPX, ROR, 6), op!(0x77, IMM, XEP, 0), op!(0x78, IMP, SEI, 2), op!(0x79, ABY, ADC, 4), op!(0x7A, IMM, XEP, 0), op!(0x7B, IMM, XEP, 0), op!(0x7C, IMM, XEP, 0), op!(0x7D, ABX, ADC, 4), op!(0x7E, ABX, ROR, 7), op!(0x7F, IMM, XEP, 0),
    op!(0x80, IMM, XEP, 0), op!(0x81, IDX, STA, 6), op!(0x82, IMM, XEP, 0), op!(0x83, IMM, XEP, 0), op!(0x84, ZP0, STY, 3), op!(0x85, ZP0, STA, 3), op!(0x86, ZP0, STX, 3), op!(0x87, IMM, XEP, 0), op!(0x88, IMP, DEY, 2), op!(0x89, IMM, XEP, 0), op!(0x8A, IMM, TXA, 2), op!(0x8B, IMM, XEP, 0), op!(0x8C, ABS, STY, 4), op!(0x8D, ABS, STA, 4), op!(0x8E, ABS, STX, 4), op!(0x8F, IMM, XEP, 0),
    op!(0x90, REL, BCC, 2), op!(0x91, IDY, STA, 6), op!(0x92, IMM, XEP, 0), op!(0x93, IMM, XEP, 0), op!(0x94, ZPX, STY, 4), op!(0x95, ZPX, STA, 4), op!(0x96, ZPY, STX, 4), op!(0x97, IMM, XEP, 0), op!(0x98, IMP, TYA, 2), op!(0x99, ABY, STA, 5), op!(0x9A, IMP, TXS, 2), op!(0x9B, IMM, XEP, 0), op!(0x9C, IMM, XEP, 0), op!(0x9D, ABX, STA, 5), op!(0x9E, IMM, XEP, 0), op!(0x9F, IMM, XEP, 0),
    op!(0xA0, IMM, LDY, 2), op!(0xA1, IDX, LDA, 6), op!(0xA2, IMM, LDX, 2), op!(0xA3, IMM, XEP, 0), op!(0xA4, ZP0, LDY, 3), op!(0xA5, ZP0, LDA, 3), op!(0xA6, ZP0, LDX, 3), op!(0xA7, IMM, XEP, 0), op!(0xA8, IMP, TAY, 2), op!(0xA9, IMM, LDA, 2), op!(0xAA, IMP, TAX, 2), op!(0xAB, IMM, XEP, 0), op!(0xAC, ABS, LDY, 4), op!(0xAD, ABS, LDA, 4), op!(0xAE, ABS, LDX, 4), op!(0xAF, IMM, XEP, 0),
    op!(0xB0, REL, BCS, 2), op!(0xB1, IDY, LDA, 5), op!(0xB2, IMM, XEP, 0), op!(0xB3, IMM, XEP, 0), op!(0xB4, ZPX, LDY, 4), op!(0xB5, ZPX, LDA, 4), op!(0xB6, ZPY, LDX, 4), op!(0xB7, IMM, XEP, 0), op!(0xB8, IMP, CLV, 2), op!(0xB9, ABY, LDA, 4), op!(0xBA, IMP, TSX, 2), op!(0xBB, IMM, XEP, 0), op!(0xBC, ABX, LDY, 4), op!(0xBD, ABX, LDA, 4), op!(0xBE, ABY, LDX, 4), op!(0xBF, IMM, XEP, 0),
    op!(0xC0, IMM, CPY, 2), op!(0xC1, IDX, CMP, 6), op!(0xC2, IMM, XEP, 0), op!(0xC3, IMM, XEP, 0), op!(0xC4, ZP0, CPY, 3), op!(0xC5, ZP0, CMP, 3), op!(0xC6, ZP0, DEC, 5), op!(0xC7, IMM, XEP, 0), op!(0xC8, IMP, INY, 2), op!(0xC9, IMM, CMP, 2), op!(0xCA, IMP, DEX, 2), op!(0xCB, IMM, XEP, 0), op!(0xCC, ABS, CPY, 4), op!(0xCD, ABS, CMP, 4), op!(0xCE, ABS, DEC, 6), op!(0xCF, IMM, XEP, 0),
    op!(0xD0, REL, BNE, 2), op!(0xD1, IDY, CMP, 5), op!(0xD2, IMM, XEP, 0), op!(0xD3, IMM, XEP, 0), op!(0xD4, IMM, XEP, 0), op!(0xD5, ZPX, CMP, 4), op!(0xD6, ZPX, DEC, 6), op!(0xD7, IMM, XEP, 0), op!(0xD8, IMP, CLD, 2), op!(0xD9, ABY, CMP, 4), op!(0xDA, IMM, XEP, 0), op!(0xDB, IMM, XEP, 0), op!(0xDC, IMM, XEP, 0), op!(0xDD, ABX, CMP, 4), op!(0xDE, ABX, DEC, 7), op!(0xDF, IMM, XEP, 0),
    op!(0xE0, IMM, CPX, 2), op!(0xE1, IDX, SBC, 6), op!(0xE2, IMM, XEP, 0), op!(0xE3, IMM, XEP, 0), op!(0xE4, ZP0, CPX, 3), op!(0xE5, ZP0, SBC, 3), op!(0xE6, ZP0, INC, 5), op!(0xE7, IMM, XEP, 0), op!(0xE8, IMP, INX, 2), op!(0xE9, IMM, SBC, 2), op!(0xEA, IMP, NOP, 2), op!(0xEB, IMM, XEP, 0), op!(0xEC, ABS, CPX, 4), op!(0xED, ABS, SBC, 4), op!(0xEE, ABS, INC, 6), op!(0xEF, IMM, XEP, 0),
    op!(0xF0, REL, BEQ, 2), op!(0xF1, IDY, SBC, 5), op!(0xF2, IMM, XEP, 0), op!(0xF3, IMM, XEP, 0), op!(0xF4, IMM, XEP, 0), op!(0xF5, ZPX, SBC, 4), op!(0xF6, ZPX, INC, 6), op!(0xF7, IMM, XEP, 0), op!(0xF8, IMP, SED, 2), op!(0xF9, ABY, SBC, 4), op!(0xFA, IMM, XEP, 0), op!(0xFB, IMM, XEP, 0), op!(0xFC, IMM, XEP, 0), op!(0xFD, ABX, SBC, 4), op!(0xFE, ABX, INC, 7), op!(0xFF, IMM, XEP, 0)
];

enum Flag {
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    V = 1 << 6,
    S = 1 << 7
}

pub struct Emu6502 {
    acc: u8,
    x: u8,
    y: u8,

    status: u8,
    stack_ptr: u8,
    prog_counter: u16,

    address: u16,
    addr_offset: u16,
    fetched_data: u8,

    opcode: u8,
    cycle_counter: u8,
    additional_cycles: u8,

    bus: RefCell<Bus>
}


#[allow(non_snake_case)]
impl Emu6502 {
    pub fn new(bus: RefCell<Bus>) -> Emu6502 {
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
            additional_cycles: 0,

            bus
        }
    }

    pub fn clock(&mut self) {
        if self.cycle_counter <= 0 {
            self.additional_cycles = 0;
            self.opcode = self.read_data(self.prog_counter);
            self.prog_counter += 1;
            let op = &OPCODES[self.opcode as usize];
            (op.addressing_mode)(self);
            (op.instruction)(self);
            self.cycle_counter = op.cycle_amount;
        }
        self.cycle_counter -= 1;
    }

    fn set_flag(&mut self, flag: Flag, state: u8) {
        match state {
            0 => self.status &= !(flag as u8),
            1 => self.status |= flag as u8,
            _ => panic!("try to set flag to wrong state: {:#010b} -> {}", flag as u8, state)
        }
    }

    fn get_flag(&self, flag: Flag) -> u8 {
        self.status & flag as u8
    }

    fn read_data(&self, address: u16) -> u8 {
        self.bus.borrow().read_data(address)
    }

    fn write_data(&self, address: u16, data: u8) {
        self.bus.borrow_mut().write_data(address, data);
    }

    fn fetch(&mut self) -> u8 {
        self.fetched_data = self.read_data(self.address);
        self.fetched_data
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
        let low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn ABX(&mut self) {
        let low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(self.x as u16);
        self.address = result_address;
        if (self.address & 0xFF00) != ((high as u16) << 8) {
            self.additional_cycles = 1;
        }
    }

    fn ABY(&mut self) {
        let low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let high = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let base_address = ((high as u16) << 8) | low as u16;
        let (result_address, _overflow) = base_address.overflowing_add(self.y as u16);
        self.address = result_address;
        if (self.address & 0xFF00) != ((high as u16) << 8) {
            self.additional_cycles = 1;
        }
    }

    fn ZP0(&mut self) {
        let low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        self.address = low as u16;
    }

    fn ZPX(&mut self) {
        let base_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(self.x);
        self.address = low as u16;
    }

    fn ZPY(&mut self) {
        let base_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let (low, _overflow) = base_low.overflowing_add(self.y);
        self.address = low as u16;
    }

    fn IND(&mut self) {
        let indirect_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_high = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        let low = self.read_data(indirect_address);
        let high = self.read_data(indirect_address + 1);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn IDX(&mut self) {
        let base_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let (indirect_low, _overflow) = base_low.overflowing_add(self.x);
        let low = self.read_data(indirect_low as u16);
        let (indirect_high, _overflow) = indirect_low.overflowing_add(1);
        let high = self.read_data(indirect_high as u16);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn IDY(&mut self) {
        let indirect_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let (next_byte, _overflow) = indirect_low.overflowing_add(1);
        let indirect_high = self.read_data(next_byte as u16);
        let mut result_address = ((indirect_high as u16) << 8) | indirect_low as u16;
        result_address += self.y as u16;
        self.address = result_address;
        if (self.address & 0xFF00) != ((indirect_high as u16) << 8) {
            self.additional_cycles = 1;
        }
    }

    fn REL(&mut self) {
        let offset = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        if (offset & 0x80) == 0x80 {
            self.addr_offset = 0xFF00 | offset as u16;
        } else {
            self.addr_offset = offset as u16;
        }
    }

    // Instructions set

    fn ADC(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn AND(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn ASL(&mut self) {

    }

    fn BCC(&mut self) {

    }

    fn BCS(&mut self) {

    }

    fn BEQ(&mut self) {

    }

    fn BIT(&mut self) {

    }

    fn BMI(&mut self) {

    }

    fn BNE(&mut self) {

    }

    fn BPL(&mut self) {

    }

    fn BRK(&mut self) {

    }

    fn BVC(&mut self) {

    }

    fn BVS(&mut self) {

    }

    fn CLC(&mut self) {

    }

    fn CLD(&mut self) {

    }

    fn CLI(&mut self) {

    }

    fn CLV(&mut self) {

    }

    fn CMP(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn CPX(&mut self) {

    }

    fn CPY(&mut self) {

    }

    fn DEC(&mut self) {

    }

    fn DEX(&mut self) {

    }

    fn DEY(&mut self) {

    }

    fn EOR(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn INC(&mut self) {

    }

    fn INX(&mut self) {

    }

    fn INY(&mut self) {

    }

    fn JMP(&mut self) {

    }

    fn JSR(&mut self) {

    }

    fn LDA(&mut self) {
        self.cycle_counter += self.additional_cycles;
        self.acc = self.fetch();
        if self.acc == 0x0000 {
            self.set_flag(Flag::Z, 1);
        }
        if self.acc & (Flag::S as u8) == (Flag::S as u8) {
            self.set_flag(Flag::S, 1)
        }
    }

    fn LDX(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn LDY(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn LSR(&mut self) {

    }

    fn NOP(&mut self) {

    }

    fn ORA(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn PHA(&mut self) {

    }

    fn PHP(&mut self) {

    }

    fn PLA(&mut self) {

    }

    fn PLP(&mut self) {

    }

    fn ROL(&mut self) {

    }

    fn ROR(&mut self) {

    }

    fn RTI(&mut self) {

    }

    fn RTS(&mut self) {

    }

    fn SBC(&mut self) {
        self.cycle_counter += self.additional_cycles;
    }

    fn SEC(&mut self) {

    }

    fn SED(&mut self) {

    }

    fn SEI(&mut self) {

    }

    fn STA(&mut self) {
        self.write_data(self.address, self.acc);
    }

    fn STX(&mut self) {

    }

    fn STY(&mut self) {

    }

    fn TAX(&mut self) {

    }

    fn TAY(&mut self) {

    }

    fn TSX(&mut self) {

    }

    fn TXA(&mut self) {

    }

    fn TXS(&mut self) {

    }

    fn TYA(&mut self) {

    }

    fn XEP(&mut self) {
        panic!("undefinded opcode: {}", self.opcode);
    }
}
