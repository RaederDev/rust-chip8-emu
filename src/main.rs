use chip8::{cpu::CPU, memory::Memory};

fn main() {
    let mem = Memory::new();
    let cpu = CPU::new(mem);
    // println!("{}", cpu.memory);
    // cpu.exec(Instruction::In0nnn);
    let instr = [0x12, 0x4E]; //JP to 0x024E each instruction has 2 bytes
    let decoded = CPU::decode(instr[0] << 8 | instr[1]);
    println!("{:?}", decoded);
}
