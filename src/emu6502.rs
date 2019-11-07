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

const OPCODES: [Op<'static>; 112] = [
    op!(0x00, IMP, BRK, 7), op!(0x01, IDX, ORA, 6), op!(0x02, IMM, XEP, 0), op!(0x03, IMM, XEP, 0), op!(0x04, IMM, XEP, 0), op!(0x05, ZP0, ORA, 3), op!(0x06, ZP0, ASL, 5), op!(0x07, IMM, XEP, 0), op!(0x08, IMP, PHP, 3), op!(0x09, IMM, ORA, 2), op!(0x0A, ACC, ASL, 2), op!(0x0B, IMM, XEP, 0), op!(0x0C, IMM, XEP, 0), op!(0x0D, ABS, ORA, 4), op!(0x0E, ABS, ASL, 6), op!(0x0F, IMM, XEP, 0),
    op!(0x10, REL, BPL, 2), op!(0x11, IDY, ORA, 5), op!(0x12, IMM, XEP, 0), op!(0x13, IMM, XEP, 0), op!(0x14, IMM, XEP, 0), op!(0x15, ZPX, ORA, 4), op!(0x16, ZPX, ASL, 6), op!(0x17, IMM, XEP, 0), op!(0x18, IMP, CLC, 2), op!(0x19, ABY, ORA, 4), op!(0x1A, IMM, XEP, 0), op!(0x1B, IMM, XEP, 0), op!(0x1C, IMM, XEP, 0), op!(0x1D, ABX, ORA, 4), op!(0x1E, ABX, ASL, 7), op!(0x1F, IMM, XEP, 0),
    op!(0x20, ABS, JSR, 6), op!(0x21, IDX, AND, 6), op!(0x22, IMM, XEP, 0), op!(0x23, IMM, XEP, 0), op!(0x24, ZP0, BIT, 3), op!(0x25, ZP0, AND, 3), op!(0x26, ZP0, ROL, 5), op!(0x27, IMM, XEP, 0), op!(0x28, IMP, PLP, 4), op!(0x29, IMM, AND, 2), op!(0x2A, ACC, ROL, 2), op!(0x2B, IMM, XEP, 0), op!(0x2C, ABS, BIT, 4), op!(0x2D, ABS, AND, 4), op!(0x2E, ABS, ROL, 6), op!(0x2F, IMM, XEP, 0),
    op!(0x30, REL, BMI, 2), op!(0x31, IDY, AND, 5), op!(0x32, IMM, XEP, 0), op!(0x33, IMM, XEP, 0), op!(0x34, IMM, XEP, 0), op!(0x35, ZPX, AND, 4), op!(0x36, ZPX, ROL, 6), op!(0x37, IMM, XEP, 0), op!(0x38, IMP, SEC, 2), op!(0x39, ABY, AND, 4), op!(0x3A, IMM, XEP, 0), op!(0x3B, IMM, XEP, 0), op!(0x3C, IMM, XEP, 0), op!(0x3D, ABX, AND, 4), op!(0x3E, ABX, ROL, 7), op!(0x3F, IMM, XEP, 0),
    op!(0x40, IMP, RTI, 6), op!(0x41, IDX, EOR, 6), op!(0x42, IMM, XEP, 0), op!(0x43, IMM, XEP, 0), op!(0x44, IMM, XEP, 0), op!(0x45, ZP0, EOR, 3), op!(0x46, ZP0, LSR, 5), op!(0x47, IMM, XEP, 0), op!(0x48, IMP, PHA, 3), op!(0x49, IMM, EOR, 2), op!(0x4A, ACC, LSR, 2), op!(0x4B, IMM, XEP, 0), op!(0x4C, ABS, JMP, 3), op!(0x4D, ABS, EOR, 4), op!(0x4E, ABS, LSR, 6), op!(0x4F, IMM, XEP, 0),
    op!(0x50, REL, BVC, 2), op!(0x51, IDY, EOR, 5), op!(0x52, IMM, XEP, 0), op!(0x53, IMM, XEP, 0), op!(0x54, IMM, XEP, 0), op!(0x55, ZPX, EOR, 4), op!(0x56, ZPX, LSR, 6), op!(0x57, IMM, XEP, 0), op!(0x58, IMP, CLI, 2), op!(0x59, ABY, EOR, 4), op!(0x5A, IMM, XEP, 0), op!(0x5B, IMM, XEP, 0), op!(0x5C, IMM, XEP, 0), op!(0x5D, ABX, EOR, 4), op!(0x5E, ABX, LSR, 7), op!(0x5F, IMM, XEP, 0),
    op!(0x60, IMP, RTS, 6), op!(0x61, IDX, ADC, 6), op!(0x62, IMM, XEP, 0), op!(0x63, IMM, XEP, 0), op!(0x64, IMM, XEP, 0), op!(0x65, ZP0, ADC, 3), op!(0x66, ZP0, ROR, 5), op!(0x67, IMM, XEP, 0), op!(0x68, IMP, PLA, 4), op!(0x69, IMM, ADC, 2), op!(0x6A, ACC, ROR, 2), op!(0x6B, IMM, XEP, 0), op!(0x6C, IND, JMP, 5), op!(0x6D, ABS, ADC, 4), op!(0x6E, ABS, ROR, 6), op!(0x6F, IMM, XEP, 0),
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
        if self.cycle_counter <= 0 {
            self.opcode = Emu6502::read_data(self.prog_counter);
            self.prog_counter += 1;
            let op = &OPCODES[self.opcode as usize];
            (op.addressing_mode)(self);
            (op.instruction)(self);
            self.cycle_counter = op.cycle_amount;
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

    fn IDX(&mut self) {
        let base_low = Emu6502::read_data(self.prog_counter);
        self.prog_counter += 1;
        let (indirect_low, _overflow) = base_low.overflowing_add(self.x);
        let low = Emu6502::read_data(indirect_low as u16);
        let (indirect_high, _overflow) = indirect_low.overflowing_add(1);
        let high = Emu6502::read_data(indirect_high as u16);
        self.address = ((high as u16) << 8) | low as u16;
    }

    fn IDY(&mut self) {
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

    fn ADC(&mut self) {

    }

    fn AND(&mut self) {

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

    }

    fn LDX(&mut self) {

    }

    fn LDY(&mut self) {

    }

    fn LSR(&mut self) {

    }

    fn NOP(&mut self) {

    }

    fn ORA(&mut self) {

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

    }

    fn SEC(&mut self) {

    }

    fn SED(&mut self) {

    }

    fn SEI(&mut self) {

    }

    fn STA(&mut self) {

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
