use std::rc::Rc;
use std::cell::RefCell;
use super::bus::Bus;

macro_rules! op {
    ($ind: literal, $addr:ident, $instr:ident, $amount: expr) => {
        Op { addressing_mode_name: stringify!($addr), addressing_mode: &Emu6502::$addr,
             instruction_name: stringify!($instr), instruction: &Emu6502::$instr,
             cycle_amount: $amount }
    };
}

struct Op<'a> {
    addressing_mode_name: &'a str,
    addressing_mode: &'a dyn Fn(&mut Emu6502),
    instruction_name: &'a str,
    instruction: &'a dyn Fn(&mut Emu6502),
    cycle_amount: u8
}

const OPCODES: [Op<'static>; 256] = [
    op!(0x00, IMP, BRK, 7), op!(0x01, IDX, ORA, 6), op!(0x02, IMP, XEP, 0), op!(0x03, IMP, XEP, 0), op!(0x04, IMP, XEP, 0), op!(0x05, ZP0, ORA, 3), op!(0x06, ZP0, ASL, 5), op!(0x07, IMP, XEP, 0), op!(0x08, IMP, PHP, 3), op!(0x09, IMM, ORA, 2), op!(0x0A, ACC, ASL, 2), op!(0x0B, IMP, XEP, 0), op!(0x0C, IMP, XEP, 0), op!(0x0D, ABS, ORA, 4), op!(0x0E, ABS, ASL, 6), op!(0x0F, IMP, XEP, 0),
    op!(0x10, REL, BPL, 2), op!(0x11, IDY, ORA, 5), op!(0x12, IMP, XEP, 0), op!(0x13, IMP, XEP, 0), op!(0x14, IMP, XEP, 0), op!(0x15, ZPX, ORA, 4), op!(0x16, ZPX, ASL, 6), op!(0x17, IMP, XEP, 0), op!(0x18, IMP, CLC, 2), op!(0x19, ABY, ORA, 4), op!(0x1A, IMP, XEP, 0), op!(0x1B, IMP, XEP, 0), op!(0x1C, IMP, XEP, 0), op!(0x1D, ABX, ORA, 4), op!(0x1E, ABX, ASL, 7), op!(0x1F, IMP, XEP, 0),
    op!(0x20, ABS, JSR, 6), op!(0x21, IDX, AND, 6), op!(0x22, IMP, XEP, 0), op!(0x23, IMP, XEP, 0), op!(0x24, ZP0, BIT, 3), op!(0x25, ZP0, AND, 3), op!(0x26, ZP0, ROL, 5), op!(0x27, IMP, XEP, 0), op!(0x28, IMP, PLP, 4), op!(0x29, IMM, AND, 2), op!(0x2A, ACC, ROL, 2), op!(0x2B, IMP, XEP, 0), op!(0x2C, ABS, BIT, 4), op!(0x2D, ABS, AND, 4), op!(0x2E, ABS, ROL, 6), op!(0x2F, IMP, XEP, 0),
    op!(0x30, REL, BMI, 2), op!(0x31, IDY, AND, 5), op!(0x32, IMP, XEP, 0), op!(0x33, IMP, XEP, 0), op!(0x34, IMP, XEP, 0), op!(0x35, ZPX, AND, 4), op!(0x36, ZPX, ROL, 6), op!(0x37, IMP, XEP, 0), op!(0x38, IMP, SEC, 2), op!(0x39, ABY, AND, 4), op!(0x3A, IMP, XEP, 0), op!(0x3B, IMP, XEP, 0), op!(0x3C, IMP, XEP, 0), op!(0x3D, ABX, AND, 4), op!(0x3E, ABX, ROL, 7), op!(0x3F, IMP, XEP, 0),
    op!(0x40, IMP, RTI, 6), op!(0x41, IDX, EOR, 6), op!(0x42, IMP, XEP, 0), op!(0x43, IMP, XEP, 0), op!(0x44, IMP, XEP, 0), op!(0x45, ZP0, EOR, 3), op!(0x46, ZP0, LSR, 5), op!(0x47, IMP, XEP, 0), op!(0x48, IMP, PHA, 3), op!(0x49, IMM, EOR, 2), op!(0x4A, ACC, LSR, 2), op!(0x4B, IMP, XEP, 0), op!(0x4C, ABS, JMP, 3), op!(0x4D, ABS, EOR, 4), op!(0x4E, ABS, LSR, 6), op!(0x4F, IMP, XEP, 0),
    op!(0x50, REL, BVC, 2), op!(0x51, IDY, EOR, 5), op!(0x52, IMP, XEP, 0), op!(0x53, IMP, XEP, 0), op!(0x54, IMP, XEP, 0), op!(0x55, ZPX, EOR, 4), op!(0x56, ZPX, LSR, 6), op!(0x57, IMP, XEP, 0), op!(0x58, IMP, CLI, 2), op!(0x59, ABY, EOR, 4), op!(0x5A, IMP, XEP, 0), op!(0x5B, IMP, XEP, 0), op!(0x5C, IMP, XEP, 0), op!(0x5D, ABX, EOR, 4), op!(0x5E, ABX, LSR, 7), op!(0x5F, IMP, XEP, 0),
    op!(0x60, IMP, RTS, 6), op!(0x61, IDX, ADC, 6), op!(0x62, IMP, XEP, 0), op!(0x63, IMP, XEP, 0), op!(0x64, IMP, XEP, 0), op!(0x65, ZP0, ADC, 3), op!(0x66, ZP0, ROR, 5), op!(0x67, IMP, XEP, 0), op!(0x68, IMP, PLA, 4), op!(0x69, IMM, ADC, 2), op!(0x6A, ACC, ROR, 2), op!(0x6B, IMP, XEP, 0), op!(0x6C, IND, JMP, 5), op!(0x6D, ABS, ADC, 4), op!(0x6E, ABS, ROR, 6), op!(0x6F, IMP, XEP, 0),
    op!(0x70, REL, BVS, 2), op!(0x71, IDY, ADC, 5), op!(0x72, IMP, XEP, 0), op!(0x73, IMP, XEP, 0), op!(0x74, IMP, XEP, 0), op!(0x75, ZPX, ADC, 4), op!(0x76, ZPX, ROR, 6), op!(0x77, IMP, XEP, 0), op!(0x78, IMP, SEI, 2), op!(0x79, ABY, ADC, 4), op!(0x7A, IMP, XEP, 0), op!(0x7B, IMP, XEP, 0), op!(0x7C, IMP, XEP, 0), op!(0x7D, ABX, ADC, 4), op!(0x7E, ABX, ROR, 7), op!(0x7F, IMP, XEP, 0),
    op!(0x80, IMP, XEP, 0), op!(0x81, IDX, STA, 6), op!(0x82, IMP, XEP, 0), op!(0x83, IMP, XEP, 0), op!(0x84, ZP0, STY, 3), op!(0x85, ZP0, STA, 3), op!(0x86, ZP0, STX, 3), op!(0x87, IMP, XEP, 0), op!(0x88, IMP, DEY, 2), op!(0x89, IMP, XEP, 0), op!(0x8A, IMP, TXA, 2), op!(0x8B, IMP, XEP, 0), op!(0x8C, ABS, STY, 4), op!(0x8D, ABS, STA, 4), op!(0x8E, ABS, STX, 4), op!(0x8F, IMP, XEP, 0),
    op!(0x90, REL, BCC, 2), op!(0x91, IDY, STA, 6), op!(0x92, IMP, XEP, 0), op!(0x93, IMP, XEP, 0), op!(0x94, ZPX, STY, 4), op!(0x95, ZPX, STA, 4), op!(0x96, ZPY, STX, 4), op!(0x97, IMP, XEP, 0), op!(0x98, IMP, TYA, 2), op!(0x99, ABY, STA, 5), op!(0x9A, IMP, TXS, 2), op!(0x9B, IMP, XEP, 0), op!(0x9C, IMP, XEP, 0), op!(0x9D, ABX, STA, 5), op!(0x9E, IMP, XEP, 0), op!(0x9F, IMP, XEP, 0),
    op!(0xA0, IMM, LDY, 2), op!(0xA1, IDX, LDA, 6), op!(0xA2, IMM, LDX, 2), op!(0xA3, IMP, XEP, 0), op!(0xA4, ZP0, LDY, 3), op!(0xA5, ZP0, LDA, 3), op!(0xA6, ZP0, LDX, 3), op!(0xA7, IMP, XEP, 0), op!(0xA8, IMP, TAY, 2), op!(0xA9, IMM, LDA, 2), op!(0xAA, IMP, TAX, 2), op!(0xAB, IMP, XEP, 0), op!(0xAC, ABS, LDY, 4), op!(0xAD, ABS, LDA, 4), op!(0xAE, ABS, LDX, 4), op!(0xAF, IMP, XEP, 0),
    op!(0xB0, REL, BCS, 2), op!(0xB1, IDY, LDA, 5), op!(0xB2, IMP, XEP, 0), op!(0xB3, IMP, XEP, 0), op!(0xB4, ZPX, LDY, 4), op!(0xB5, ZPX, LDA, 4), op!(0xB6, ZPY, LDX, 4), op!(0xB7, IMP, XEP, 0), op!(0xB8, IMP, CLV, 2), op!(0xB9, ABY, LDA, 4), op!(0xBA, IMP, TSX, 2), op!(0xBB, IMP, XEP, 0), op!(0xBC, ABX, LDY, 4), op!(0xBD, ABX, LDA, 4), op!(0xBE, ABY, LDX, 4), op!(0xBF, IMP, XEP, 0),
    op!(0xC0, IMM, CPY, 2), op!(0xC1, IDX, CMP, 6), op!(0xC2, IMP, XEP, 0), op!(0xC3, IMP, XEP, 0), op!(0xC4, ZP0, CPY, 3), op!(0xC5, ZP0, CMP, 3), op!(0xC6, ZP0, DEC, 5), op!(0xC7, IMP, XEP, 0), op!(0xC8, IMP, INY, 2), op!(0xC9, IMM, CMP, 2), op!(0xCA, IMP, DEX, 2), op!(0xCB, IMP, XEP, 0), op!(0xCC, ABS, CPY, 4), op!(0xCD, ABS, CMP, 4), op!(0xCE, ABS, DEC, 6), op!(0xCF, IMP, XEP, 0),
    op!(0xD0, REL, BNE, 2), op!(0xD1, IDY, CMP, 5), op!(0xD2, IMP, XEP, 0), op!(0xD3, IMP, XEP, 0), op!(0xD4, IMP, XEP, 0), op!(0xD5, ZPX, CMP, 4), op!(0xD6, ZPX, DEC, 6), op!(0xD7, IMP, XEP, 0), op!(0xD8, IMP, CLD, 2), op!(0xD9, ABY, CMP, 4), op!(0xDA, IMP, XEP, 0), op!(0xDB, IMP, XEP, 0), op!(0xDC, IMP, XEP, 0), op!(0xDD, ABX, CMP, 4), op!(0xDE, ABX, DEC, 7), op!(0xDF, IMP, XEP, 0),
    op!(0xE0, IMM, CPX, 2), op!(0xE1, IDX, SBC, 6), op!(0xE2, IMP, XEP, 0), op!(0xE3, IMP, XEP, 0), op!(0xE4, ZP0, CPX, 3), op!(0xE5, ZP0, SBC, 3), op!(0xE6, ZP0, INC, 5), op!(0xE7, IMP, XEP, 0), op!(0xE8, IMP, INX, 2), op!(0xE9, IMM, SBC, 2), op!(0xEA, IMP, NOP, 2), op!(0xEB, IMP, XEP, 0), op!(0xEC, ABS, CPX, 4), op!(0xED, ABS, SBC, 4), op!(0xEE, ABS, INC, 6), op!(0xEF, IMP, XEP, 0),
    op!(0xF0, REL, BEQ, 2), op!(0xF1, IDY, SBC, 5), op!(0xF2, IMP, XEP, 0), op!(0xF3, IMP, XEP, 0), op!(0xF4, IMP, XEP, 0), op!(0xF5, ZPX, SBC, 4), op!(0xF6, ZPX, INC, 6), op!(0xF7, IMP, XEP, 0), op!(0xF8, IMP, SED, 2), op!(0xF9, ABY, SBC, 4), op!(0xFA, IMP, XEP, 0), op!(0xFB, IMP, XEP, 0), op!(0xFC, IMP, XEP, 0), op!(0xFD, ABX, SBC, 4), op!(0xFE, ABX, INC, 7), op!(0xFF, IMP, XEP, 0)
];

enum Flag {
    C = 1 << 0, // Cary
    Z = 1 << 1, // Zero
    I = 1 << 2, // Interrupt Disable
    D = 1 << 3, // Decimal
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    S = 1 << 7  // Sign
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

    bus: Rc<RefCell<Bus>>,
    pub clock_complete: bool,
    pub debug: bool,
}


#[allow(non_snake_case)]
impl Emu6502 {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Emu6502 {
        Emu6502 {
            acc: 0,
            x: 0,
            y: 0,

            status: 0x20,
            stack_ptr: 0xFD,
            prog_counter: 0,

            address: 0,
            addr_offset: 0,
            fetched_data: 0,

            opcode: 0,
            cycle_counter: 0,
            additional_cycles: 0,

            bus: bus.clone(),
            clock_complete: false,
            debug: false
        }
    }

    pub fn set_programm_counter(&mut self, address: u16) {
        self.prog_counter = address;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.prog_counter
    }

    pub fn clock(&mut self) {
        if self.cycle_counter == 0 {
            self.additional_cycles = 0;
            self.opcode = self.read_data(self.prog_counter);
            let op = &OPCODES[self.opcode as usize];

            if self.debug {
                print!(
                    "a={:02X} x={:02X} y={:02X} st={:08b}({:02X}) pc={:04X} st_ptr={:02X}\t| opcode {:02X}: {} {} ",
                    self.acc, self.x, self.y, self.status, self.status, self.prog_counter, self.stack_ptr,
                    self.opcode, op.instruction_name, op.addressing_mode_name
                );
            }

            self.prog_counter += 1;
            (op.addressing_mode)(self);

            if self.debug {
                if op.instruction_name.chars().next().unwrap() == 'B'
                    && op.instruction_name != "BIT"
                    && op.instruction_name != "BRK"
                {
                    print!("{} ", self.addr_offset as i16);
                } else if op.addressing_mode_name == "ACC" || op.addressing_mode_name == "IMP" {
                    println!("");
                } else {
                    println!("{:04X} = {:02X}", self.address, self.bus.borrow().read_only_data(self.address));
                }
            }

            (op.instruction)(self);
            self.cycle_counter = op.cycle_amount;
        } else {
            self.clock_complete = false;
        }
        self.cycle_counter -= 1;
        if self.cycle_counter == 0 {
            self.clock_complete = true;
        }
    }

    pub fn irq(&mut self) {
        if self.get_flag(Flag::I) == 0 {
            let low = self.prog_counter as u8;
            let high = (self.prog_counter >> 8) as u8;
            self.push_to_stack(high);
            self.push_to_stack(low);
            self.set_flag(Flag::B, false);
            self.set_flag(Flag::I, true);
            self.set_flag(Flag::U, true);
            self.push_to_stack(self.status);
            let new_low = self.read_data(0xFFFE);
            let new_high = self.read_data(0xFFFF);
            self.prog_counter = ((new_high as u16) << 8) | new_low as u16;
            self.cycle_counter = 7;
        }
    }

    pub fn nmi(&mut self) { // non-maskable interrupt
        let low = self.prog_counter as u8;
        let high = (self.prog_counter >> 8) as u8;
        self.push_to_stack(high);
        self.push_to_stack(low);
        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, true);
        self.set_flag(Flag::I, true);
        self.push_to_stack(self.status);
        let new_low = self.read_data(0xFFFA);
        let new_high = self.read_data(0xFFFB);
        self.prog_counter = ((new_high as u16) << 8) | new_low as u16;
        self.cycle_counter = 8;
        if self.debug {
            println!("cpu: nmi executing, new prog_counter: {:04X}", self.prog_counter);
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.x = 0;
        self.y = 0;
        self.stack_ptr = 0xFD;
        self.status = 0x24;
        let low = self.read_data(0xFFFC);
        let high = self.read_data(0xFFFD);
        self.prog_counter = ((high as u16) << 8) | low as u16;
    }

    fn set_flag(&mut self, flag: Flag, state: bool) {
        match state {
            true  => self.status |= flag as u8,
            false => self.status &= !(flag as u8),
        }
    }

    fn get_flag(&self, flag: Flag) -> u8 {
        match (self.status & flag as u8) != 0 {
            true  => 1,
            false => 0
        }
    }

    fn push_to_stack(&mut self, data: u8) {
        let stack_address = 0x0100 | self.stack_ptr as u16;
        self.bus.borrow_mut().write_cpu_ram(stack_address, data);
        self.stack_ptr -= 1;
    }

    fn pop_from_stack(&mut self) -> u8 {
        self.stack_ptr += 1;
        let stack_address = 0x0100 | self.stack_ptr as u16;
        self.read_data(stack_address)
    }

    fn read_data(&self, address: u16) -> u8 {
        self.bus.borrow_mut().read_cpu_ram(address)
    }

    fn write_data(&self, data: u8) {
        self.bus.borrow_mut().write_cpu_ram(self.address, data);
    }

    fn fetch(&mut self) -> u8 {
        let addressing_mode_name = &OPCODES[self.opcode as usize].addressing_mode_name;
        if addressing_mode_name.trim() != "ACC" &&
            addressing_mode_name.trim() != "IMP" {
            self.fetched_data = self.read_data(self.address);
        }
        self.fetched_data
    }

    fn branching_instruction(&mut self) {
        if self.debug {
            print!("branching operation processed");
        }
        self.cycle_counter += 1;
        let new_prog_counter = self.prog_counter.overflowing_add(self.addr_offset).0;
        if (new_prog_counter & 0xFF00) != (self.prog_counter & 0xFF00) {
            self.cycle_counter += 1;
        }
        self.prog_counter = new_prog_counter;
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
        self.address = base_address.overflowing_add(self.x as u16).0;
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
        self.address = base_address.overflowing_add(self.y as u16).0;
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
        self.address = base_low.overflowing_add(self.x).0 as u16;
    }

    fn ZPY(&mut self) {
        let base_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        self.address = base_low.overflowing_add(self.y).0 as u16;
    }

    fn IND(&mut self) {
        let indirect_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_high = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let address_low_byte = ((indirect_high as u16) << 8) | indirect_low as u16;
        let address_high_byte = (address_low_byte & 0xFF00) | (address_low_byte + 1) & 0x00FF;
        let low = self.read_data(address_low_byte);
        let high = self.read_data(address_high_byte);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn IDX(&mut self) {
        let base_low = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let indirect_low = base_low.overflowing_add(self.x).0;
        let indirect_high = indirect_low.overflowing_add(1).0;
        let low = self.read_data(indirect_low as u16);
        let high = self.read_data(indirect_high as u16);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn IDY(&mut self) {
        let zero_page_addr = self.read_data(self.prog_counter);
        self.prog_counter += 1;
        let next_byte = zero_page_addr.overflowing_add(1).0;
        let low = self.read_data(zero_page_addr as u16);
        let high = self.read_data(next_byte as u16);
        let result_address = ((high as u16) << 8) | low as u16;
        self.address = result_address.overflowing_add(self.y as u16).0;
        if (self.address & 0xFF00) != ((high as u16) << 8) {
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

    fn LDA(&mut self) { // load data to accumulator
        self.cycle_counter += self.additional_cycles;
        self.acc = self.fetch();
        self.set_flag(Flag::Z, self.acc == 0x0000);
        self.set_flag(Flag::S, self.acc & 0x80 != 0)
    }

    fn STA(&mut self) { // store accumulator to memory
        self.write_data(self.acc);
    }

    fn ADC(&mut self) { // add with carry
        self.cycle_counter += self.additional_cycles;
        let (add_value, overflow_carry) = self.fetch().overflowing_add(self.get_flag(Flag::C));
        let (result, overflow) = self.acc.overflowing_add(add_value);
        self.set_flag(Flag::C, overflow || overflow_carry);
        self.set_flag(Flag::V, (self.acc & 0x80 == self.fetched_data & 0x80) && (self.acc & 0x80 != result & 0x80));
        self.set_flag(Flag::S, result & 0x80 != 0);
        self.set_flag(Flag::Z, result == 0x0000);
        self.acc = result;
    }

    fn SBC(&mut self) { // subtract with carry
        self.cycle_counter += self.additional_cycles;
        self.fetch();
        let (operand, overflow1) = (!self.fetched_data).overflowing_add(self.get_flag(Flag::C));
        let (result, overflow) = self.acc.overflowing_add(operand);
        self.set_flag(Flag::C, overflow || overflow1);
        self.set_flag(Flag::V, (self.acc & 0x80 == operand & 0x80) && (self.acc & 0x80 != result & 0x80));
        self.set_flag(Flag::S, result & 0x80 != 0);
        self.set_flag(Flag::Z, result == 0);
        self.acc = result;
    }

    fn AND(&mut self) { // bitwise and
        self.cycle_counter += self.additional_cycles;
        self.acc = self.acc & self.fetch();
        self.set_flag(Flag::Z, self.acc == 0);
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
    }

    fn ORA(&mut self) { // bitwise or
        self.cycle_counter += self.additional_cycles;
        self.acc = self.acc | self.fetch();
        self.set_flag(Flag::Z, self.acc == 0);
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
    }

    fn EOR(&mut self) { // bitwise xor
        self.cycle_counter += self.additional_cycles;
        self.acc = self.acc ^ self.fetch();
        self.set_flag(Flag::Z, self.acc == 0);
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
    }

    fn SEC(&mut self) { // set carry flag
        self.set_flag(Flag::C, true);
    }

    fn CLC(&mut self) { // reset carry flag
        self.set_flag(Flag::C, false);
    }

    fn SEI(&mut self) { // set interrupt disable flag
        self.set_flag(Flag::I, true);
    }

    fn CLI(&mut self) { // reset interrupt disable flag
        self.set_flag(Flag::I, false);
    }

    fn SED(&mut self) { // set decimal mode
        self.set_flag(Flag::D, true);
    }

    fn CLD(&mut self) { // reset decimal mode
        self.set_flag(Flag::D, false);
    }

    fn CLV(&mut self) { // reset overflow flag
        self.set_flag(Flag::V, false);
    }

    fn JMP(&mut self) {
        self.prog_counter = self.address;
    }

    fn BMI(&mut self) { // branch if minus
        if self.get_flag(Flag::S) == 1 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BPL(&mut self) { // branch if plus
        if self.get_flag(Flag::S) == 0 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BCC(&mut self) { // branch if carry reset
        if self.get_flag(Flag::C) == 0 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BCS(&mut self) { // branch if carry set
        if self.get_flag(Flag::C) == 1 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BEQ(&mut self) { // branch if zero
        if self.get_flag(Flag::Z) == 1 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BNE(&mut self) { // branch if not zero
        if self.get_flag(Flag::Z) == 0 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BVS(&mut self) { // branch if overflow set
        if self.get_flag(Flag::V) == 1 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn BVC(&mut self) { // branch if overflow reset
        if self.get_flag(Flag::V) == 0 {
            self.branching_instruction();
        }
        if self.debug {
            println!("");
        }
    }

    fn CMP(&mut self) { // compare accumulator to memory
        self.cycle_counter += self.additional_cycles;
        self.fetch();
        let invert_fetched_data = (!self.fetched_data).overflowing_add(1).0;
        let result = self.acc.overflowing_add(invert_fetched_data).0;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, result & 0x80 != 0);
        self.set_flag(Flag::C, self.acc >= self.fetched_data);
    }

    fn BIT(&mut self) {
        self.fetch();
        let result = self.acc & self.fetched_data;
        self.set_flag(Flag::S, self.fetched_data & 0x80 != 0);
        self.set_flag(Flag::V, self.fetched_data & (1 << 6) != 0);
        self.set_flag(Flag::Z, result == 0);
    }

    fn LDX(&mut self) { // load memory to x register
        self.cycle_counter += self.additional_cycles;
        self.x = self.fetch();
        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::S, self.x & 0x80 != 0);
    }

    fn LDY(&mut self) { // load memory to y register
        self.cycle_counter += self.additional_cycles;
        self.y = self.fetch();
        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::S, self.y & 0x80 != 0);
    }

    fn STX(&mut self) { // store x register to memory
        self.write_data(self.x);
    }

    fn STY(&mut self) {  // store y register to memory
        self.write_data(self.y);
    }

    fn INX(&mut self) { // increment x register
        self.x = self.x.overflowing_add(1).0;
        self.set_flag(Flag::S, self.x & 0x80 != 0);
        self.set_flag(Flag::Z, self.x == 0);
    }

    fn INY(&mut self) { // increment y register
        self.y = self.y.overflowing_add(1).0;
        self.set_flag(Flag::S, self.y & 0x80 != 0);
        self.set_flag(Flag::Z, self.y == 0);
    }

    fn DEX(&mut self) { // decrement x register
        self.x = self.x.overflowing_sub(1).0;
        self.set_flag(Flag::S, self.x & 0x80 != 0);
        self.set_flag(Flag::Z, self.x == 0);
    }

    fn DEY(&mut self) { // decrement y register
        self.y = self.y.overflowing_sub(1).0;
        self.set_flag(Flag::S, self.y & 0x80 != 0);
        self.set_flag(Flag::Z, self.y == 0);
    }

    fn CPX(&mut self) { // compare x to memory
        self.fetch();
        let invert_fetched_data = (!self.fetched_data).overflowing_add(1).0;
        let result = self.x.overflowing_add(invert_fetched_data).0;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, result & 0x80 != 0);
        self.set_flag(Flag::C, self.x >= self.fetched_data);
    }

    fn CPY(&mut self) { // compare y to memory
        self.fetch();
        let invert_fetched_data = (!self.fetched_data).overflowing_add(1).0;
        let result = self.y.overflowing_add(invert_fetched_data).0;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, result & 0x80 != 0);
        self.set_flag(Flag::C, self.y >= self.fetched_data);
    }

    fn TAX(&mut self) { // transfer accumulator to x
        self.x = self.acc;
        self.set_flag(Flag::S, self.x & 0x80 != 0);
        self.set_flag(Flag::Z, self.x == 0);
    }

    fn TXA(&mut self) { // transfer x to accumulator
        self.acc = self.x;
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
        self.set_flag(Flag::Z, self.acc == 0);
    }

    fn TAY(&mut self) { // transfer accumulator to y
        self.y = self.acc;
        self.set_flag(Flag::S, self.y & 0x80 != 0);
        self.set_flag(Flag::Z, self.y == 0);
    }

    fn TYA(&mut self) { // transfer y to accumulator
        self.acc = self.y;
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
        self.set_flag(Flag::Z, self.acc == 0);
    }

    fn JSR(&mut self) { // jump to subroutine
        self.prog_counter -= 1;
        let low = self.prog_counter as u8;
        let high = (self.prog_counter >> 8) as u8;
        self.push_to_stack(high);
        self.push_to_stack(low);
        self.prog_counter = self.address;
    }

    fn RTS(&mut self) { // return from subroutin
        let low = self.pop_from_stack();
        let high = self.pop_from_stack();
        self.prog_counter = ((high as u16) << 8) | low as u16;
        self.prog_counter += 1;
    }

    fn PHA(&mut self) { // push accumulator on stack
        self.push_to_stack(self.acc);
    }

    fn PLA(&mut self) { // pop accumulator from stack
        self.acc = self.pop_from_stack();
        self.set_flag(Flag::S, self.acc & 0x80 != 0);
        self.set_flag(Flag::Z, self.acc == 0);
    }

    fn TXS(&mut self) { // transfer x register to stack pointer
        self.stack_ptr = self.x;
    }

    fn TSX(&mut self) { // transfer stack pointer to x register
        self.x = self.stack_ptr;
        self.set_flag(Flag::S, self.x & 0x80 != 0);
        self.set_flag(Flag::Z, self.x == 0);
    }

    fn PHP(&mut self) { // push status register to stack
        self.push_to_stack(self.status);
    }

    fn PLP(&mut self) { // pop status register from stack
        self.status = self.pop_from_stack();
        self.set_flag(Flag::U, true);
    }

    fn LSR(&mut self) { // logical shift right
        self.fetch();
        let poped_bit = self.fetched_data & 0x01;
        let result = self.fetched_data >> 1;
        match self.opcode {
            0x4A => self.acc = result,
            _ => self.write_data(result),
        };
        self.set_flag(Flag::C, poped_bit == 1);
        self.set_flag(Flag::S, false);
        self.set_flag(Flag::Z, result == 0);
    }

    fn ASL(&mut self) { // arithmetic shift left
        self.fetch();
        let poped_bit = self.fetched_data >> 7;
        let result = self.fetched_data << 1;
        match self.opcode {
            0x0A => self.acc = result,
            _ => self.write_data(result),
        };
        self.set_flag(Flag::C, poped_bit == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, result & 0x80 != 0);
    }

    fn ROL(&mut self) { // rotate left
        self.fetch();
        let carry_value = self.get_flag(Flag::C);
        let result = (self.fetched_data << 1) | carry_value;
        match self.opcode {
            0x2A => self.acc = result,
            _ => self.write_data(result),
        };
        self.set_flag(Flag::C, self.fetched_data & 0x80 != 0);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, (result & 0x80) != 0);
    }

    fn ROR(&mut self) { // rotate right
        self.fetch();
        let carry_value = self.get_flag(Flag::C);
        let result = (self.fetched_data >> 1) | (carry_value << 7);
        match self.opcode {
            0x6A => self.acc = result,
            _ => self.write_data(result),
        };
        self.set_flag(Flag::C, self.fetched_data & 0x01 != 0);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, (result & 0x80) != 0);
    }

    fn INC(&mut self) { // increment memory by one
        let result = self.fetch().overflowing_add(1).0;
        self.write_data(result);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, (result & 0x80) != 0);
    }

    fn DEC(&mut self) { // decrement memory by one
        let result = self.fetch().overflowing_sub(1).0;
        self.write_data(result);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::S, (result & 0x80) != 0);
    }

    fn RTI(&mut self) { // return from interrupt
        self.status = self.pop_from_stack();
        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, true);
        let low = self.pop_from_stack();
        let high = self.pop_from_stack();
        self.prog_counter = ((high as u16) << 8) | low as u16;
    }

    fn BRK(&mut self) { // break
        let low = self.prog_counter as u8;
        let high = (self.prog_counter >> 8) as u8;
        self.push_to_stack(high);
        self.push_to_stack(low);
        self.set_flag(Flag::U, true);
        self.set_flag(Flag::I, true);
        self.set_flag(Flag::B, true);
        self.push_to_stack(self.status);
        let new_low = self.read_data(0xFFFE);
        let new_high = self.read_data(0xFFFF);
        self.prog_counter = ((new_high as u16) << 8) | new_low as u16;
    }

    fn NOP(&mut self) {}

    fn XEP(&mut self) {
        eprintln!("undefinded opcode: {:02X}", self.opcode);
    }
}
