use crate::memory::{Memory, PROGRAM_LOAD_OFFSET};
use crate::instructions::Instruction;

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
    pc: usize, //program counter
    sp: u8, //stack pointer
}

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
                pc: PROGRAM_LOAD_OFFSET as usize,
                sp: 0x0,
            },
        };

        cpu
    }

    pub fn fetch_current_instruction(&self) -> u32 {
        (self.memory.memory[self.registers.pc] as u32) << 8
            | (self.memory.memory[self.registers.pc + 1] as u32)
    }

    pub fn step(&mut self) {
        let (instr, value) = Instruction::decode(self.fetch_current_instruction());
        self.registers.pc += 2;
        println!("{:?} 0x{:X}", instr, value);
        match instr {
            Instruction::SYS => {
                panic!("machine code execution not supported");
            },
            _ => {
                println!("not implemented");
            }
        }
    }
}

