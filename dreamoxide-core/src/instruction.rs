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
    MacL(Operand, Operand),
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
    Dt(Operand),
    Xor(Operand, Operand),
    XorImm(Operand),
    XorB(Operand),
    MovData(Operand, Operand),
    MovDataBStore(Operand, Operand),
    MovDataWStore(Operand, Operand),
    MovDataLStore(Operand, Operand),
    MovDataBLoad(Operand, Operand),
    MovDataWLoad(Operand, Operand),
    MovDataSignBLoad(Operand, Operand),
    MovDataSignWLoad(Operand, Operand),
    MovDataSignLLoad(Operand, Operand),
    MovDataBStore1(Operand, Operand),
    MovDataWStore2(Operand, Operand),
    MovDataLStore4(Operand, Operand),
    MovDataSignBLoad1(Operand, Operand),
    MovDataSignWLoad2(Operand, Operand),
    MovDataSignLLoad4(Operand, Operand),
    MovDataStoreR0B(Operand, Operand),
    MovDataStoreR0W(Operand, Operand),
    MovDataStoreR0L(Operand, Operand),
    MovDataLoadR0B(Operand, Operand),
    MovDataLoadR0W(Operand, Operand),
    MovDataLoadR0L(Operand, Operand),
    MovConstantSign(Operand, Operand),
    MovConstantLoadW(Operand, Operand),
    MovConstantLoadL(Operand, Operand),
    MovGlobalLoadB(Operand),
    MovGlobalLoadW(Operand),
    MovGlobalLoadL(Operand),
    MovGlobalStoreB(Operand),
    MovGlobalStoreW(Operand),
    MovGlobalStoreL(Operand),
    MovStructStoreB(Operand, Operand),
    MovStructStoreW(Operand, Operand),
    MovStructStoreL(Operand, Operand),
    MovStructLoadB(Operand, Operand),
    MovStructLoadW(Operand, Operand),
    MovStructLoadL(Operand, Operand),
    MovA(Operand),
    MovCA(Operand),
    MovT(Operand),
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
    Shar(Operand),
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
    FAdd(Operand, Operand),
    Pref(Operand),
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
