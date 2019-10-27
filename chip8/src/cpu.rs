use crate::memory::{Memory, PROGRAM_LOAD_OFFSET};
use crate::instructions::Instruction;

#[derive(Debug)]
pub struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    dt: u8, //timer
    i: u16, //index
    pc: u16, //program counter
    sp: usize, //stack pointer
}

#[derive(Debug)]
pub struct CPU {
    pub memory: Memory,
    pub stack: [u16; 16],
    pub registers: Registers,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        let mut cpu = CPU {
            memory,
            stack: [0x0; 16],
            registers: Registers {
                v0: 0x0,
                v1: 0x0,
                v2: 0x0,
                v3: 0x0,
                v4: 0x0,
                v5: 0x0,
                v6: 0x0,
                v7: 0x0,
                v8: 0x0,
                v9: 0x0,
                va: 0x0,
                vb: 0x0,
                vc: 0x0,
                vd: 0x0,
                ve: 0x0,
                vf: 0x0,
                dt: 0x0,
                i: 0x0,
                pc: PROGRAM_LOAD_OFFSET as u16,
                sp: 0x0,
            },
        };

        cpu
    }

    pub fn fetch_current_instruction(&self) -> u32 {
        (self.memory.memory[self.registers.pc as usize] as u32) << 8
            | (self.memory.memory[self.registers.pc as usize + 1] as u32)
    }

    pub fn get_top_of_stack(&self) -> u16 {
        self.stack[self.registers.sp - 1]
    }

    pub fn write_register(&mut self, register: u32, value: u8) {
        match register {
            0x00 => self.registers.v0 = value,
            0x01 => self.registers.v1 = value,
            0x02 => self.registers.v2 = value,
            0x03 => self.registers.v3 = value,
            0x04 => self.registers.v4 = value,
            0x05 => self.registers.v5 = value,
            0x06 => self.registers.v6 = value,
            0x07 => self.registers.v7 = value,
            0x08 => self.registers.v8 = value,
            0x09 => self.registers.v9 = value,
            0x0a => self.registers.va = value,
            0x0b => self.registers.vb = value,
            0x0c => self.registers.vc = value,
            0x0d => self.registers.vd = value,
            0x0e => self.registers.ve = value,
            0x0f => self.registers.vf = value,
            _ => panic!("Invalid register accessed"),
        }
    }

    pub fn read_register(&self, register: u32) -> u8 {
        match register {
            0x00 => self.registers.v0,
            0x01 => self.registers.v1,
            0x02 => self.registers.v2,
            0x03 => self.registers.v3,
            0x04 => self.registers.v4,
            0x05 => self.registers.v5,
            0x06 => self.registers.v6,
            0x07 => self.registers.v7,
            0x08 => self.registers.v8,
            0x09 => self.registers.v9,
            0x0a => self.registers.va,
            0x0b => self.registers.vb,
            0x0c => self.registers.vc,
            0x0d => self.registers.vd,
            0x0e => self.registers.ve,
            0x0f => self.registers.vf,
            _ => panic!("Invalid register accessed"),
        }
    }

    pub fn step(&mut self) {
        let (instr, value) = Instruction::decode(self.fetch_current_instruction());
        self.registers.pc += 2;
        //println!("{:?} 0x{:X}", instr, value);
        match instr {
            Instruction::SYS => {
                panic!("machine code execution not supported");
            },
            Instruction::CLS => {
                println!("Clear screen");
            },
            Instruction::RET => {
                self.registers.sp -= 1;
                self.registers.pc = self.stack[self.registers.sp];
            },
            Instruction::JP => {
                self.registers.pc = (value & 0x0FFF) as u16;
            },
            Instruction::CALL => {
                self.stack[self.registers.sp] = self.registers.pc;
                self.registers.sp += 1;
                self.registers.pc = (value & 0x0FFF) as u16;
            },
            Instruction::SE_VX_BT => {
                let content = self.read_register(value >> 8 & 0x000F);
                if content == (value & 0x00FF) as u8 {
                    self.registers.pc += 2;
                }
            },
            Instruction::SNE_VX_BT => {
                let content = self.read_register(value >> 8 & 0x000F);
                if content != (value & 0x00FF) as u8 {
                    self.registers.pc += 2;
                }
            },
            Instruction::SE_VX_VY => {
                let content_x = self.read_register(value >> 8 & 0x000F);
                let content_y = self.read_register(value >> 4 & 0x000F);
                if content_x == content_y {
                    self.registers.pc += 2;
                }
            },
            Instruction::LD_VX_BT => {
                self.write_register(value >> 8 & 0x000F, (value & 0x00FF) as u8);
            },
            Instruction::ADD_VX_BT => {
                let register = value >> 8 & 0x000F;
                let to_add = (value & 0x00FF) as u8;
                self.write_register(register, self.read_register(register) as u8 + to_add);
            },
            Instruction::LD_VX_VY => {
                let content_y = self.read_register(value >> 4 & 0x000F);
                self.write_register(value >> 8 & 0x000F, content_y);
            },
            Instruction::OR_VX_VY => {
                let register_x = value >> 8 & 0x000F;
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(value >> 4 & 0x000F);
                self.write_register(register_x, content_x | content_y);
            },
            Instruction::AND_VX_VY => {
                let register_x = value >> 8 & 0x000F;
                let content_x = self.read_register(register_x);
                let content_y = self.read_register(value >> 4 & 0x000F);
                self.write_register(register_x, content_x & content_y);
            },
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
            // JP 0x200
            0x12, //0x20E
            0x00, //0x20F
        ]);
        cpu.registers.vb = 0x22;
        cpu.registers.vd = 0x11;
        cpu.registers.ve = 0x11;
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
        ]);

        cpu.step();
        assert_eq!(0xFF, cpu.registers.va);
        cpu.step();
        assert_eq!(0xAF, cpu.registers.va);
        cpu.step();
        assert_eq!(0xB0, cpu.registers.va);
        assert_ne!(0xB0, cpu.registers.vb);
        cpu.step();
        assert_eq!(0xB0, cpu.registers.vb);
    }

    #[test]
    fn test_math() {
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
        ]);
        cpu.registers.va = 0x02;
        cpu.registers.vb = 0xA1;

        cpu.step();
        assert_eq!(0xA3, cpu.registers.va);
        cpu.step();
        cpu.step();
        assert_eq!(0xA0, cpu.registers.va);
    }

}
