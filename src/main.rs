use emu::emu6502::Emu6502;

fn main() {
    let mut emu6502 = Emu6502::new();
    emu6502.clock();
    println!("Hello, world!");
}
