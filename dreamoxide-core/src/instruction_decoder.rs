use Instruction;
use InstructionGroup;
use Operand;

pub struct InstructionDecoder;

impl InstructionDecoder {
    #[inline(always)]
    pub fn decode(code: u16) -> Instruction {
        let c1 = (code & 0xF000) >> 12;
        let c4 = code & 0xF;

        let n = ((code & 0x0F00) >> 8) as u8;
        let m = ((code & 0x00F0) >> 4) as u8;
        let i = (code & 0xF) as u8;

        let op_n = Operand::RegisterOperand(n);
        let op_m = Operand::RegisterOperand(m);
        let imm = Operand::ImmediateOperand(i);
        let disp = Operand::DisplacementOperand(i);

        match c1 {
            0x0 => match c4 {
                0x3 => Instruction::Braf(op_n),
                0x7 => Instruction::MulL(op_n, op_m),
                0x8 => match m {
                    0x0 => Instruction::Clrt,
                    0x4 => Instruction::Clrs,
                    _   => Instruction::Nop
                },
                0x9 => Instruction::Div0u,
                0xA => match m {
                    0x0 => Instruction::StsMacH(op_n),
                    0x1 => Instruction::StsMacL(op_n),
                    0x2 => Instruction::StsPr(op_n),
                    _   => Instruction::Nop
                },
                _   => Instruction::Nop
            },
            0x1 => Instruction::MovLDispStore(op_n, imm),
            0x2 => match c4 {
                0x0 => Instruction::MovB(op_n, op_m),
                0x1 => Instruction::MovW(op_n, op_m),
                0x2 => Instruction::MovL(op_n, op_m),
                0x7 => Instruction::Div0s(op_n, op_m),
                0x8 => Instruction::Tst(op_n, op_m),
                0x9 => Instruction::And(op_n, op_m),
                0xA => Instruction::Xor(op_n, op_m),
                0xB => Instruction::Or(op_n, op_m),
                0xC => Instruction::CmpStr(op_n, op_m),
                0xE => Instruction::MulUW(op_n, op_m),
                0xF => Instruction::MulSW(op_n, op_m),
                _   => Instruction::Nop
            },
            0x3 => match c4 {
                0x0 => Instruction::CmpEq(op_n, op_m),
                0x2 => Instruction::CmpHs(op_n, op_m),
                0x3 => Instruction::CmpGe(op_n, op_m),
                0x4 => Instruction::Div1(op_n, op_m),
                0x6 => Instruction::CmpHi(op_n, op_m),
                0x7 => Instruction::CmpGt(op_n, op_m),
                0xC => Instruction::Add(op_n, op_m),
                0xE => Instruction::AddWithCarry(op_n, op_m),
                0xF => Instruction::AddOverflow(op_n, op_m),
                _   => Instruction::Nop
            },
            0x4 => match c4 {
                0x0 => Instruction::Shll(op_n),
                0x1 => match m {
                    0x0 => Instruction::Shlr(op_n),
                    _ => Instruction::CmpPz(op_n),
                },
                0x4 => match m {
                    0x0 => Instruction::Rotl(op_n),
                    0x2 => Instruction::RotCl(op_n),
                    _   => Instruction::Nop
                },
                0x5 => match m {
                    0x0 => Instruction::Rotr(op_n),
                    0x1 => Instruction::CmpPl(op_n),
                    0x2 => Instruction::RotCr(op_n),
                    _   => Instruction::Nop
                },
                0x8 => match m {
                    0x0 => Instruction::Shll2(op_n),
                    0x1 => Instruction::Shll8(op_n),
                    0x2 => Instruction::Shll16(op_n),
                    _   => Instruction::Nop
                },
                0x9 => match m {
                    0x0 => Instruction::Shlr2(op_n),
                    0x1 => Instruction::Shlr8(op_n),
                    0x2 => Instruction::Shlr16(op_n),
                    _   => Instruction::Nop
                },
                0xB => Instruction::Jmp(op_n),
                _   => Instruction::Nop
            },
            0x5 => Instruction::MovLDispLoad(op_n, imm),
            0x6 => match c4 {
                0x3 => Instruction::Mov(op_n, op_m),
                0x7 => Instruction::Not(op_n, op_m),
                0x8 => Instruction::SwapB(op_n, op_m),
                0x9 => Instruction::SwapW(op_n, op_m),
                _   => Instruction::Nop
            },
            0x7 => Instruction::AddConstant(op_n, imm),
            0x8 => match n {
                0x0 => Instruction::MovBDispStore(op_m, Operand::ImmediateOperand(c4 as u8)),
                0x1 => Instruction::MovWDispStore(op_m, Operand::ImmediateOperand(c4 as u8)),
                0x8 => Instruction::CmpEqImm(imm),
                0x9 => Instruction::Bt(disp),
                0xB => Instruction::Bf(disp),
                _   => Instruction::Nop
            },
            0x9 => Instruction::MovWDisp(op_n, disp),
            0xC => match n {
                0x8 => Instruction::TstImm(imm),
                0x9 => Instruction::AndImm(imm),
                0xA => Instruction::XorImm(imm),
                0xB => Instruction::OrImm(imm),
                0xC => Instruction::TstB(imm),
                0xD => Instruction::AndB(imm),
                0xE => Instruction::XorB(imm),
                0xF => Instruction::OrB(imm),
                _   => Instruction::Nop,
            },
            0xD => Instruction::MovLDisp(op_n, disp),
            0xE => Instruction::MovImm(op_n, imm),
            _ => Instruction::Nop
        }
    }

    pub fn instruction_group(inst: Instruction) -> InstructionGroup {
        match inst {
            Instruction::Add(_, _) => InstructionGroup::EX,
            Instruction::AddConstant(_, _) => InstructionGroup::EX,
            Instruction::AddWithCarry(_, _) => InstructionGroup::EX,
            Instruction::AddOverflow(_, _) => InstructionGroup::EX,
            Instruction::And(_, _) => InstructionGroup::EX,
            _         => InstructionGroup::Unknown
        }
    }

    pub fn alters_pc(inst: Instruction) -> bool {
        match inst {
            Instruction::Bf(_) => true,
            Instruction::Bt(_) => true,
            Instruction::Jmp(_) => true,
            _ => false
        }
    }
}
