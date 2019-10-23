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
    LD_VX_BT,
    ADD_VX_BT,
    LD_VX_VY,
    OR_VX_VY,
    AND_VX_VY,
    XOR_VX_VY,
    ADD_VX_VY,
    SUB_VX_VY,
    SHR_VX_VY,
    SUBN_VX_VY,
    SHL_VX_VY,
    SNE_VX_VY,
    LD_I_ADDR,
    JP_V0_ADDR,
    RND_VX_BT,
    DRW_VX_VY_NIB,
    SKP_VX,
    SKNP_VX,
    LD_VX_DT,
    LD_DT_VX,
    LD_ST_VX,
    ADD_I_VX,
    LD_F_VX,
    LD_B_VX,
    LD_I_VX,
    LD_VX_I,
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
        let mut cpu = CPU {
            memory
        };

        cpu
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
            (0x6000, 0xF000, 0x0FFF, Instruction::LD_VX_BT),
            (0x7000, 0xF000, 0x0FFF, Instruction::ADD_VX_BT),
            (0x8000, 0xF00F, 0x0FF0, Instruction::LD_VX_VY),
            (0x8001, 0xF00F, 0x0FF0, Instruction::OR_VX_VY),
            (0x8002, 0xF00F, 0x0FF0, Instruction::AND_VX_VY),
            (0x8003, 0xF00F, 0x0FF0, Instruction::XOR_VX_VY),
            (0x8004, 0xF00F, 0x0FF0, Instruction::ADD_VX_VY),
            (0x8005, 0xF00F, 0x0FF0, Instruction::SUB_VX_VY),
            (0x8006, 0xF00F, 0x0FF0, Instruction::SHR_VX_VY),
            (0x8007, 0xF00F, 0x0FF0, Instruction::SUBN_VX_VY),
            (0x800E, 0xF00F, 0x0FF0, Instruction::SHL_VX_VY),
            (0x9000, 0xF00F, 0x0FF0, Instruction::SNE_VX_VY),
            (0xA000, 0xF000, 0x0FFF, Instruction::LD_I_ADDR),
            (0xB000, 0xF000, 0x0FFF, Instruction::JP_V0_ADDR),
            (0xC000, 0xF000, 0x0FFF, Instruction::RND_VX_BT),
            (0xD000, 0xF000, 0x0FFF, Instruction::DRW_VX_VY_NIB),
            (0xE09E, 0xF0FF, 0x0F00, Instruction::SKP_VX),
            (0xE0A1, 0xF0FF, 0x0F00, Instruction::SKNP_VX),
            (0xF007, 0xF0FF, 0x0F00, Instruction::LD_VX_DT),
            (0xF00A, 0xF0FF, 0x0F00, Instruction::LD_VX_BT),
            (0xF015, 0xF0FF, 0x0F00, Instruction::LD_DT_VX),
            (0xF018, 0xF0FF, 0x0F00, Instruction::LD_ST_VX),
            (0xF01E, 0xF0FF, 0x0F00, Instruction::ADD_I_VX),
            (0xF029, 0xF0FF, 0x0F00, Instruction::LD_F_VX),
            (0xF033, 0xF0FF, 0x0F00, Instruction::LD_B_VX),
            (0xF055, 0xF0FF, 0x0F00, Instruction::LD_I_VX),
            (0xF065, 0xF0FF, 0x0F00, Instruction::LD_VX_I),
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
        assert_ne!(Instruction::CALL, Instruction::INVALID);
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
        assert_eq!(Instruction::LD_VX_BT, CPU::decode(0x6Af1).0);
        assert_eq!(0xAF1, CPU::decode(0x6Af1).1);
        assert_eq!(Instruction::ADD_VX_BT, CPU::decode(0x7Af1).0);
        assert_eq!(0xAF1, CPU::decode(0x7AF1).1);
        assert_eq!(Instruction::LD_VX_VY, CPU::decode(0x8AB0).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB0).1);
        assert_eq!(Instruction::OR_VX_VY, CPU::decode(0x8AB1).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB1).1);
        assert_eq!(Instruction::AND_VX_VY, CPU::decode(0x8AB2).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB2).1);
        assert_eq!(Instruction::XOR_VX_VY, CPU::decode(0x8AB3).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB3).1);
        assert_eq!(Instruction::ADD_VX_VY, CPU::decode(0x8AB4).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB4).1);
        assert_eq!(Instruction::SUB_VX_VY, CPU::decode(0x8AB5).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB5).1);
        assert_eq!(Instruction::SHR_VX_VY, CPU::decode(0x8AB6).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB6).1);
        assert_eq!(Instruction::SUBN_VX_VY, CPU::decode(0x8AB7).0);
        assert_eq!(0xAB0, CPU::decode(0x8AB7).1);
        assert_eq!(Instruction::SHL_VX_VY, CPU::decode(0x8ABE).0);
        assert_eq!(0xAB0, CPU::decode(0x8ABE).1);
        assert_eq!(Instruction::SNE_VX_VY, CPU::decode(0x9AB0).0);
        assert_eq!(0xAB0, CPU::decode(0x9AB0).1);
        assert_eq!(Instruction::LD_I_ADDR, CPU::decode(0xA123).0);
        assert_eq!(0x123, CPU::decode(0xA123).1);
        assert_eq!(Instruction::JP_V0_ADDR, CPU::decode(0xB123).0);
        assert_eq!(0x123, CPU::decode(0xB123).1);
        assert_eq!(Instruction::RND_VX_BT, CPU::decode(0xCA23).0);
        assert_eq!(0xA23, CPU::decode(0xCA23).1);
        assert_eq!(Instruction::DRW_VX_VY_NIB, CPU::decode(0xD123).0);
        assert_eq!(0x123, CPU::decode(0xD123).1);
        assert_eq!(Instruction::SKP_VX, CPU::decode(0xE19E).0);
        assert_eq!(0x100, CPU::decode(0xE19E).1);
        assert_eq!(Instruction::SKNP_VX, CPU::decode(0xEAA1).0);
        assert_eq!(0xA00, CPU::decode(0xEAA1).1);
        assert_eq!(Instruction::LD_VX_DT, CPU::decode(0xFA07).0);
        assert_eq!(0xA00, CPU::decode(0xFA07).1);
        assert_eq!(Instruction::LD_VX_BT, CPU::decode(0xFA0A).0);
        assert_eq!(0xA00, CPU::decode(0xFA0A).1);
        assert_eq!(Instruction::LD_DT_VX, CPU::decode(0xFA15).0);
        assert_eq!(0xA00, CPU::decode(0xFA15).1);
        assert_eq!(Instruction::LD_ST_VX, CPU::decode(0xFA18).0);
        assert_eq!(0xA00, CPU::decode(0xFA18).1);
        assert_eq!(Instruction::ADD_I_VX, CPU::decode(0xFA1E).0);
        assert_eq!(0xA00, CPU::decode(0xFA1E).1);
        assert_eq!(Instruction::LD_F_VX, CPU::decode(0xFA29).0);
        assert_eq!(0xA00, CPU::decode(0xFA29).1);
        assert_eq!(Instruction::LD_B_VX, CPU::decode(0xFA33).0);
        assert_eq!(0xA00, CPU::decode(0xFA33).1);
        assert_eq!(Instruction::LD_I_VX, CPU::decode(0xFA55).0);
        assert_eq!(0xA00, CPU::decode(0xFA55).1);
        assert_eq!(Instruction::LD_VX_I, CPU::decode(0xFA65).0);
        assert_eq!(0xA00, CPU::decode(0xFA65).1);
    }
}
