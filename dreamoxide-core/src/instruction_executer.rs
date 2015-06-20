use Cpu;
use Memory;
use Instruction;
use Operand;

/// Handles execution of stored instructions
pub struct InstructionExecuter;

impl InstructionExecuter {
    /// Execute the instruction currently pointed at by PC
    #[inline(always)]
    pub fn execute(cpu: &mut Cpu, mem: &mut Memory, inst: Instruction) {
        //println!("[0x{:8x}] [{:?}] <0x{:8x}> {:?}",
                 //cpu.pc,
                 //cpu.status,
                 //cpu.registers[0].value,
                 //inst);
        match inst {
            Instruction::Add(dest, src) => add(dest, src, cpu),
            Instruction::AddConstant(dest, src) => addi(dest, src, cpu),
            Instruction::AddWithCarry(dest, src) => addc(dest, src, cpu),
            Instruction::AddOverflow(dest, src) => addv(dest, src, cpu),
            Instruction::MulUW(dest, src) => muluw(dest, src, cpu),

            Instruction::And(dest, src) => and(dest, src, cpu),
            Instruction::AndImm(imm) => andi(imm, cpu),
            Instruction::AndB(imm) => andb(imm, cpu),
            Instruction::Or(dest, src) => or(dest, src, cpu),
            Instruction::OrImm(imm) => ori(imm, cpu),
            Instruction::OrB(imm) => orb(imm, cpu),
            Instruction::Xor(dest, src) => xor(dest, src, cpu),
            Instruction::XorImm(imm) => xori(imm, cpu),
            Instruction::XorB(imm) => xorb(imm, cpu),

            Instruction::Tst(dest, src) => tst(dest, src, cpu),
            Instruction::TstImm(imm) => tsti(imm, cpu),
            Instruction::TstB(imm) => tstb(imm, cpu),

            Instruction::Shll(dest) => shll(dest, cpu),
            Instruction::Shll2(dest) => shll2(dest, cpu),
            Instruction::Shll8(dest) => shll8(dest, cpu),
            Instruction::Shll16(dest) => shll16(dest, cpu),
            Instruction::Rotr(dest) => rotr(dest, cpu),

            Instruction::Shlr(dest) => shlr(dest, cpu),
            Instruction::Shlr2(dest) => shlr2(dest, cpu),
            Instruction::Shlr8(dest) => shlr8(dest, cpu),
            Instruction::Shlr16(dest) => shlr16(dest, cpu),

            Instruction::Bf(disp) => bf(disp, cpu),
            Instruction::Bt(disp) => bt(disp, cpu),
            Instruction::Jmp(dest) => jmp(dest, cpu, mem),

            Instruction::SwapB(dest, src) => swapb(dest, src, cpu),
            Instruction::SwapW(dest, src) => swapw(dest, src, cpu),

            Instruction::StsMacL(dest) => stsmacl(dest, cpu),
            Instruction::StsMacH(dest) => stsmach(dest, cpu),

            Instruction::Mov(dest, src) => mov(dest, src, cpu),
            Instruction::MovImm(dest, imm) => movimm(dest, imm, cpu),
            Instruction::MovLDispLoad(dest, imm) => struct_movldispload(dest, imm, cpu, mem),
            _ => ()
        }
    }
}

/// Simply add the dest and src registers
#[inline(always)]
fn add(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu[dest].value += cpu[src].value;
}

/// Simply add the constant to the dest register
#[inline(always)]
fn addi(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_immediate());

    let imm = src.unwrap();

    if imm & 0x80 == 0 {
        cpu[dest].value += 0x000000FF & (imm as u32);
    } else {
        cpu[dest].value += 0xFFFFFF00 | (imm as u32);
    }
}

/// Add with attention to carry flag
#[inline(always)]
fn addc(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    let tmp1 = cpu[dest].value + cpu[src].value;
    let tmp0 = cpu[dest].value;

    cpu[dest].value = tmp1 + if cpu.status.is_carry() { 1 } else { 0 };
    cpu.status.set_carry_cond(tmp0 > tmp1);

    if tmp1 > cpu[dest].value {
        cpu.status.value |= 1;
    }
}

/// Add with overflow
#[inline(always)]
fn addv(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

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

/// Performs a 16 bit unsigned multiplication and stores it
/// in MACL
#[inline(always)]
fn muluw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu.macl.value = (cpu[dest].value & 0x0000FFFF) * (cpu[src].value & 0x0000FFFF);
}

/// Bitwise AND the registers
#[inline(always)]
fn and(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu[dest].value &= cpu[src].value;
}

/// Bitwise AND r0 and the immediate value
#[inline(always)]
fn andi(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value &= 0x000000FF & imm.unwrap() as u32;
}

/// Bitwise AND
#[inline(always)]
fn andb(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());
}

/// Bitwise OR the registers
#[inline(always)]
fn or(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu[dest].value |= cpu[src].value;
}

/// Bitwise OR r0 and the immediate value
#[inline(always)]
fn ori(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value |= 0x000000FF & imm.unwrap() as u32;
}

/// Bitwise OR
#[inline(always)]
fn orb(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());
}

#[inline(always)]
fn xor(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu[dest].value ^= cpu[src].value;
}

#[inline(always)]
fn xori(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());

    cpu[Operand::RegisterOperand(0)].value ^= 0x000000FF & imm.unwrap() as u32;
}

#[inline(always)]
fn xorb(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());
}

#[inline(always)]
fn tst(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    let temp = cpu[dest].value & cpu[src].value;
    cpu.status.set_carry_cond(temp == 0);
}

#[inline(always)]
fn tsti(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());

    let temp = cpu[Operand::RegisterOperand(0)].value & (0x000000FF & imm.unwrap() as u32);
    cpu.status.set_carry_cond(temp == 0);
}

#[inline(always)]
fn tstb(imm: Operand, cpu: &mut Cpu) {
    assert!(imm.is_immediate());

    let temp = cpu[Operand::RegisterOperand(0)].value & (0x000000FF & imm.unwrap() as u32);
    cpu.status.set_carry_cond(temp == 0);
}

#[inline(always)]
fn shll(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    let v = cpu[dest].value;
    cpu.status.set_carry_cond(v & 0x80000000 != 0);
    cpu[dest].value <<= 1;
}

#[inline(always)]
fn shll2(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    cpu[dest].value <<= 2;
}

#[inline(always)]
fn shll8(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    cpu[dest].value <<= 8;
}

#[inline(always)]
fn shll16(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    cpu[dest].value <<= 16;
}

#[inline(always)]
fn shlr(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    let v = cpu[dest].value;
    cpu.status.set_carry_cond(v & 0x00000001 != 0);
    cpu[dest].value >>= 1;
    cpu[dest].value &= 0x7fffffff;
}

#[inline(always)]
fn shlr2(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value >>= 2;
    cpu[dest].value &= 0x3fffffff;
}

#[inline(always)]
fn shlr8(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value >>= 8;
    cpu[dest].value &= 0x00ffffff;
}

#[inline(always)]
fn shlr16(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value >>= 16;
    cpu[dest].value &= 0x0000ffff;
}

#[inline(always)]
fn rotr(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value = cpu[dest].value.rotate_right(1);
}

#[inline(always)]
fn bf(disp: Operand, cpu: &mut Cpu) {
    assert!(disp.is_displacement());

    let d = if disp.unwrap() & 0x80 == 0 {
        0x000000FF & disp.unwrap() as u32
    } else {
        0xFFFFFF00 | disp.unwrap() as u32
    };

    if !cpu.status.is_carry() {
        cpu.pc = cpu.pc + 2 + (d << 1);
    }
}

#[inline(always)]
fn bt(disp: Operand, cpu: &mut Cpu) {
    assert!(disp.is_displacement());

    let d = if disp.unwrap() & 0x80 == 0 {
        0x000000FF & disp.unwrap() as u32
    } else {
        0xFFFFFF00 | disp.unwrap() as u32
    };

    if cpu.status.is_carry() {
        cpu.pc = cpu.pc + 2 + (d << 1);
    }
}

#[inline(always)]
fn jmp(dest: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    assert!(dest.is_register());

    let temp = cpu.pc;
    cpu.pc += 2;
    cpu.step(mem);
    cpu.pc = cpu[dest].value - 2;
}

#[inline(always)]
fn clrt(cpu: &mut Cpu) {
    cpu.status.set_carry_cond(false);
}

#[inline(always)]
fn mov(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    cpu[dest].value = cpu[src].value;
}

#[inline(always)]
fn movimm(dest: Operand, imm: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(imm.is_immediate());

    if imm.unwrap() & 0x80 == 0 {
        cpu[dest].value = 0x000000FF & imm.unwrap() as u32;
    } else {
        cpu[dest].value = 0xFFFFFF00 | imm.unwrap() as u32;
    }
}

#[inline(always)]
fn swapb(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    let temp0 = cpu[src].value & 0xFFFF0000;
    let temp1 = (cpu[src].value & 0x000000FF) << 8;
    cpu[dest].value = (cpu[src].value & 0x0000FF00) >> 8;
    cpu[dest].value = cpu[dest].value | temp0 | temp1;
}

#[inline(always)]
fn swapw(dest: Operand, src: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());
    assert!(src.is_register());

    let temp = (cpu[src].value >> 16) & 0x0000FFFF;
    cpu[dest].value = (cpu[src].value << 16) | temp;
}

#[inline(always)]
fn stsmach(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value = cpu.mach.value;
}

#[inline(always)]
fn stsmacl(dest: Operand, cpu: &mut Cpu) {
    assert!(dest.is_register());

    cpu[dest].value = cpu.macl.value;
}

fn struct_movldispstore(dest: Operand, imm: Operand, cpu: &mut Cpu, mem: &mut Memory) {
    assert!(dest.is_register());
    assert!(imm.is_immediate());

    let src = Operand::RegisterOperand((imm.unwrap() & 0xF0) >> 4);
    let disp = imm.unwrap() as usize & 0xF;

    mem.write_u32(disp * 4 + cpu[dest].value as usize, cpu[src].value);
}

fn struct_movldispload(dest: Operand, imm: Operand, cpu: &mut Cpu, mem: &Memory) {
    assert!(dest.is_register());
    assert!(imm.is_immediate());

    let src = Operand::RegisterOperand((imm.unwrap() & 0xF0) >> 4);
    let disp = imm.unwrap() as usize & 0xF;

    cpu[dest].value = mem.read_u32(disp * 4 + cpu[src].value as usize);
}

#[inline(always)]
fn nop() {
}
