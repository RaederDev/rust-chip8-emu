use crate::memory::Memory;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE_VX_BT,
    SNE_VX_BT,
    SE_VX_VY,
    LD_X_BT,
    INVALID,
}

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
    //carry
    i: u8,
    //index
    pc: u8, //program counter
}

pub struct CPU {
    pub memory: Memory,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            memory
        }
    }

//    pub fn exec(&self, instr: Instructions) {
//        match instr {
//            Instructions::SYS(_, _) => {
//                panic!("machine code execution not supported");
//            },
//            _ => {
//                println!("not implemented");
//            }
//        }
//    }

    pub fn decode(instruction: i32) -> (Instruction, i32) {
        let instructions = [
            (0x00E0, 0xFFFF, 0x0000, Instruction::CLS),
            (0x00EE, 0xFFFF, 0x0000, Instruction::RET),
            (0x1000, 0xF000, 0x0FFF, Instruction::JP),
            (0x2000, 0xF000, 0x0FFF, Instruction::CALL),
            (0x3000, 0xF000, 0x0FFF, Instruction::SE_VX_BT),
            (0x4000, 0xF000, 0x0FFF, Instruction::SNE_VX_BT),
            (0x5000, 0xF00F, 0x0FF0, Instruction::SE_VX_VY),
            (0x6000, 0xF000, 0x0FFF, Instruction::LD_X_BT),
        ];

        for (opcode, inverse_mask, mask, instruction_type) in instructions.iter() {
            if instruction & *inverse_mask == *opcode {
                return (*instruction_type, instruction & *mask);
            }
        }

        (Instruction::INVALID, 0x0)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{Instruction, CPU};

    #[test]
    fn test_decoder() {
        assert_eq!(Instruction::INVALID, Instruction::INVALID);
        assert_eq!(Instruction::CLS, CPU::decode(0x00E0).0);
        assert_eq!(Instruction::RET, CPU::decode(0x00EE).0);
        assert_eq!(Instruction::JP, CPU::decode(0x124E).0);
        assert_eq!(0x024E, CPU::decode(0x124E).1);
        assert_eq!(Instruction::CALL, CPU::decode(0x224E).0);
        assert_eq!(0x024E, CPU::decode(0x224E).1);
        assert_eq!(Instruction::SE_VX_BT, CPU::decode(0x3Af0).0);
        assert_eq!(0xAF0, CPU::decode(0x3AF0).1);
        assert_eq!(Instruction::SNE_VX_BT, CPU::decode(0x4Af0).0);
        assert_eq!(0xAF0, CPU::decode(0x4AF0).1);
        assert_eq!(Instruction::SE_VX_VY, CPU::decode(0x5Af0).0);
        assert_ne!(Instruction::SE_VX_VY, CPU::decode(0x5Af1).0);
        assert_eq!(0xAF0, CPU::decode(0x5AF0).1);
        assert_eq!(Instruction::LD_X_BT, CPU::decode(0x6Af1).0);
        assert_eq!(0xAF1, CPU::decode(0x6Af1).1);
    }
}
