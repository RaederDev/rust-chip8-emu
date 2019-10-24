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

impl Instruction {
    pub fn decode(instruction: u32) -> (Instruction, u32) {
        let instructions = [
            (0x00E0, 0xFFFF, Instruction::CLS),
            (0x00EE, 0xFFFF, Instruction::RET),
            (0x1000, 0xF000, Instruction::JP),
            (0x2000, 0xF000, Instruction::CALL),
            (0x3000, 0xF000, Instruction::SE_VX_BT),
            (0x4000, 0xF000, Instruction::SNE_VX_BT),
            (0x5000, 0xF00F, Instruction::SE_VX_VY),
            (0x6000, 0xF000, Instruction::LD_VX_BT),
            (0x7000, 0xF000, Instruction::ADD_VX_BT),
            (0x8000, 0xF00F, Instruction::LD_VX_VY),
            (0x8001, 0xF00F, Instruction::OR_VX_VY),
            (0x8002, 0xF00F, Instruction::AND_VX_VY),
            (0x8003, 0xF00F, Instruction::XOR_VX_VY),
            (0x8004, 0xF00F, Instruction::ADD_VX_VY),
            (0x8005, 0xF00F, Instruction::SUB_VX_VY),
            (0x8006, 0xF00F, Instruction::SHR_VX_VY),
            (0x8007, 0xF00F, Instruction::SUBN_VX_VY),
            (0x800E, 0xF00F, Instruction::SHL_VX_VY),
            (0x9000, 0xF00F, Instruction::SNE_VX_VY),
            (0xA000, 0xF000, Instruction::LD_I_ADDR),
            (0xB000, 0xF000, Instruction::JP_V0_ADDR),
            (0xC000, 0xF000, Instruction::RND_VX_BT),
            (0xD000, 0xF000, Instruction::DRW_VX_VY_NIB),
            (0xE09E, 0xF0FF, Instruction::SKP_VX),
            (0xE0A1, 0xF0FF, Instruction::SKNP_VX),
            (0xF007, 0xF0FF, Instruction::LD_VX_DT),
            (0xF00A, 0xF0FF, Instruction::LD_VX_BT),
            (0xF015, 0xF0FF, Instruction::LD_DT_VX),
            (0xF018, 0xF0FF, Instruction::LD_ST_VX),
            (0xF01E, 0xF0FF, Instruction::ADD_I_VX),
            (0xF029, 0xF0FF, Instruction::LD_F_VX),
            (0xF033, 0xF0FF, Instruction::LD_B_VX),
            (0xF055, 0xF0FF, Instruction::LD_I_VX),
            (0xF065, 0xF0FF, Instruction::LD_VX_I),
        ];

        for (opcode, inverse_mask, instruction_type) in instructions.iter() {
            if instruction & *inverse_mask == *opcode {
                return (*instruction_type, instruction);
            }
        }

        (Instruction::INVALID, 0x0)
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;

    #[test]
    fn test_decoder() {
        assert_eq!(0x124E, Instruction::decode(0x124E).1);
        assert_eq!(Instruction::INVALID, Instruction::INVALID);
        assert_ne!(Instruction::CALL, Instruction::INVALID);
        assert_eq!(Instruction::CLS, Instruction::decode(0x00E0).0);
        assert_eq!(Instruction::RET, Instruction::decode(0x00EE).0);
        assert_eq!(Instruction::JP, Instruction::decode(0x124E).0);
        assert_eq!(Instruction::CALL, Instruction::decode(0x224E).0);
        assert_eq!(Instruction::SE_VX_BT, Instruction::decode(0x3Af0).0);
        assert_eq!(Instruction::SNE_VX_BT, Instruction::decode(0x4Af0).0);
        assert_eq!(Instruction::SE_VX_VY, Instruction::decode(0x5Af0).0);
        assert_ne!(Instruction::SE_VX_VY, Instruction::decode(0x5Af1).0);
        assert_eq!(Instruction::LD_VX_BT, Instruction::decode(0x6Af1).0);
        assert_eq!(Instruction::ADD_VX_BT, Instruction::decode(0x7Af1).0);
        assert_eq!(Instruction::LD_VX_VY, Instruction::decode(0x8AB0).0);
        assert_eq!(Instruction::OR_VX_VY, Instruction::decode(0x8AB1).0);
        assert_eq!(Instruction::AND_VX_VY, Instruction::decode(0x8AB2).0);
        assert_eq!(Instruction::XOR_VX_VY, Instruction::decode(0x8AB3).0);
        assert_eq!(Instruction::ADD_VX_VY, Instruction::decode(0x8AB4).0);
        assert_eq!(Instruction::SUB_VX_VY, Instruction::decode(0x8AB5).0);
        assert_eq!(Instruction::SHR_VX_VY, Instruction::decode(0x8AB6).0);
        assert_eq!(Instruction::SUBN_VX_VY, Instruction::decode(0x8AB7).0);
        assert_eq!(Instruction::SHL_VX_VY, Instruction::decode(0x8ABE).0);
        assert_eq!(Instruction::SNE_VX_VY, Instruction::decode(0x9AB0).0);
        assert_eq!(Instruction::LD_I_ADDR, Instruction::decode(0xA123).0);
        assert_eq!(Instruction::JP_V0_ADDR, Instruction::decode(0xB123).0);
        assert_eq!(Instruction::RND_VX_BT, Instruction::decode(0xCA23).0);
        assert_eq!(Instruction::DRW_VX_VY_NIB, Instruction::decode(0xD123).0);
        assert_eq!(Instruction::SKP_VX, Instruction::decode(0xE19E).0);
        assert_eq!(Instruction::SKNP_VX, Instruction::decode(0xEAA1).0);
        assert_eq!(Instruction::LD_VX_DT, Instruction::decode(0xFA07).0);
        assert_eq!(Instruction::LD_VX_BT, Instruction::decode(0xFA0A).0);
        assert_eq!(Instruction::LD_DT_VX, Instruction::decode(0xFA15).0);
        assert_eq!(Instruction::LD_ST_VX, Instruction::decode(0xFA18).0);
        assert_eq!(Instruction::ADD_I_VX, Instruction::decode(0xFA1E).0);
        assert_eq!(Instruction::LD_F_VX, Instruction::decode(0xFA29).0);
        assert_eq!(Instruction::LD_B_VX, Instruction::decode(0xFA33).0);
        assert_eq!(Instruction::LD_I_VX, Instruction::decode(0xFA55).0);
        assert_eq!(Instruction::LD_VX_I, Instruction::decode(0xFA65).0);
    }
}
