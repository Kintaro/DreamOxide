use Cpu;
use Memory;
use Instruction;
use Operand;
use FPSCR_MASK;

use std::mem::transmute;

/// Handles execution of stored instructions
pub struct InstructionExecuter;

impl InstructionExecuter {
    /// Execute the instruction currently pointed at by PC
    //#[inline]
    pub fn execute(cpu: &mut Cpu, mem: &mut Memory, inst: Instruction) {
        //if cpu.pc >= 0x8c0010f0 && cpu.pc <= 0x8c001100 {
        let reg = Operand::RegisterOperand(0);
        //if cpu.pc >= 0x8c00b9F8 && cpu.pc <= 0x8c00b9F8 {
        //if cpu.max >= 0x8c00b978 || 
        //if (cpu.pc >= 0x8c00b864 && cpu.pc <= 0x8c00b86c) {
        //println!("[0x{:8x}] [{:?}] <0x{:8x}> <0x{:8x}> <0x{:8x}> <0x{:8x}> <0x{:8x}> {:?}",
                 //cpu.pc,
                 //cpu.status,
                 //cpu[reg].value,
                 //cpu[Operand::RegisterOperand(1)].value,
                 //cpu[Operand::RegisterOperand(2)].value,
                 //cpu[Operand::RegisterOperand(3)].value,
                 //cpu[Operand::RegisterOperand(4)].value,
                 //inst);
        //}
        match inst {
            Instruction::Add(dest, src) => add(dest, src, cpu),
            Instruction::AddConstant(dest, src) => addi(dest, src, cpu),
            Instruction::AddWithCarry(dest, src) => addc(dest, src, cpu),
            Instruction::AddOverflow(dest, src) => addv(dest, src, cpu),
            Instruction::Sub(dest, src) => sub(dest, src, cpu),
            Instruction::MulSW(dest, src) => mulsw(dest, src, cpu),
            Instruction::MulUW(dest, src) => muluw(dest, src, cpu),
            Instruction::ExtUB(dest, src) => extub(dest, src, cpu),
            Instruction::ExtUW(dest, src) => extuw(dest, src, cpu),
            Instruction::ExtSB(dest, src) => extsb(dest, src, cpu),
            Instruction::ExtSW(dest, src) => extsw(dest, src, cpu),

            Instruction::MacL(dest, src) => macl(dest, src, cpu, mem),

            Instruction::And(dest, src) => and(dest, src, cpu),
            Instruction::AndImm(imm) => andi(imm, cpu),
            Instruction::AndB(imm) => andb(imm, cpu),
            Instruction::Or(dest, src) => or(dest, src, cpu),
            Instruction::OrImm(imm) => ori(imm, cpu),
            Instruction::OrB(imm) => orb(imm, cpu),
            Instruction::Xor(dest, src) => xor(dest, src, cpu),
            Instruction::XorImm(imm) => xori(imm, cpu),
            Instruction::XorB(imm) => xorb(imm, cpu),

            Instruction::CmpEqImm(dest) => cmpeqimm(dest, cpu),
            Instruction::CmpEq(dest, src) => cmpeq(dest, src, cpu),
            Instruction::CmpHs(dest, src) => cmphs(dest, src, cpu),
            Instruction::CmpGe(dest, src) => cmpge(dest, src, cpu),
            Instruction::CmpHi(dest, src) => cmphi(dest, src, cpu),
            Instruction::CmpGt(dest, src) => cmpgt(dest, src, cpu),
            Instruction::CmpPz(src) => cmppz(src, cpu),
            Instruction::CmpPl(src) => cmppl(src, cpu),
            Instruction::CmpStr(dest, src) => cmpstr(dest, src, cpu),
            Instruction::Tst(dest, src) => tst(dest, src, cpu),
            Instruction::TstImm(imm) => tsti(imm, cpu),
            Instruction::TstB(imm) => tstb(imm, cpu),
            Instruction::Tas(dest) => tas(dest, cpu, mem),
            Instruction::Dt(dest) => dt(dest, cpu),

            Instruction::Shll(dest) => shll(dest, cpu),
            Instruction::Shll2(dest) => shll2(dest, cpu),
            Instruction::Shll8(dest) => shll8(dest, cpu),
            Instruction::Shll16(dest) => shll16(dest, cpu),

            Instruction::Shlr(dest) => shlr(dest, cpu),
            Instruction::Shlr2(dest) => shlr2(dest, cpu),
            Instruction::Shlr8(dest) => shlr8(dest, cpu),
            Instruction::Shlr16(dest) => shlr16(dest, cpu),

            Instruction::Rotr(dest) => rotr(dest, cpu),
            Instruction::Shar(dest) => shar(dest, cpu),

            Instruction::Bf(disp) => bf(disp, cpu),
            Instruction::Bt(disp) => bt(disp, cpu),
            Instruction::Bfs(disp) => bfs(disp, cpu, mem),
            Instruction::Bts(disp) => bts(disp, cpu, mem),
            Instruction::Bra(n, disp) => bra(n, disp, cpu, mem),
            Instruction::Braf(dest) => braf(dest, cpu, mem),
            Instruction::Bsr(n, disp) => bsr(n, disp, cpu, mem),
            Instruction::Bsrf(dest) => bsrf(dest, cpu, mem),
            Instruction::Jmp(dest) => jmp(dest, cpu, mem),
            Instruction::Jsr(dest) => jsr(dest, cpu, mem),
            Instruction::Rts => rts(cpu, mem),

            Instruction::SwapB(dest, src) => swapb(dest, src, cpu),
            Instruction::SwapW(dest, src) => swapw(dest, src, cpu),

            Instruction::StcGbr(dest) => stcgbr(dest, cpu),
            Instruction::StcDbr(dest) => stcdbr(dest, cpu),
            Instruction::StcBanked(dest, reg) => stcbanked(dest, reg, cpu),
            Instruction::StsMacL(dest) => stsmacl(dest, cpu),
            Instruction::StsMacH(dest) => stsmach(dest, cpu),
            Instruction::StsLMacH(dest) => stslmach(dest, cpu, mem),
            Instruction::StsLMacL(dest) => stslmacl(dest, cpu, mem),
            Instruction::StsLPr(dest) => stslpr(dest, cpu, mem),

            Instruction::Clrs => clrs(cpu),
            Instruction::Clrt => clrt(cpu),
            Instruction::Sets => sets(cpu),
            Instruction::Sett => sett(cpu),

            Instruction::LdcSr(src) => ldcsr(src, cpu),
            Instruction::LdcDbr(src) => ldcdbr(src, cpu),
            Instruction::LdcLSr(src) => ldclsr(src, cpu, mem),
            Instruction::LdcLGbr(src) => ldclgbr(src, cpu, mem),
            Instruction::LdcLVbr(src) => ldclvbr(src, cpu, mem),
            Instruction::LdcLSsr(src) => ldclssr(src, cpu, mem),
            Instruction::LdcLSpc(src) => ldclspc(src, cpu, mem),

            Instruction::LdsPr(src) => ldspr(src, cpu),
            Instruction::LdsLMacl(src) => ldslmacl(src, cpu, mem),
            Instruction::LdsLMach(src) => ldslmach(src, cpu, mem),
            Instruction::LdsLPr(src) => ldslpr(src, cpu, mem),
            Instruction::LdsFpscr(src) => ldsfpscr(src, cpu),
            Instruction::LdsFpscrL(src) => ldsfpscrl(src, cpu, mem),
            Instruction::LdsFpulL(src) => ldsfpull(src, cpu, mem),

            Instruction::MovData(dest, src) => mov(dest, src, cpu),
            Instruction::MovDataBStore(dest, src) => mov_data_store_b(dest, src, cpu, mem),
            Instruction::MovDataWStore(dest, src) => mov_data_store_w(dest, src, cpu, mem),
            Instruction::MovDataLStore(dest, src) => mov_data_store_l(dest, src, cpu, mem),
            Instruction::MovConstantSign(dest, imm) => mov_const_sign(dest, imm, cpu),
            Instruction::MovConstantLoadW(dest, disp) => mov_const_load_w(dest, disp, cpu, mem),
            Instruction::MovConstantLoadL(dest, disp) => mov_const_load_l(dest, disp, cpu, mem),
            Instruction::MovDataSignBLoad(dest, src) => mov_data_sign_load_b(dest, src, cpu, mem),
            Instruction::MovDataSignWLoad(dest, src) => mov_data_sign_load_w(dest, src, cpu, mem),
            Instruction::MovDataSignBLoad1(dest, src) => mov_data_sign_load_b1(dest, src, cpu, mem),
            Instruction::MovDataSignWLoad2(dest, src) => mov_data_sign_load_w2(dest, src, cpu, mem),
            Instruction::MovDataSignLLoad(dest, src) => mov_data_sign_load_l(dest, src, cpu, mem),
            Instruction::MovDataSignLLoad4(dest, src) => mov_data_sign_load_l4(dest, src, cpu, mem),
            Instruction::MovDataBStore1(dest, src) => mov_data_store_b1(dest, src, cpu, mem),
            Instruction::MovDataWStore2(dest, src) => mov_data_store_w2(dest, src, cpu, mem),
            Instruction::MovDataLStore4(dest, src) => mov_data_store_l4(dest, src, cpu, mem),
            Instruction::MovDataLoadR0W(dest, src) => mov_data_load_r0w(dest, src, cpu, mem),
            Instruction::MovDataLoadR0L(dest, src) => mov_data_load_r0l(dest, src, cpu, mem),
            Instruction::MovDataStoreR0L(dest, src) => mov_data_store_r0b(dest, src, cpu, mem),

            Instruction::MovStructLoadB(src, disp) => mov_struct_load_b(src, disp, cpu, mem),
            Instruction::MovStructLoadW(src, disp) => mov_struct_load_w(src, disp, cpu, mem),
            Instruction::MovStructLoadL(dest, imm) => mov_struct_load_l(dest, imm, cpu, mem),
            Instruction::MovStructStoreW(dest, disp) => mov_struct_store_w(dest, disp, cpu, mem),
            Instruction::MovStructStoreL(dest, imm) => mov_struct_store_l(dest, imm, cpu, mem),

            Instruction::MovGlobalLoadB(disp) => mov_glob_load_b(disp, cpu, mem),
            Instruction::MovGlobalLoadW(disp) => mov_glob_load_w(disp, cpu, mem),
            Instruction::MovGlobalLoadL(disp) => mov_glob_load_l(disp, cpu, mem),
            Instruction::MovGlobalStoreB(disp) => mov_glob_store_b(disp, cpu, mem),
            Instruction::MovGlobalStoreW(disp) => mov_glob_store_w(disp, cpu, mem),
            Instruction::MovGlobalStoreL(disp) => mov_glob_store_l(disp, cpu, mem),

            Instruction::MovA(disp) => mov_a(disp, cpu),

            Instruction::FAdd(dest, src) => fadd(dest, src, cpu),
            Instruction::FMov(dest, src) => fmov(dest, src, cpu),
            Instruction::FMovLoadS4(dest, src) => fmov_load_s4(dest, src, cpu, mem),
            Instruction::FMovLoadD8(dest, src) => fmov_load_d8(dest, src, cpu, mem),
            Instruction::FMovStoreS4(dest, src) => fmov_store_s4(dest, src, cpu, mem),
            Instruction::FMovStoreD8(dest, src) => fmov_store_d8(dest, src, cpu, mem),
            Instruction::Frchg => frchg(cpu),

            Instruction::Pref(_) => (),
            Instruction::Nop => (),

            _ => panic!("Something went wrong! {:?} (at 0x{:08x})", inst, cpu.pc)
        }
    }
}

/// Simply add the dest and src registers
//#[inline]
fn add(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value += cpu[src].value;
}

/// Simply add the constant to the dest register
//#[inline]
fn addi(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_immediate());

    cpu[dest].value += Memory::sign_extend_u8(src.unwrap()) as u32;
}

/// Add with attention to carry flag
//#[inline]
fn addc(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let tmp1 = cpu[dest].value + cpu[src].value;
    let tmp0 = cpu[dest].value;

    cpu[dest].value = tmp1 + if cpu.status.is_carry() { 1 } else { 0 };
    cpu.status.set_carry_cond(tmp0 > tmp1);

    if tmp1 > cpu[dest].value {
        cpu.status.value |= 1;
    }
}

/// Add with overflow
//#[inline]
fn addv(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let d = if cpu[dest].value as i32 >= 0 { 0 } else { 1 };
    let s = if cpu[src].value as i32 >= 0 { 0 } else { 1 } + d;

    cpu[dest].value += cpu[src].value;

    let a = if cpu[dest].value as i32 >= 0 { 0 } else { 1 } + d;

    if s == 0 ||  s == 2 {
        cpu.status.set_carry_cond(a == 1);
    } else {
        cpu.status.set_carry_cond(false);
    }
}

//#[inline]
fn sub(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= cpu[src].value;
}

//#[inline]
fn mulsw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu.macl.value = ((cpu[dest].value as i32) * (cpu[src].value as i32)) as u32;
}

/// Performs a 16 bit unsigned multiplication and stores it
/// in MACL
//#[inline]
fn muluw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu.macl.value = (cpu[dest].value & 0x0000FFFF) * (cpu[src].value & 0x0000FFFF);
}

//#[inline]
fn extub(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = cpu[src].value & 0x000000FF;
}

//#[inline]
fn extuw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = cpu[src].value & 0x0000FFFF;
}

//#[inline]
fn extsb(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u8(cpu[src].value as u8 & 0x000000FF) as u32;
}

//#[inline]
fn extsw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u16(cpu[src].value as u16 & 0x0000FFFF) as u32;
}


//#[inline]
fn macl(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let rm = mem.read_u32(cpu[dest].value as usize) as i32;
    let rn = mem.read_u32(cpu[dest].value as usize) as i32;

    cpu[dest].value += 4;
    cpu[src].value += 4;

    let r = rm as i64 * rn as i64;
    let mach = (cpu.mach.value as u64) << 32;
    let macl = cpu.macl.value as u64;
    let mac = (mach as i64 | macl as i64) + r;

    cpu.mach.value = (mac >> 32) as u32;
    cpu.macl.value = mac as u32;
}

/// Bitwise AND the registers
//#[inline]
fn and(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value &= cpu[src].value;
}

/// Bitwise AND r0 and the immediate value
//#[inline]
fn andi(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value &= 0x000000FF & imm.unwrap() as u32;
}

/// Bitwise AND
//#[inline]
fn andb(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());
}

/// Bitwise OR the registers
//#[inline]
fn or(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value |= cpu[src].value;
}

/// Bitwise OR r0 and the immediate value
//#[inline]
fn ori(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value |= 0x000000FF & imm.unwrap() as u32;
}

/// Bitwise OR
//#[inline]
fn orb(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());
}

//#[inline]
fn xor(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value ^= cpu[src].value;
}

//#[inline]
fn xori(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value ^= 0x000000FF & imm.unwrap() as u32;
}

//#[inline]
fn xorb(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());
}

//#[inline]
fn cmpeqimm(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    let v = Memory::sign_extend_u8(imm.unwrap()) as u32;
    let eq = cpu[Operand::RegisterOperand(0)].value == v;
    cpu.status.set_carry_cond(eq);
}

//#[inline]
fn cmpeq(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = cpu[dest].value == cpu[src].value;
    cpu.status.set_carry_cond(v);
}

//#[inline]
fn cmpge(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = cpu[dest].value as i32 >= cpu[src].value as i32;
    cpu.status.set_carry_cond(v);
}

//#[inline]
fn cmpgt(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = cpu[dest].value as i32 > cpu[src].value as i32;
    cpu.status.set_carry_cond(v);
}

//#[inline]
fn cmphs(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = cpu[dest].value >= cpu[src].value;
    cpu.status.set_carry_cond(v);
}

//#[inline]
fn cmphi(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    let v = cpu[dest].value > cpu[src].value;
    cpu.status.set_carry_cond(v);
}

//#[inline]
fn cmppl(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    let val = cpu[src].value as i32 > 0;
    cpu.status.set_carry_cond(val);
}

//#[inline]
fn cmppz(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    let val = cpu[src].value as i32 >= 0;
    cpu.status.set_carry_cond(val);
}

//#[inline]
fn cmpstr(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let temp = cpu[dest].value ^ cpu[src].value;
    let hh = (temp & 0xFF000000) >> 24;
    let hl = (temp & 0x00FF0000) >> 16;
    let lh = (temp & 0x0000FF00) >>  8;
    let ll =  temp & 0x000000FF;
    let r = hh != 0 && hl != 0 && lh != 0 && ll != 0;

    cpu.status.set_carry_cond(!r);
}

//#[inline]
fn tst(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let temp = cpu[dest].value & cpu[src].value;
    cpu.status.set_carry_cond(temp == 0);
}

//#[inline]
fn tsti(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    let temp = cpu[Operand::RegisterOperand(0)].value & (0x000000FF & imm.unwrap() as u32);
    cpu.status.set_carry_cond(temp == 0);
}

//#[inline]
fn tstb(imm: Operand, cpu: &mut Cpu) {
    debug_assert!(imm.is_immediate());

    let temp = cpu[Operand::RegisterOperand(0)].value & (0x000000FF & imm.unwrap() as u32);
    cpu.status.set_carry_cond(temp == 0);
}

//#[inline]
fn tas(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    let temp = mem.read_u8(cpu[dest].value as usize);
    cpu.status.set_carry_cond(temp == 0);
    mem.write_u8(cpu[dest].value as usize, temp | 0x80);
}

//#[inline]
fn dt(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value -= 1;
    let v = cpu[dest].value;
    cpu.status.set_carry_cond(v == 0);
}

//#[inline]
fn shll(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    let v = cpu[dest].value;
    cpu.status.set_carry_cond(v & 0x80000000 != 0);
    cpu[dest].value <<= 1;
}

//#[inline]
fn shll2(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    cpu[dest].value <<= 2;
}

//#[inline]
fn shll8(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    cpu[dest].value <<= 8;
}

//#[inline]
fn shll16(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    cpu[dest].value <<= 16;
}

//#[inline]
fn shlr(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    let v = cpu[dest].value;
    cpu.status.set_carry_cond(v & 0x00000001 != 0);
    cpu[dest].value >>= 1;
    cpu[dest].value &= 0x7fffffff;
}

//#[inline]
fn shlr2(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value >>= 2;
    cpu[dest].value &= 0x3fffffff;
}

//#[inline]
fn shlr8(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value >>= 8;
    cpu[dest].value &= 0x00ffffff;
}

//#[inline]
fn shlr16(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value >>= 16;
    cpu[dest].value &= 0x0000ffff;
}

//#[inline]
fn rotr(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    let v = cpu[dest].value & 0x1 != 0;
    cpu.status.set_carry_cond(v);
    cpu[dest].value = cpu[dest].value.rotate_right(1);
}

//#[inline]
fn shar(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    let t = cpu[dest].value & 0x1 != 0;
    cpu.status.set_carry_cond(t);

    let temp = if cpu[dest].value & 0x80000000 == 0 { 0 } else { 1 };
    cpu[dest].value >>= 1;

    if temp == 1 {
        cpu[dest].value |= 0x80000000;
    } else {
        cpu[dest].value &= 0x7FFFFFFF;
    }
}

//#[inline]
fn bf(disp: Operand, cpu: &mut Cpu) {
    debug_assert!(disp.is_displacement());

    let d = Memory::sign_extend_u8(disp.unwrap()) as usize;

    if !cpu.status.is_carry() {
        cpu.pc = cpu.pc + 2 + (d << 1);
    }
}

//#[inline]
fn bt(disp: Operand, cpu: &mut Cpu) {
    debug_assert!(disp.is_displacement());

    let d = Memory::sign_extend_u8(disp.unwrap()) as usize;

    if cpu.status.is_carry() {
        cpu.pc = cpu.pc + 2 + (d << 1);
    }
}

//#[inline]
fn bfs(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let d = Memory::sign_extend_u8(disp.unwrap()) as usize;

    let carry = cpu.status.is_carry();
    let oldpc = cpu.pc;
    let temp = cpu.pc + 2 + (d << 1);
    cpu.pc += 2;
    cpu.step(mem);

    if !carry {
        cpu.pc = temp;
    } else {
        cpu.pc = oldpc + 2;
    }
}

//#[inline]
fn bts(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let d = Memory::sign_extend_u8(disp.unwrap()) as usize;

    let carry = cpu.status.is_carry();
    let oldpc = cpu.pc;
    let temp = cpu.pc + 2 + (d << 1);
    cpu.pc += 2;
    cpu.step(mem);

    if carry {
        cpu.pc = temp;
    } else {
        cpu.pc = oldpc + 2;
    }
}

//#[inline]
fn bra(n: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(n.is_register());
    debug_assert!(disp.is_displacement());

    let d = ((n.unwrap() as usize) << 8) | (disp.unwrap() as usize);
    let off = if d & 0x800 == 0 {
        0x00000FFF & d
    } else {
        0xFFFFF000 | d
    } as usize;

    let temp = cpu.pc;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp + 2 + (off * 2);
}

//#[inline]
fn braf(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    let temp = cpu.pc + 2 + cpu[dest].value as usize;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp;
}

//#[inline]
fn bsr(n: Operand, d: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(n.is_register());
    debug_assert!(d.is_displacement());

    let temp = cpu.pc;

    let v = ((n.unwrap() as usize) << 8) | d.unwrap() as usize;
    let disp = if v & 0x800 == 0 {
        v & 0x00000FFF
    } else {
        v | 0xFFFFF000
    };

    cpu.pr = cpu.pc + 4;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp + 2 + (disp << 1);
}

//#[inline]
fn bsrf(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    let temp = cpu.pc + 2 + cpu[dest].value as usize;
    cpu.pr = cpu.pc + 4;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp;
}

//#[inline]
fn jmp(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    let temp = cpu[dest].value as usize;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp - 2;
}

//#[inline]
fn jsr(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    let temp = cpu[dest].value as usize;

    cpu.pr = cpu.pc + 4;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp - 2;
}

//#[inline]
fn rts(cpu: &mut Cpu, mem: &mut Memory) {
    let temp = cpu.pr;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = temp - 2;
}

//#[inline]
fn clrs(cpu: &mut Cpu) {
    cpu.status.set_saturated_cond(false);
}

//#[inline]
fn clrt(cpu: &mut Cpu) {
    cpu.status.set_carry_cond(false);
}

//#[inline]
fn sets(cpu: &mut Cpu) {
    cpu.status.set_saturated_cond(true);
}

//#[inline]
fn sett(cpu: &mut Cpu) {
    cpu.status.set_carry_cond(true);
}

//#[inline]
fn ldcsr(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    cpu.status.value = cpu[src].value & 0x700083f3;
}

//#[inline]
fn ldcdbr(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    cpu.dbr.value = cpu[src].value;
}

//#[inline]
fn ldclsr(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.status.value = v & 0x700083F3;
}

//#[inline]
fn ldclgbr(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.gbr.value = v;
}
//#[inline]
fn ldclvbr(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.vbr.value = v;
}

//#[inline]
fn ldclssr(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.ssr.value = v;
}

//#[inline]
fn ldclspc(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.spc.value = v;
}

//#[inline]
fn ldspr(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    cpu.pr = cpu[src].value as usize;
}

//#[inline]
fn ldslmacl(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.macl.value = v;
}

//#[inline]
fn ldslmach(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.mach.value = v;
}

//#[inline]
fn ldslpr(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize) as usize;
    cpu[src].value += 4;
    cpu.pr = v;
}

//#[inline]
fn ldsfpscr(src: Operand, cpu: &mut Cpu) {
    debug_assert!(src.is_register());

    cpu.fpscr.value = cpu[src].value & FPSCR_MASK;
}

//#[inline]
fn ldsfpscrl(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.fpscr.value = v;
}

//#[inline]
fn ldsfpull(src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[src].value += 4;
    cpu.fpul.value = v;
}

//#[inline]
fn mov(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = cpu[src].value;
}

//#[inline]
fn mov_data_store_b(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    mem.write_u8(cpu[dest].value as usize, cpu[src].value as u8);
}

//#[inline]
fn mov_data_store_w(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    mem.write_u16(cpu[dest].value as usize, cpu[src].value as u16);
}

//#[inline]
fn mov_const_sign(dest: Operand, imm: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(imm.is_immediate());

    cpu[dest].value = Memory::sign_extend_u8(imm.unwrap()) as u32;
}

//#[inline]
fn mov_const_load_w(dest: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(disp.is_displacement());

    let address = cpu.pc + 4 + (disp.unwrap() as usize * 2);
    cpu[dest].value = Memory::sign_extend_u16(mem.read_u16(address)) as u32;
}

//#[inline]
fn mov_const_load_l(dest: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(disp.is_displacement());

    let address = (cpu.pc & 0xFFFFFFFC) as usize + 4 + (disp.unwrap() as usize * 4);
    cpu[dest].value = mem.read_u32(address);
}

//#[inline]
fn mov_data_sign_load_b(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u8(mem.read_u8(cpu[src].value as usize)) as u32;
}

//#[inline]
fn mov_data_sign_load_w(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u16(mem.read_u16(cpu[src].value as usize)) as u32;
}

//#[inline]
fn mov_data_sign_load_b1(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u8(mem.read_u8(cpu[src].value as usize)) as u32;

    if dest != src {
        cpu[src].value += 1;
    }
}


//#[inline]
fn mov_data_sign_load_w2(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value = Memory::sign_extend_u16(mem.read_u16(cpu[src].value as usize)) as u32;

    if dest != src {
        cpu[src].value += 2;
    }
}

//#[inline]
fn mov_data_sign_load_l(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[dest].value = v;
}

//#[inline]
fn mov_data_sign_load_l4(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    cpu[dest].value = v;

    if dest != src {
        cpu[src].value += 4;
    }
}

//#[inline]
fn mov_data_store_l(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    mem.write_u32(cpu[dest].value as usize, cpu[src].value);
}

//#[inline]
fn mov_data_store_b1(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= 1;
    mem.write_u8(cpu[dest].value as usize, cpu[src].value as u8);
}

//#[inline]
fn mov_data_store_w2(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= 2;
    mem.write_u16(cpu[dest].value as usize, cpu[src].value as u16);
}

//#[inline]
fn mov_data_store_l4(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= 4;
    mem.write_u32(cpu[dest].value as usize, cpu[src].value);
}

//#[inline]
fn mov_data_load_r0w(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = mem.read_u16(cpu[src].value as usize + cpu[Operand::RegisterOperand(0)].value as usize);
    cpu[dest].value = Memory::sign_extend_u16(v) as u32;
}

//#[inline]
fn mov_data_load_r0l(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize + cpu[Operand::RegisterOperand(0)].value as usize);
    cpu[dest].value = v;
}


//#[inline]
fn mov_data_store_r0b(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    mem.write_u32(cpu[dest].value as usize + cpu[Operand::RegisterOperand(0)].value as usize, cpu[src].value);
}

//#[inline]
fn mov_struct_load_b(src: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());
    debug_assert!(disp.is_displacement());

    let address = cpu[src].value as usize + (disp.unwrap() & 0xF) as usize * 2;
    let r0 = Operand::RegisterOperand(0);
    cpu[r0].value = Memory::sign_extend_u8(mem.read_u8(address)) as u32;
}


//#[inline]
fn mov_struct_load_w(src: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(src.is_register());
    debug_assert!(disp.is_displacement());

    let address = cpu[src].value as usize + (disp.unwrap() & 0xF) as usize * 2;
    let r0 = Operand::RegisterOperand(0);
    cpu[r0].value = Memory::sign_extend_u16(mem.read_u16(address)) as u32;
}

//#[inline]
fn mov_struct_load_l(dest: Operand, imm: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(imm.is_immediate());

    let m = Operand::RegisterOperand((imm.unwrap() & 0xF0) >> 4);
    let d = imm.unwrap() & 0xF;
    let address = cpu[m].value as usize + d as usize * 4;
    cpu[dest].value = mem.read_u32(address);
}

//#[inline]
fn mov_struct_store_w(dest: Operand, disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(disp.is_displacement());

    let address = cpu[dest].value as usize + disp.unwrap() as usize * 2;
    let r0 = Operand::RegisterOperand(0);
    mem.write_u16(address, cpu[r0].value as u16);
}

//#[inline]
fn mov_struct_store_l(dest: Operand, imm: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(imm.is_immediate());

    let m = Operand::RegisterOperand((imm.unwrap() & 0xF0) >> 4);
    let d = imm.unwrap() & 0xF;
    let address = cpu[dest].value as usize + d as usize * 4;
    mem.write_u32(address, cpu[m].value);
}

//#[inline]
fn mov_glob_load_b(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let v = mem.read_u8(cpu.gbr.value as usize + (disp.unwrap() as usize * 4)) as u32;
    let e = if v & 0x80 == 0 {
        v & 0x000000FF
    } else {
        v | 0xFFFFFF00
    };
    cpu[Operand::RegisterOperand(0)].value = e;
}


//#[inline]
fn mov_glob_load_w(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let v = mem.read_u16(cpu.gbr.value as usize + (disp.unwrap() as usize * 4)) as u32;
    let e = if v & 0x8000 == 0 {
        v & 0x0000FFFF
    } else {
        v | 0xFFFF0000
    };
    cpu[Operand::RegisterOperand(0)].value = e;
}

//#[inline]
fn mov_glob_load_l(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let v = mem.read_u32(cpu.gbr.value as usize + (disp.unwrap() as usize * 4));
    cpu[Operand::RegisterOperand(0)].value = v;
}

//#[inline]
fn mov_glob_store_b(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let address = disp.unwrap() as usize + cpu.gbr.value as usize;
    mem.write_u8(address, cpu[Operand::RegisterOperand(0)].value as u8);
}


//#[inline]
fn mov_glob_store_w(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let address = disp.unwrap() as usize * 2 + cpu.gbr.value as usize;
    mem.write_u16(address, cpu[Operand::RegisterOperand(0)].value as u16);
}


//#[inline]
fn mov_glob_store_l(disp: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(disp.is_displacement());

    let address = disp.unwrap() as usize * 4 + cpu.gbr.value as usize;
    mem.write_u32(address, cpu[Operand::RegisterOperand(0)].value);
}

//#[inline]
fn mov_a(disp: Operand, cpu: &mut Cpu) {
    debug_assert!(disp.is_displacement());

    let address = disp.unwrap() as usize * 4 + (cpu.pc & 0xFFFFFFFC) + 4;
    let r0 = Operand::RegisterOperand(0);

    cpu[r0].value = address as u32;
}

//#[inline]
fn swapb(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let temp0 = cpu[src].value & 0xFFFF0000;
    let temp1 = (cpu[src].value & 0x000000FF) << 8;
    cpu[dest].value = (cpu[src].value & 0x0000FF00) >> 8;
    cpu[dest].value = cpu[dest].value | temp0 | temp1;
}

//#[inline]
fn swapw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let temp = (cpu[src].value >> 16) & 0x0000FFFF;
    cpu[dest].value = (cpu[src].value << 16) | temp;
}

//#[inline]
fn stcgbr(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value = cpu.gbr.value;
}

//#[inline]
fn stcdbr(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value = cpu.dbr.value;
}

//#[inline]
fn stcbanked(dest: Operand, reg: u8, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value = cpu.banked(Operand::RegisterOperand(reg)).value;
}

//#[inline]
fn stsmach(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value = cpu.mach.value;
}

//#[inline]
fn stsmacl(dest: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());

    cpu[dest].value = cpu.macl.value;
}

//#[inline]
fn stslmach(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    cpu[dest].value -= 4;
    mem.write_u32(cpu[dest].value as usize, cpu.mach.value);
}

//#[inline]
fn stslmacl(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    cpu[dest].value -= 4;
    mem.write_u32(cpu[dest].value as usize, cpu.macl.value);
}

//#[inline]
fn stslpr(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());

    cpu[dest].value -= 4;
    mem.write_u32(cpu[dest].value as usize, cpu.pr as u32);
}

//#[inline]
fn fadd(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu.fpu_mut(dest).value += cpu.fpu(src).value;
}

//#[inline]
fn fmov(dest: Operand, src: Operand, cpu: &mut Cpu) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu.fpu_mut(dest).value = cpu.fpu(src).value;
}

//#[inline]
fn fmov_load_s4(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let v = mem.read_u32(cpu[src].value as usize);
    unsafe {
        cpu.fpu_mut(dest).value = transmute(v);
    }
    cpu[src].value += 4;
}

//#[inline]
fn fmov_load_d8(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    let h = mem.read_u32(cpu[src].value as usize);
    let l = mem.read_u32(cpu[src].value as usize + 4);
    let dest1 = Operand::RegisterOperand(dest.unwrap() + 1);
    unsafe {
        cpu.fpu_mut(dest1).value = transmute(l);
        cpu.fpu_mut(dest).value = transmute(h);
    }
    cpu[src].value += 8;
}

//#[inline]
fn fmov_store_s4(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= 4;
    unsafe {
        let v = transmute::<f32, u32>(cpu.fpu(src).value);

        mem.write_u32(cpu[dest].value as usize, v);
    }
}

//#[inline]
fn fmov_store_d8(dest: Operand, src: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    debug_assert!(dest.is_register());
    debug_assert!(src.is_register());

    cpu[dest].value -= 8;
    let src1 = Operand::RegisterOperand(src.unwrap() + 1);
    unsafe {
        let h = transmute::<f32, u32>(cpu.fpu(src).value);
        let l = transmute::<f32, u32>(cpu.fpu(src1).value);

        mem.write_u32(cpu[dest].value as usize, h);
        mem.write_u32(cpu[dest].value as usize + 4, l);
    }
}

//#[inline]
fn frchg(cpu: &mut Cpu) {
    cpu.fpscr.value ^= 0x00200000;
}
