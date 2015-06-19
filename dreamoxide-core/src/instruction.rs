use Operand;

use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Error;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Add(Operand, Operand),
    AddConstant(Operand, Operand),
    AddWithCarry(Operand, Operand),
    AddOverflow(Operand, Operand),
    CmpEqImm(Operand),
    CmpEq(Operand, Operand),
    CmpHs(Operand, Operand),
    CmpGe(Operand, Operand),
    CmpHi(Operand, Operand),
    CmpGt(Operand, Operand),
    CmpPz(Operand),
    CmpPl(Operand),
    CmpStr(Operand, Operand),
    Div1(Operand, Operand),
    Div0s(Operand, Operand),
    Div0u,
    MulL(Operand, Operand),
    MulSW(Operand, Operand),
    MulUW(Operand, Operand),
    And(Operand, Operand),
    AndImm(Operand),
    AndB(Operand),
    Not(Operand, Operand),
    Or(Operand, Operand),
    OrImm(Operand),
    OrB(Operand),
    Tst(Operand, Operand),
    TstImm(Operand),
    TstB(Operand),
    Xor(Operand, Operand),
    XorImm(Operand),
    XorB(Operand),
    MovImm(Operand, Operand),
    MovWDisp(Operand, Operand),
    MovLDisp(Operand, Operand),
    Mov(Operand, Operand),
    MovB(Operand, Operand),
    MovW(Operand, Operand),
    MovL(Operand, Operand),
    MovBDispStore(Operand, Operand),
    MovWDispStore(Operand, Operand),
    MovLDispLoad(Operand, Operand), // 3
    MovLDispStore(Operand, Operand), // 3
    SwapB(Operand, Operand),
    SwapW(Operand, Operand),
    Rotl(Operand),
    Rotr(Operand),
    RotCl(Operand),
    RotCr(Operand),
    Shll(Operand),
    Shll2(Operand),
    Shll8(Operand),
    Shll16(Operand),
    Shlr(Operand),
    Shlr2(Operand),
    Shlr8(Operand),
    Shlr16(Operand),
    Bf(Operand),
    Bt(Operand),
    Bra(Operand, Operand),
    Braf(Operand),
    Jmp(Operand),
    Clrs,
    Clrt,
    LdcSr(Operand),
    LdcGbr(Operand),
    LdcVbr(Operand),
    Sets,
    Sett,
    StsMacH(Operand),
    StsMacL(Operand),
    StsPr(Operand),
    Nop
}

pub enum InstructionGroup {
    EX,
    BR,
    Unknown
}

//impl Debug for Instruction {
    //fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        //match *self {
            //Instruction::Add(n, m) => fmt.write_str((format!("add {:?}, {:?}", n, m)).as_str()),
            //Instruction::MovImm(n, imm) => fmt.write_str((format!("mov {:?}, {:?}", n, imm)).as_str()),
            //_ => fmt.write_str("not implemented")
        //}
    //}
//}
