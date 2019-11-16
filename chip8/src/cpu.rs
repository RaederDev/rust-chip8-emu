use crate::memory::{Memory, PROGRAM_LOAD_OFFSET};
use crate::instructions::Instruction;
use std::num::Wrapping;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Registers {
    prg_regs: [u8; 16],
    //timer
    dt: u8,
    //index
    i: u16,
    //program counter
    pc: u16,
    //stack pointer
    sp: usize,
}

#[derive(Debug)]
pub struct CPU {
    pub memory: Memory,
    pub stack: [u16; 16],
    pub registers: Registers,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            memory,
            stack: [0x0; 16],
            registers: Registers {
                prg_regs: [0x0; 16],
                dt: 0x0,
                i: 0x0,
                pc: PROGRAM_LOAD_OFFSET as u16,
                sp: 0x0,
            },
        }
    }

    pub fn fetch_current_instruction(&self) -> u32 {
        (self.memory.memory[self.registers.pc as usize] as u32) << 8
            | (self.memory.memory[self.registers.pc as usize + 1] as u32)
    }

    pub fn get_top_of_stack(&self) -> u16 {
        self.stack[self.registers.sp - 1]
    }

    pub fn write_register(&mut self, register: u32, value: u8) {
        self.registers.prg_regs[register as usize] = value;
    }

    pub fn read_register(&self, register: u32) -> u8 {
        self.registers.prg_regs[register as usize]
    }

    pub fn get_x_reg(value: u32) -> u32 {
        value >> 8 & 0x000F
    }

    pub fn get_y_reg(value: u32) -> u32 {
        value >> 4 & 0x000F
    }

    pub fn step(&mut self) {
        let (instr, value) = Instruction::decode(self.fetch_current_instruction());
        self.registers.pc += 2;
        //println!("{:?} 0x{:X}", instr, value);
        match instr {
            Instruction::SYS => {
                panic!("machine code execution not supported");
            }
            Instruction::CLS => {
                println!("Clear screen");
            }
            Instruction::RET => {
                self.registers.sp -= 1;
                self.registers.pc = self.stack[self.registers.sp];
            }
            Instruction::JP => {
                self.registers.pc = (value & 0x0FFF) as u16;
            }
            Instruction::CALL => {
                self.stack[self.registers.sp] = self.registers.pc;
                self.registers.sp += 1;
                self.registers.pc = (value & 0x0FFF) as u16;
            }
            Instruction::SE_VX_BT => {
                let content = self.read_register(CPU::get_x_reg(value));
                if content == (value & 0x00FF) as u8 {
                    self.registers.pc += 2;
                }
            }
            Instruction::SNE_VX_BT => {
                let content = self.read_register(CPU::get_x_reg(value));
                if content != (value & 0x00FF) as u8 {
                    self.registers.pc += 2;
                }
            }
            Instruction::SE_VX_VY => {
                let content_x = self.read_register(CPU::get_x_reg(value));
                let content_y = self.read_register(CPU::get_y_reg(value));
                if content_x == content_y {
                    self.registers.pc += 2;
                }
            }
            Instruction::LD_VX_BT => {
                self.write_register(CPU::get_x_reg(value), (value & 0x00FF) as u8);
            }
            Instruction::ADD_VX_BT => {
                let register = CPU::get_x_reg(value);
                let to_add = (value & 0x00FF) as u8;
                self.write_register(register, self.read_register(register) as u8 + to_add);
            }
            Instruction::LD_VX_VY => {
                let content_y = self.read_register(CPU::get_y_reg(value));
                self.write_register(CPU::get_x_reg(value), content_y);
            }
            Instruction::OR_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                self.write_register(register_x, content_x | content_y);
            }
            Instruction::AND_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                self.write_register(register_x, content_x & content_y);
            }
            Instruction::XOR_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                self.write_register(register_x, content_x ^ content_y);
            }
            Instruction::ADD_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                let result: u16 = content_x as u16 + content_y as u16;
                if result > 255 {
                    self.registers.prg_regs[0xF] = 1;
                } else {
                    self.registers.prg_regs[0xF] = 0;
                }
                self.write_register(register_x, (result & 0x000000FF) as u8);
            }
            Instruction::SUB_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                let result = Wrapping(content_x) - Wrapping(content_y);
                if content_x > content_y {
                    self.registers.prg_regs[0xF] = 1;
                } else {
                    self.registers.prg_regs[0xF] = 0;
                }
                self.write_register(register_x, result.0);
            }
            Instruction::SHR_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                self.registers.prg_regs[0xF] = content_x % 2;
                self.write_register(register_x, content_x / 2);
            }
            Instruction::SUBN_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(CPU::get_y_reg(value));
                let result = Wrapping(content_y) - Wrapping(content_x);
                if content_y > content_x {
                    self.registers.prg_regs[0xF] = 1;
                } else {
                    self.registers.prg_regs[0xF] = 0;
                }
                self.write_register(register_x, result.0);
            }
            Instruction::SHL_VX_VY => {
                let register_x = CPU::get_x_reg(value);
                let content_x = self.read_register(register_x);
                if content_x >= 0x80 { //msb = 1
                    self.registers.prg_regs[0xF] = 1;
                } else {
                    self.registers.prg_regs[0xF] = 0;
                }
                self.write_register(
                    register_x,
                    (Wrapping(content_x) + Wrapping(content_x)).0
                );
            }
            Instruction::SNE_VX_VY => {
                let content_x = self.read_register(CPU::get_x_reg(value));
                let content_y = self.read_register(CPU::get_y_reg(value));
                if content_x != content_y {
                    self.registers.pc += 2;
                }
            }
            Instruction::LD_I_ADDR => {
                self.registers.i = (value & 0x0FFF) as u16;
            }
            Instruction::JP_V0_ADDR => {
                let addr = Wrapping((value & 0x0FFF) as u16);
                self.registers.pc = (addr + Wrapping(self.registers.prg_regs[0] as u16)).0;
            }
            Instruction::RND_VX_BT => {
                let a = thread_rng().gen_range(0x00, 0x100) as u8;
            }
            _ => {
                println!("not implemented");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::memory::Memory;

    fn prepare_cpu(prg: Vec<u8>) -> CPU {
        let mut mem = Memory::new();
        mem.load_program(&prg);
        CPU::new(mem)
    }

    #[test]
    fn test_stack_flow() {
        let mut cpu = prepare_cpu(vec![
            // CALL 0x208
            0x22, //0x200
            0x04, //0x201
            // JP 0x200
            0x12, //0x202
            0x00, //0x203
            // RET to 0x200
            0x00, //0x204
            0xEE, //0x205
        ]);
        for _ in 0..2 { //ensure the CALL JMP loop actually works
            assert_eq!(0x200, cpu.registers.pc);
            cpu.step();
            assert_eq!(0x202, cpu.get_top_of_stack());
            assert_eq!(0x204, cpu.registers.pc);
            cpu.step();
            assert_eq!(0x000, cpu.registers.sp);
            assert_eq!(0x202, cpu.registers.pc);
            cpu.step();
            assert_eq!(0x200, cpu.registers.pc);
        }
    }

    #[test]
    fn test_sd_sne() {
        let mut cpu = prepare_cpu(vec![
            // SE VB, 0x22 -> should skip
            0x3B, //0x200
            0x22, //0x201
            // INVALID
            0x00, //0x202
            0x00, //0x203
            // SE VB, 0x18 -> should not skip next
            0x3B, //0x204
            0x18, //0x205
            // SNE VB, 0x22 -> should not skip next
            0x4B, //0x206
            0x22, //0x207
            // SE VD, VE -> should skip
            0x5D, //0x208
            0xE0, //0x209
            // INVALID
            0x00, //0x20A
            0x00, //0x20B
            // SE VB, VD -> should not skip
            0x5B, //0x20C
            0xD0, //0x20D
            // SNE VB, VD -> skip
            0x9B, //0x20E
            0xD0, //0x20F
            // INVALID
            0x00, //0x210
            0x00, //0x211
            // JP 0x200
            0x12, //0x212
            0x00, //0x213
        ]);
        cpu.registers.prg_regs[0xB] = 0x22;
        cpu.registers.prg_regs[0xD] = 0x11;
        cpu.registers.prg_regs[0xE] = 0x11;
        cpu.step();
        assert_eq!(0x204, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x206, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x208, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x20C, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x20E, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x212, cpu.registers.pc);
        cpu.step();
        assert_eq!(0x200, cpu.registers.pc);
    }

    #[test]
    fn test_load_add_instr() {
        let mut cpu = prepare_cpu(vec![
            // LD A, 0xFF
            0x6A, //0x200
            0xFF, //0x201
            // LD A, 0xAF
            0x6A, //0x202
            0xAF, //0x204
            // ADD A, 0x01
            0x7A, //0x205
            0x01, //0x206
            // LD B, A
            0x8B, //0x207
            0xA0, //0x208
            // LD, I, 0x3AB
            0xA3, //0x209
            0xAB, //0x20A
            // LD 0, 0x10
            0x60, //0x20B
            0x10, //0x20C
            // JP V0, 0x1F0
            0xB1, //0x20D
            0xF0, //0x20E
        ]);

        cpu.step();
        assert_eq!(0xFF, cpu.registers.prg_regs[0xA]);
        cpu.step();
        assert_eq!(0xAF, cpu.registers.prg_regs[0xA]);
        cpu.step();
        assert_eq!(0xB0, cpu.registers.prg_regs[0xA]);
        assert_ne!(0xB0, cpu.registers.prg_regs[0xB]);
        cpu.step();
        assert_eq!(0xB0, cpu.registers.prg_regs[0xB]);
        cpu.step();
        assert_eq!(0x3AB, cpu.registers.i);
        cpu.step();
        assert_eq!(0x10, cpu.registers.prg_regs[0x0]);
        cpu.step();
        assert_eq!(0x200, cpu.registers.pc);
    }

    #[test]
    fn test_math() {
        //xor, add, sub
        let mut cpu = prepare_cpu(vec![
            // OR A, B
            0x8A, //0x200
            0xB1, //0x201
            // LD A, 0xF0
            0x6A, //0x202
            0xF0, //0x204
            // AND A, B
            0x8A, //0x202
            0xB2, //0x203
            // XOR A, C
            0x8A, //0x204
            0xC3, //0x205
            // ADD D, E
            0x8D, //0x206
            0xE4, //0x207
            // ADD D, E
            0x8D, //0x208
            0xE4, //0x209
            // LD D, 0xFF
            0x6D, //0x20A
            0xFF, //0x20B
            // SUB D, E
            0x8D, //0x20C
            0xE5, //0x20D
            // SUB 0, 1
            0x80, //0x20E
            0x15, //0x20F
        ]);
        cpu.registers.prg_regs[0x0] = 0x01;
        cpu.registers.prg_regs[0x1] = 0x03;
        cpu.registers.prg_regs[0xA] = 0x02;
        cpu.registers.prg_regs[0xB] = 0xA1;
        cpu.registers.prg_regs[0xC] = 0x02;
        cpu.registers.prg_regs[0xD] = 0xFE;
        cpu.registers.prg_regs[0xE] = 0x01;

        cpu.step();
        assert_eq!(0xA3, cpu.registers.prg_regs[0xA]);
        cpu.step();
        cpu.step();
        assert_eq!(0xA0, cpu.registers.prg_regs[0xA]);
        cpu.step();
        assert_eq!(0xA2, cpu.registers.prg_regs[0xA]);
        cpu.step();
        assert_eq!(0xFF, cpu.registers.prg_regs[0xD]);
        assert_eq!(0x0, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0x00, cpu.registers.prg_regs[0xD]);
        assert_eq!(0x1, cpu.registers.prg_regs[0xF]);
        cpu.step();
        cpu.step();
        assert_eq!(0xFE, cpu.registers.prg_regs[0xD]);
        assert_eq!(0x01, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0xFE, cpu.registers.prg_regs[0x0]);
        assert_eq!(0x00, cpu.registers.prg_regs[0xF]);
    }

    #[test]
    fn test_additional_math() {
        //xor, add, sub
        let mut cpu = prepare_cpu(vec![
            // SHR A, {_}
            0x8A, //0x200
            0xB6, //0x201
            // SHR A, {_}
            0x8A, //0x202
            0xB6, //0x203
            // SUBN A, B
            0x8A, //0x204
            0xB7, //0x205
            // SUBN B, A
            0x8B, //0x206
            0xA7, //0x207
            // SHL C, {_}
            0x8C, //0x206
            0xAE, //0x207
            // SHL D, {_}
            0x8D, //0x208
            0xAE, //0x209
        ]);
        cpu.registers.prg_regs[0xA] = 0x0A;
        cpu.registers.prg_regs[0xB] = 0x01;
        cpu.registers.prg_regs[0xC] = 0x05;
        cpu.registers.prg_regs[0xD] = 0xA0;

        cpu.step();
        assert_eq!(0x05, cpu.registers.prg_regs[0xA]);
        assert_eq!(0x00, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0x02, cpu.registers.prg_regs[0xA]);
        assert_eq!(0x01, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0xFF, cpu.registers.prg_regs[0xA]);
        assert_eq!(0x00, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0xFE, cpu.registers.prg_regs[0xB]);
        assert_eq!(0x01, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0x0A, cpu.registers.prg_regs[0xC]);
        assert_eq!(0x00, cpu.registers.prg_regs[0xF]);
        cpu.step();
        assert_eq!(0x40, cpu.registers.prg_regs[0xD]);
        assert_eq!(0x01, cpu.registers.prg_regs[0xF]);
    }
}
